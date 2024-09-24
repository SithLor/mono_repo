#sudo apt update
#sudo apt upgrade
#sudo apt-get install qemu ovmf
#sudo apt-get install qemu-system
#qemu-system-x86_64

#https://rust-osdev.github.io/uefi-rs/HEAD/



#build efi image
cd efi_app
cargo build --target x86_64-unknown-uefi
cd ..

#make the dir for efi image
mkdir -p ./efi_test/esp/efi/boot
#copy the efi image to the dir
cp "./efi_app/target/x86_64-unknown-uefi/debug/uefi_app.efi" "./efi_test/esp/efi/boot/bootx64.efi"

cp /usr/share/OVMF/OVMF_CODE.fd ./efi_test/OVMF_CODE.fd
cp /usr/share/OVMF/OVMF_VARS.fd ./efi_test/OVMF_VARS.fd

cd efi_test
#-enable-kvm \
qemu-system-x86_64 -nographic -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp 
