# visualizing the samples
import matplotlib.pyplot as plt

f = open("../bit_star/samples.txt", "r")
lines = f.readlines()
x, y = [], []
for line in lines:
    #print("printing lines")
    #print(line.split(','))
    line_split = line.split(',')
    x.append( float(line_split[1]))
    y.append( float(line_split[2]))
plt.scatter(x,y)
plt.show()
    
