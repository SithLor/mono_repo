# Remove the existing Rust file
rm -rf ./chip_color.rs
rm -rf ./chip_color/chip_color.rs
# Run the Python script to generate a new Rust file
python3 ./chip_color.py

# Copy the generated Rust file to the desired location
cp ./chip_color.rs ./chip_color/src/chip_color.rs

##rm -rf ./chip_color/chip_color.rs
echo "color chip data has been generated."
echo "You can now run the following command to build the project:"
echo "cd chip_color"
echo "cargo build --release"