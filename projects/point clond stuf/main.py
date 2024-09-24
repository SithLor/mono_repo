##https://imagetostl.com/convert/file/obj/to/xyz
file_in = "./points.txt"
file_out = "./points_desmos.txt"

X = []
Y = []
Z = []

# remove dups
remove_dups = False
# specify the number of decimal places to which you want to round
decimal_places = 2
# specify the line at which you want to stop
stop_line = 7612

with open(file_in, "r") as f:
    lines = f.readlines()
    for i, line in enumerate(lines):
        if i == stop_line:
            break
        x, y, z = map(float, line.split())
        X.append(round(x, decimal_places))
        Y.append(round(y, decimal_places))
        Z.append(round(z, decimal_places))

if remove_dups == True:
    X = [x for i, x in enumerate(X) if X.index(x) == i]
    Y = [y for i, y in enumerate(Y) if Y.index(y) == i]
    Z = [z for i, z in enumerate(Z) if Z.index(z) == i]


with open(file_out+"_x", "w") as f:
    #out like x=[val,val,val] y=[val,val,val] z=[val,val,val]
    ##f.write(f"{X}\n{Y}\n{Z}")
    f.write(f"{X}")
with open(file_out+"_y", "w") as f:
    #out like x=[val,val,val] y=[val,val,val] z=[val,val,val]
    f.write(f"{Y}")
with open(file_out+"_z", "w") as f:
    #out like x=[val,val,val] y=[val,val,val] z=[val,val,val]
    f.write(f"{Z}")

print("Done")