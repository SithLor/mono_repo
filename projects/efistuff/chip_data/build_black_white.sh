rm ./chip_black_and_white.rs
# Remove the existing Rust file
rm ./chip_black_and_white/chip_black_and_white.rs
# Run the Python script to generate a new Rust file
python3 chip_blackwhite.py
# Copy the generated Rust file to the desired location
cp ./chip_black_and_white.rs ./chip_black_and_white/chip_black_and_white.rs
clear
echo "Black and white chip data has been generated."
echo "You can now run the following command to build the project:"
echo "cd chip_black_and_white"
echo "cargo build --release"