//https://github.com/gcardone/www.gcardone.net/blob/master/_posts/2021-03-04-blistering-fast-mandelbrot-in-rust.markdown
use image;
use std::format;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use rayon::prelude::*;
// Import SIMD instructions
use std::arch::x86_64::*;
const MAX_ITERATIONS: u32 = 5000; // or any other value you want
// Function to map the number of iterations i to a grey value between 0 (black)
// and 255 (white).
fn _get_color(i: u32, max_iterations: u32) -> image::Rgb<u8> {
    if i > max_iterations {
        return image::Rgb([255, 255, 255]);
    }
    if max_iterations == 255 {
        let idx = i as u8;
        return image::Rgb([idx, idx, idx]);
    }
    let idx = (((i as f32) / (max_iterations as f32)) * 255.0).round() as u8;
    return image::Rgb([idx, idx, idx]);
}
fn get_color(i: u32, max_iterations: u32) -> image::Rgb<u8> {
    if i > max_iterations {
        return image::Rgb([0, 0, 0]); // You can change this to any color for values beyond max_iterations
    }

    let r = ((i as f32) / (max_iterations as f32) * 6.0).clamp(0.0, 6.0);
    let fract = r - r.floor();
    let (mut p, mut q, mut t) = (0.0, 1.0 - fract, fract);
    
    if r >= 6.0 { p = 0.0; q = 0.0; t = 0.0; }
    else if r >= 5.0 { p = 1.0; q = 0.0; }
    else if r >= 4.0 { p = 1.0; t = 0.0; }
    else if r >= 3.0 { p = 0.0; t = 0.0; }
    else if r >= 2.0 { q = 0.0; }
    else if r >= 1.0 { p = 0.0; }

    let red = ((p * 255.0) as u8).min(255);
    let green = ((q * 255.0) as u8).min(255);
    let blue = ((t * 255.0) as u8).min(255);

    image::Rgb([red, green, blue])
}

// Function to run a Mandelbrot rendering algorithm and measure its execution
// time.
// Arguments:
//   name: name of the algorithm, it's used to print its name and save the output.
//   w: width of the output image, in pixels.
//   h: height of the output image, in pixels.
//   save_image: if true, save the output of the algorithm to
//               /tmp/mandelbrot_{name}.png
//   algo: actual rendering algorithm that should take as inputs the width and
//         height of the output image and returns an image::RgbImage
use std::env;
fn runalgo(name: &str, w: u32, h: u32, save_image: bool, algo: fn(u32, u32) -> image::RgbImage) {
    print!("Executing {}... ", name);
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = algo(w, h);
    let elapsed = now.elapsed().as_millis() as f32 / 1000.0;

    let path: String = format!("./src/m_{}.png", name);
    img.save(path).unwrap();
    println!("{}s", elapsed);
}

// Render the Mandelbrot set. w and h are respectively the widht and height of
// the output image. Refer to
// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set for
// a detailed explanation of this algorithm.
fn naive(w: u32, h: u32) -> image::RgbImage {
    let mut img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::RgbImage::new(w, h);
    for c in 0..w {
        let x0: f32 = ((c as f32) / (w as f32)) * 3.5 - 2.5;
        for r in 0..h {
            let y0 = ((r as f32) / (h as f32)) * 2.0 - 1.0;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration: u32 = 0;
            while x * x + y * y <= 4.0 && iteration < MAX_ITERATIONS {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration = iteration + 1;
            }
            let rgb: image::Rgb<u8> = get_color(iteration, MAX_ITERATIONS);
            img.put_pixel(c, r, rgb);
        }
    }
    img
}
// Import rayon




struct Pixel {
    x: u32,
    y: u32,
    iterations: u32,
}


fn avxrayon(w: u32, h: u32) -> image::RgbImage {
    let mut img = image::RgbImage::new(w as u32, h as u32);
    // Conditional build only for x86_64, see
    // https://doc.rust-lang.org/reference/conditional-compilation.html
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                // The closure process_column processes a column of the mandelbrot image and
                // returns a vector of Pixels. Each pixel contains its coordinates and the
                // number of iterations. The algorithm below is an exact implementation of the
                // naive algorithm using AVX2 instructions. `c` is the column to process,
                // that is the x coordinate.
                let process_column = |c: u32| -> Vec<Pixel> {
                    let mut v = vec![];
                    let x0 = ((c as f32) / (w as f32)) * 3.5 - 2.5;
                    // Initialize ax0 as 8 packed single precision float initialized to x0.
                    let ax0 = _mm256_set1_ps(x0);
                    // r is the row to process, that is the y coordinate. We step by 8 because
                    // AVX2 packs 8 floats in a single register.
                    for r in (0..h).step_by(8) {
                        // Initialize ar with r, repeated 8 times...
                        let ar: __m256 = _mm256_set1_ps(r as f32);
                        // ... and then add 7, 6, ... 0 to the ar coordinates. This means that
                        // the floats packed in ay0 contain the coordinates of contiguous pixels
                        // along the y axis.
                        let mut ay0 =
                            _mm256_add_ps(_mm256_set_ps(7., 6., 5., 4., 3., 2., 1., 0.), ar);
                        // ay0 = (r / h) * 2 - 1
                        ay0 = _mm256_sub_ps(
                            _mm256_mul_ps(
                                _mm256_div_ps(ay0, _mm256_set1_ps(h as f32)),
                                _mm256_set1_ps(2.0),
                            ),
                            _mm256_set1_ps(1.0),
                        );
                        // ax = 0
                        let mut ax = _mm256_set1_ps(0.0);
                        // ay = 0
                        let mut ay = _mm256_set1_ps(0.0);
                        // aiters contains the number of iterations for each pixel, initialized
                        // to 0.
                        let mut aiters = _mm256_set1_epi32(0);
                        // If a packed integer in amask is set to 1, then the iterator in aiters
                        // in the same position will be incremented. This allows us to repeat the
                        // core escape loop only if at least one of the pixels needs more iterations.
                        // If amask is all set to zero then we can bail out.
                        let mut amask = _mm256_set1_epi32(1);
                        for _ in 0..MAX_ITERATIONS {
                            // axtemp = x * x - y * y + x0
                            let axtemp = _mm256_add_ps(
                                _mm256_sub_ps(_mm256_mul_ps(ax, ax), _mm256_mul_ps(ay, ay)),
                                ax0,
                            );
                            // y = 2.0 * x * y + y0
                            // The "2.0 * x" multiplication has been replaced with a more efficient
                            // x + x.
                            ay = _mm256_add_ps(
                                _mm256_mul_ps(_mm256_add_ps(ax, ax), ay),
                                ay0,
                            );
                            ax = axtemp;
                            // Increase all the iterations if the matching mask is set to 1.
                            aiters = _mm256_add_epi32(aiters, amask);
                            // threshold = x * x + y * y
                            let athreshold =
                                _mm256_add_ps(_mm256_mul_ps(ax, ax), _mm256_mul_ps(ay, ay));
                            // Compare the values in athreshold with 4.0, and store 0xFFFFFFFF in
                            // acond if the condition is true, 0 otherwise.
                            let acond = _mm256_cmp_ps(athreshold, _mm256_set1_ps(4.0), _CMP_LE_OQ);
                            // Do a logical and between amask and the acond. This means that each
                            // packed integer in amask will be set to zero if x * x + y * y > 4.0.
                            amask = _mm256_and_si256(amask, _mm256_castps_si256(acond));
                            // If amask contains only bits set to zero, then we don't need to keep
                            // iterating.
                            let breakthreshold: i32 =
                                _mm256_testz_si256(amask, _mm256_set1_epi32(-1));
                            if breakthreshold == 1 {
                                break;
                            }
                        }
                        // Unpack the iteration values in a rust old-fashioned array
                        let mut iters_unpacked = [0i32; 8];
                        _mm256_maskstore_epi32(&mut iters_unpacked[0], _mm256_set1_epi32(-1), aiters);
                        // Store the result of the computation in a vector
                        for (count, ir) in iters_unpacked.iter().enumerate() {
                            v.push(Pixel{x: c, y: r + count as u32, iterations: *ir as u32});
                        }
                    }
                    v
                };
                // Use rayon to parallelize the execution of the code above across all available
                // cores and collect the results.
                let vecs: Vec<Vec<Pixel>> =
                    (0..w).into_par_iter().map(|c| process_column(c)).collect();
                for column_result in vecs.iter() {
                    for item in column_result.iter() {
                        img.put_pixel(item.x, item.y, get_color(item.iterations, MAX_ITERATIONS))
                    }
                }
            }
        }
    }
    img
}


fn main() {
    let width = 4024;
    let height = 4024;
    let save_image = false;
    runalgo("avxrayon", width, height, save_image, avxrayon);
    runalgo("naive", width, height, save_image, naive);
}