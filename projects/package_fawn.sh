function build(){
    rm -rf ./BINS/fawn_app/fawn_app.exe
    #build the libary for fawn-lib
    cd fawn-lib
    cargo build --release -j 64
    cd ..

    APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/release/fawn-app.exe"
    cd fawn-app
    cargo build --target x86_64-pc-windows-gnu --release -j 64
    cd ..
    cp $APP_RELEASE ./BINS/fawn_app/fawn_app.exe
}
function test_linux(){
    rm -rf ./BINS/fawn_app/fawn_app.exe
    cd fawn-lib
    cargo build --release -j 64
    cd ..

    APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/debug/fawn-app.exe"
    cd fawn-app
    cargo build --target x86_64-pc-windows-gnu -j 64
    cd ..
    echo " "
    echo " "
    echo " "
    echo " "
    echo " "
    echo " "
    echo " "
    
    wine $APP_RELEASE
}

# check 
## ask $1 if it is empty and check for test build
if [ -z "$1" ]
then
    echo "No argument supplied"
    build
elif [ "$1" == "test" ]
then
    test_linux
else
    echo "Invalid argument"
    clear
fi
