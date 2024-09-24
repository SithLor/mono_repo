#!/bin/bash

# Check if the correct number of arguments are provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 path_to_binary_file (c|rust) output_file"
    exit 1
fi

binaryfile="$1"
lang="$2"
outputfile="$3"

# Use xxd to create a hex dump and then format it as a byte array
shellcode=$(xxd -p < "$binaryfile" | tr -d '\n')

# Calculate the length of the byte array
len=$(echo -n "$shellcode" | wc -c)
len=$((len / 2))

# Format the output based on the language
if [ "$lang" = "c" ]; then
    # Format as a C byte array
    shellcode=$(echo "$shellcode" | sed 's/../0x&, /g')
    shellcode="${shellcode::-2};"
elif [ "$lang" = "rust" ]; then
    # Format as a Rust byte array
    shellcode=$(echo "$shellcode" | sed 's/../0x&, /g')
    shellcode="let SHELL_CODE: [u8; $len] = [${shellcode::-2}];"
else
    echo "Unsupported language: $lang"
    exit 1
fi

# Write the result to the output file
echo "$shellcode" > "$outputfile"