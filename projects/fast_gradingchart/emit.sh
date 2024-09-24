cd ./debug
rustc ../src/main.rs --emit llvm-bc
rustc ../src/main.rs --emit mir
rustc ../src/main.rs --emit asm 
rustc ../src/main.rs --emit llvm-ir
rustc ../src/main.rs --emit obj
rustc ../src/main.rs --emit metadata 
rustc ../src/main.rs --emit link 
rustc ../src/main.rs --emit dep-info