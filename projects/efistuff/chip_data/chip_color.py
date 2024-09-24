
# config zone
CHAR_SET = '@%#*+=-:. ' # defaut:'@%#*+=-:. ' extra:qazwsxedcrfvtgbyhnujmikolpQAZWSXEDCRFVTGBYHNUJMIKOLP,|\~`<>?.;/[]\{\}!@#$%^&*()@%#*+=-:. 
RANDOM_ODER = True  # defaut: False whether to use a random order of the characters
WIDTH = 50  # defuat:80 width of the ASCII art 
HEIGHT = 50  # defuat:80 height of the ASCII art
#config zone

def random_order(char_set):
    import random
    char_list = list(char_set)
    random.shuffle(char_list)
    return ''.join(char_list)
if RANDOM_ODER:
    CHAR_SET = random_order(CHAR_SET)
else:
    CHAR_SET = CHAR_SET

import os
import numpy as np
from PIL import Image
from sklearn.cluster import KMeans
from colored import fg, attr
def reduce_colors(rgb_array, n_colors=256):
    reshaped_array = rgb_array.reshape(-1, 3)
    unique_colors = np.unique(reshaped_array, axis=0)
    n_colors = min(n_colors, unique_colors.shape[0])
    kmeans = KMeans(n_clusters=n_colors)
    labels = kmeans.fit_predict(reshaped_array)
    palette = kmeans.cluster_centers_
    reduced_color_img = palette[labels].reshape(rgb_array.shape).astype(int)
    return reduced_color_img

#def image_to_rgb_array(image_path):
#    img = Image.open(image_path)
#    img = img.convert('RGB')
#    return np.array(img)
##def image_to_rgb_array(image_path, scale_factor=0.2):
##    img = Image.open(image_path)
##    # Scale down the image
##    width, height = img.size
##    new_size = (int(width * scale_factor), int(height * scale_factor))
##    img = img.resize(new_size)
##    img = img.convert('RGB')
##    return np.array(img)
##def rgb_to_ascii(rgb_array):
##    ascii_chars = '@%#*+=-:. '  # define the ASCII characters to use
##    ascii_img = ''
##    for row in rgb_array:
##        for pixel in row:
##            r, g, b = pixel
##            brightness = (r + g + b) / 3 / 255  # calculate the brightness
##            ascii_index = brightness * (len(ascii_chars) - 1)  # scale brightness to the index of ascii_chars
##            ascii_img += f"\033[38;2;{r};{g};{b}m{ascii_chars[int(ascii_index)]}\033[0m"  # print with color
##        ascii_img += '\n'
##    return ascii_img

###def image_to_rgb_array(image_path, max_size=(80, 80)):
###    img = Image.open(image_path)
###    img.thumbnail(max_size)  # scale it down proportionally
###    img = img.convert('RGB')
###    return np.array(img)

def image_to_rgb_array(image_path, max_size=(80, 80)):
    img = Image.open(image_path)
    img.thumbnail(max_size)  # scale it down
    img = img.convert('RGB')  # convert image to RGB

    WIDTH, HEIGHT = img.size
    rgb_array = []

    for y in range(0, HEIGHT, 2):  # we skip a line each time to correct the aspect ratio
        row = []
        for x in range(WIDTH):
            pixel = img.getpixel((x, y))  # get the RGB values of the pixel
            row.append(pixel)
        rgb_array.append(row)

    return np.array(rgb_array)



def rgb_to_8bit(r, g, b):
    """Convert RGB color to 8-bit color"""
    return 16 + (36 * (r // 51)) + (6 * (g // 51)) + (b // 51)

def rgb_to_ascii(rgb_array):
    ascii_chars = CHAR_SET  # define the ASCII characters to use
    ascii_img = ''
    for row in rgb_array:
        for pixel in row:
            r, g, b = pixel
            brightness = (r + g + b) / 3 / 255  # calculate the brightness
            ascii_index = brightness * (len(ascii_chars) - 1)  # scale brightness to the index of ascii_chars
            color_code = rgb_to_8bit(r, g, b)
            ascii_img += f"{fg(color_code)}{ascii_chars[int(ascii_index)]}{attr(0)}"  # print with color
        ascii_img += '\n'
    return ascii_img

def write_to_file(i, ascii):
    with open('chip_color.rs', 'a') as f:
        f.write(f"pub const ASCII_ART_{i}: &str = r##\"{ascii}\"##;\n")




def main():
    images_path = "./frames/"
    max_size = (WIDTH, HEIGHT)
    for i in range(1, 9):
        rgb_array = image_to_rgb_array(images_path + "000" + str(i) + ".png", max_size=max_size)
        reduced_color_array = reduce_colors(rgb_array)
        ascii = rgb_to_ascii(reduced_color_array)
        write_to_file(i, ascii)

main()