
fn findprime(_min:u64, _max:u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    for i in _min.._max {
        let mut isprime: bool = true;
        for j in 2..i {
            if i % j == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            primes.push(i);
        }
    }
    primes
}

fn writefile(path:String,_primes: Vec<u64>) {
    use std::fs::File;
    use std::io::Write;
    let mut file: File = File::create(path).expect("Unable to create file");
    for p in _primes {
        file.write_all(p.to_string().as_bytes()).expect("Unable to write data");
        file.write_all(b"\n").expect("Unable to write data");
    }
}
fn main() {
    use std::time::Instant;
    use std::time::Duration;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: primefinder <min> <max> <output file>");
        std::process::exit(1);
    }
    let min: u64 = args[1].parse().expect("Unable to parse min");
    let max: u64 = args[2].parse().expect("Unable to parse max");
    //get thetime before the prime search
    let start: Instant = Instant::now();
    let primes: Vec<u64> = findprime(min, max);
    let elapsed: Duration = start.elapsed();
    println!("Time to find primes: {:?}", elapsed);
    writefile(args[3].to_string(), primes);

}
