const INPUT_FILE: &str = "src/list.txt";
fn code(data:Vec<u8>){
    //add all of the each u8 together
    let mut sum:u64 = 0;
    for i in data{
        sum += i as u64;
    }
    //println!("Sum: {}", sum);
}

fn main() {
    //create file buffer and read file spilt file
//    let file = std::fs::read("./src/list.txt").unwrap();
//    let mut data:Vec<u8> = Vec::new();
//    for i in file{
//        data.push(i);
//    }
//    let start: std::time::Instant = std::time::Instant::now();
//    for i in file{
//        data.push(i);
//    }
//    code(data);
//    println!("Time: {}ms", start.elapsed().as_millis());

    //call code function
    bench();
}
fn bench(){
    //run main 1000 times get average time
    let mut TIMES:Vec<u128> = Vec::new();
    let file: Vec<u8> = std::fs::read(INPUT_FILE).unwrap();
    for _ in 0..1000{
        let mut data:Vec<u8> = Vec::new();
        let start: std::time::Instant = std::time::Instant::now();
        for i in &file{
            data.push(*i);
        }
        code(data);
        let end: u128 = start.elapsed().as_nanos();
        TIMES.push(end);
    }
    //get average time
    let mut sum:u128 = 0;
    for i in &TIMES{
        sum += *i;
    }
    let avg: u128 = sum / TIMES.len() as u128;
    //convert to ms
    let avg_ms: f64 = avg as f64 / 1_000_000.0;
    println!("Average Time: {}ns", avg_ms);
}
