import re

def convert_to_rust(input_file, output_file):
    with open(input_file, 'r') as f_in, open(output_file, 'w') as f_out:
        for line in f_in:
            line = line.strip()
            if line.startswith('#define'):
                parts = line.split(None, 2)  # Split on whitespace, max 2 parts
                if len(parts) == 3:
                    f_out.write(f"const {parts[1]}: usize = {parts[2]};\n")
            elif line.startswith('#ifdef'):
                parts = line.split()
                if len(parts) == 2:
                    f_out.write(f"#[cfg(feature = \"{parts[1].lower()}\")]\n")
            elif line.startswith('#endif'):
                pass  # Ignore #endif in Rust
            else:
                f_out.write(line + '\n')  # Copy other lines as is

convert_to_rust('asm.h', 'asm.rs')