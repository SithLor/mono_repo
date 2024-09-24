fn run_file_as_byte_array(file_path: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Unable to read file");
    buffer
}
// 

use std::arch::x86_64::*;

fn code_count(data: Vec<u8>) {
    let mut total_count: u64 = 0;
    let mut i = 0;
    let len = data.len() / 16 * 16;

    unsafe {
        while i < len {
            let chunk = _mm_loadu_si128(data[i..].as_ptr() as *const __m128i);
            for j in 0..16 {
                let val = match j {
                    0 => _mm_extract_epi8(chunk, 0),
                    1 => _mm_extract_epi8(chunk, 1),
                    2 => _mm_extract_epi8(chunk, 2),
                    3 => _mm_extract_epi8(chunk, 3),
                    4 => _mm_extract_epi8(chunk, 4),
                    5 => _mm_extract_epi8(chunk, 5),
                    6 => _mm_extract_epi8(chunk, 6),
                    7 => _mm_extract_epi8(chunk, 7),
                    8 => _mm_extract_epi8(chunk, 8),
                    9 => _mm_extract_epi8(chunk, 9),
                    10 => _mm_extract_epi8(chunk, 10),
                    11 => _mm_extract_epi8(chunk, 11),
                    12 => _mm_extract_epi8(chunk, 12),
                    13 => _mm_extract_epi8(chunk, 13),
                    14 => _mm_extract_epi8(chunk, 14),
                    15 => _mm_extract_epi8(chunk, 15),
                    _ => unreachable!(),
                };
                if val >= 48 && val <= 57 {
                    total_count += (val - 48) as u64;
                }
            }
            i += 16;
        }
    }

    // process remaining elements
    while i < data.len() {
        if data[i] >= 48 && data[i] <= 57 {
            total_count += (data[i] - 48) as u64;
        }
        i += 1;
    }

    println!("Count: {}", total_count);
}



fn main() {
    //settings
    let path: &str = "./src/pi-billion.txt";

    let start: std::time::Instant = std::time::Instant::now();
    let data: Vec<u8> = run_file_as_byte_array(path);
    println!("File Read: {}ms", start.elapsed().as_millis());

    let _start = std::time::Instant::now();
    code_avg_all_values(data);
    println!("Code Count: {}ms", _start.elapsed().as_millis());

    println!("Hello, world!");
}
