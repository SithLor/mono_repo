
# using input.txt
code_slow: 3705ms
code_fast: 179ms
code_rayon: 193ms
code_rayon_optimized(): 154ms


# using input.m.txt

code_slow:              26311ms
code_fast:              1029ms
code_rayon:             1376ms
code_rayon_optimized(): 1248ms

java impl 1: 1413ms 


# Commands 

```sh
cd src
java -ea ./main.java
cd ..
cargo build --release
cargo run --release
```

