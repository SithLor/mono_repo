cargo rustc --release -- -Z tune-cpu=native -C opt-level=3 -C overflow-checks=false -C strip=debuginfo -C target-cpu=native -C lto=thin -C debuginfo=0 -C panic=abort
./target/release/fast_seatingchart > bench.txt

# append to bench.txt

