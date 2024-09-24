function calculateEntropy(password) {
    const charsetSize = 94; // printable ASCII characters
    const passwordLength = password.length;
    const entropy = Math.log2(Math.pow(charsetSize, passwordLength));
    return entropy;
}
function main(argc, argv) {
    console.log("ARGC:" + argc);
    console.log("ARGV:" + argv);
    if (argv[2]) {
        const entropy = calculateEntropy(argv[2]);
        console.log("Password entropy: " + entropy);
    } else {
        console.log("Please provide a password as an argument.");
    }
}

main(process.argv.length, process.argv);