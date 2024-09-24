#!/bin/bash


# hashes the all file and folder in cwd except for folder name target and file name sig.sh
# using openssl dgst 
# using algo blake2b512,blake2s256,md4,md5,mdc2,rmd160,sha1,sha224,sha256,sha3-224,sha3-256,sha3-384,sha3-512,sha384,sha512,sha512-224,sha512-256,shake128,shake256,sm3, 
# output is in the format of HASH_TYPE FILE_NAME HASH

# check if openssl is installed


# List of algorithms
algos=("md5"  "sha1" "sha224" "sha256" "sha3-224" "sha3-256" "sha3-384" "sha3-512" "sha384" "sha512" "sha512-224" "sha512-256" )

# Loop through all files in the current directory and its subdirectories
# Ignore any folder called target or .git, and file called sig.sh
for file in $(find . -type f -not -path "**/target/*" -not -path "./.git/*" -not -name "sig.sh")
do
    # Loop through all algorithms
    for algo in ${algos[@]}
    do
        # Calculate the hash of the file
        hash=$(openssl dgst -$algo $file | awk '{print $2}')
        # Print the hash in the format HASH_TYPE FILE_NAME HASH
        echo $algo $file $hash  >> sig.txt
    done
done

