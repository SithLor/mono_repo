use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

mod chip_black_and_white;

fn main() {
    let start = Instant::now();
    let fps = 10;
    let frame_time = Duration::from_secs(1) / fps;

    // Print all frames in a loop for 10 seconds
    while start.elapsed() < Duration::from_secs(10) {
        for i in 1..=8 {
            let frame_start = Instant::now();

            // Clear the console
            let _ = Command::new("clear").status();

            match i {
                1 => println!("{}", chip_black_and_white::ASCII_ART_1),
                2 => println!("{}", chip_black_and_white::ASCII_ART_2),
                3 => println!("{}", chip_black_and_white::ASCII_ART_3),
                4 => println!("{}", chip_black_and_white::ASCII_ART_4),
                5 => println!("{}", chip_black_and_white::ASCII_ART_5),
                6 => println!("{}", chip_black_and_white::ASCII_ART_6),
                7 => println!("{}", chip_black_and_white::ASCII_ART_7),
                8 => println!("{}", chip_black_and_white::ASCII_ART_8),
                _ => (),
            }

            // Sleep for the remaining frame time
            if frame_start.elapsed() < frame_time {
                thread::sleep(frame_time - frame_start.elapsed());
            }
        }
    }
    println!("Hello, world!");
}