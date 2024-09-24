
# 100k

code_fast(): 29ms
code_faster(): 17ms
code_rayon(): 27ms
code_faster_2(): 5ms

# 1 millons

code_fast(): 3214ms
code_faster(): 1924ms
code_rayon(): 3650ms
code_faster_2(): 534ms



# Commands 

```sh
cargo rustc --release -- -Z tune-cpu=machine -C opt-level=3 -C overflow-checks=no -C strip=debuginfo -C target-cpu=native -C debuginfo=0 -C lto
./target/release/fast_seatingchart
```