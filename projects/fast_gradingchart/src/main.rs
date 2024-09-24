//use rustup 2024 ; cargo run 
const OUTPUT_FILE: &str = "./src/output.txt";
const INPUT_FILE: &str = "./src/input.txt";



use std::{fs::File, io::{BufRead, BufWriter, Write}};
use rayon::prelude::*; // This line brings the rayon prelude into scope
use std::collections::HashMap;
use std::cell::RefCell;

thread_local! {
    static MEMO: RefCell<HashMap<usize, (usize, usize)>> = RefCell::new(HashMap::new());
}
fn min_rows_cols(amount: usize) -> (usize, usize) {
    let sqrt: f64 = (amount as f64).sqrt();
    let rows: usize = sqrt.ceil() as usize;
    let cols: usize = if rows * (rows - 1) >= amount {
        rows - 1
    } else {
        rows
    };
    (rows, cols)
}
#[inline]
fn min_rows_cols_fast(amount: usize) -> (usize, usize) {
    MEMO.with(|memo| {
        if let Some(&result) = memo.borrow().get(&amount) {
            return result;
        }
        let sqrt: f64 = (amount as f64).sqrt();
        let rows: usize = sqrt.ceil() as usize;
        let cols: usize = if rows * (rows - 1) >= amount {
            rows - 1
        } else {
            rows
        };
        let result = (rows, cols);
        memo.borrow_mut().insert(amount, result);
        result
    })
}

fn code_slow(){
        //4044 
        //file input
        let input: String = std::fs::read_to_string(INPUT_FILE).expect("Unable to read file");
        let mut output: std::fs::File = std::fs::File::create(OUTPUT_FILE).expect("Unable to create file");
        let mut grades: Vec<i8> = Vec::new();
        let lines: std::str::Lines<'_> = input.lines();
        for line in lines {
            //parse grade split by space
            let grade: i8 = line.split_whitespace().collect::<Vec<&str>>()[1]
                .parse()
                .expect("Unable to parse grade");
            grades.push(grade);
        }
        //get the avg of the grades
        let avg: f64 = grades.iter().map(|&x| x as i32).sum::<i32>() as f64 / grades.len() as f64;
        //println!("Average: {}", avg);
        let (rows, cols) = min_rows_cols(grades.len());
        for i in 0..rows {
            for j in 0..cols {
                if let Some(grade) = grades.get(i * cols + j) {
                    write!(output, "{} ", grade).expect("Unable to write to file");
                }
            }
            write!(output, "\n").unwrap();
        }
    
}
fn code_rayon(){
    use rayon::prelude::*;

    let input: std::fs::File = File::open(INPUT_FILE).expect("Unable to read file");
    let input: std::io::BufReader<File> = std::io::BufReader::new(input);

    let output: File = File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<File> = BufWriter::new(output);

    let mut grades: Vec<i8> = Vec::new();

    for line in input.lines() {
        let line: String = line.expect("Unable to read line");
        let grade: i8 = line.split_whitespace().nth(1)
            .and_then(|s| s.parse().ok())
            .expect("Unable to parse grade");
        grades.push(grade);
    }
    let sum: i32 = grades.par_iter().map(|&x| x as i32).sum();
    let avg: f64 = sum as f64 / grades.len() as f64;
    //println!("Average: {}", avg);

    let (rows, cols) = min_rows_cols_fast(grades.len());
    for i in 0..rows {
        for j in 0..cols {
            if let Some(grade) = grades.get(i * cols + j) {
                write!(output, "{} ", grade).expect("Unable to write to file");
            }
        }
        writeln!(output).unwrap();
    }
}
fn code_fast() {
    let input: std::fs::File = std::fs::File::open(INPUT_FILE).expect("Unable to read file");
    let input: std::io::BufReader<std::fs::File> = std::io::BufReader::new(input);

    let output: std::fs::File = std::fs::File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<std::fs::File> = BufWriter::new(output);

    let mut grades: Vec<i8> = Vec::new();

    for line in input.lines() {
        let line: String = line.expect("Unable to read line");
        let grade: i8 = line.split_whitespace().nth(1)
            .and_then(|s| s.parse().ok())
            .expect("Unable to parse grade");
        grades.push(grade);
    }

    let avg: f64 = grades.iter().map(|&x| x as i32).sum::<i32>() as f64 / grades.len() as f64;
    //println!("Average: {}", avg);

    let (rows, cols) = min_rows_cols_fast(grades.len());
    for i in 0..rows {
        for j in 0..cols {
            if let Some(grade) = grades.get(i * cols + j) {
                write!(output, "{} ", grade).expect("Unable to write to file");
            }
        }
        writeln!(output).unwrap();
    }
}





fn code_rayon_optimized() {
    let input: File = File::open(INPUT_FILE).expect("Unable to read file");
    let input: std::io::BufReader<File> = std::io::BufReader::new(input);

    let output: File = File::create(OUTPUT_FILE).expect("Unable to create file");
    let mut output: BufWriter<File> = BufWriter::new(output);

    let grades: Vec<i8> = input.lines()
        .filter_map(|line: Result<String, std::io::Error>| line.ok())
        .filter_map(|line: String| line.split_whitespace().nth(1).map(|s| s.parse().ok()).flatten())
        .collect();

    let sum: i32 = grades.par_iter().map(|&x| x as i32).sum();
    let avg: f64 = sum as f64 / grades.len() as f64;
    //println!("Average: {}", avg);

    let (rows, cols) = min_rows_cols_fast(grades.len());
    for i in 0..rows {
        for j in 0..cols {
            if let Some(grade) = grades.get(i * cols + j) {
                write!(output, "{} ", grade).expect("Unable to write to file");
            }
        }
        writeln!(output).unwrap();
    }
}

fn code_sort(){
}

fn main() {
    for i in 0..4 {
        if i == 0 {
            let start: std::time::Instant = std::time::Instant::now();
            code_slow();
            println!("code_slow: {}ms", start.elapsed().as_millis());
        } else if i == 1 {
            let start: std::time::Instant = std::time::Instant::now();
            code_fast();
            println!("code_fast: {}ms", start.elapsed().as_millis());
        } else if i == 2 {
            let start: std::time::Instant = std::time::Instant::now();
            code_rayon();
            println!("code_rayon: {}ms", start.elapsed().as_millis());
        } else if i == 3{
            let start: std::time::Instant = std::time::Instant::now();
            code_rayon_optimized();
            println!("code_rayon_optimized(): {}ms", start.elapsed().as_millis());
        }
    }   
    //let start: std::time::Instant = std::time::Instant::now();
    //code_rayon_optimized();
    //println!("Time: {}ms", start.elapsed().as_millis());
}

