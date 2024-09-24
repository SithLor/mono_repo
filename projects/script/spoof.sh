# Create the usb_drive directory
mkdir -p "./usb_drive"
cd "./usb_drive"

# Create a 512KB image file and format it as ext4
# min drive size 256 for ext4
fallocate -l 256KB data.img
mkfs -t ext4 ./data.img

# Mount the image file to a new directory
mkdir -p "./data"
sudo mount -t auto -o loop ./data.img ./data

echo "Hello, World!" | sudo tee ./data/hello.txt > /dev/null

sudo umount ./data
sudo rm -rf ./data

cd ..
# Convert the image file to a C array and write it to data.h
xxd -i ./usb_drive/data.img > ./data.h
