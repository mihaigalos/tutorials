from matplotlib import animation
import mpl_toolkits.mplot3d.axes3d as p3
import numpy as np
from matplotlib.animation import FuncAnimation
from matplotlib.ticker import LinearLocator, FormatStrFormatter
from matplotlib import cm
import matplotlib.pyplot as plt
from numpy import genfromtxt
from mpl_toolkits.mplot3d import Axes3D
import matplotlib


fig = plt.figure()
ax = plt.axes(projection ="3d")

X = np.arange(-10, 10, 0.25)
Y = np.arange(-10, 10, 0.25)
X, Y = np.meshgrid(X, Y)

Z = 0.1*X**3-0.3*X*(Y**2)+200.0

surf = ax.plot_surface(X, Y, Z, cmap=cm.coolwarm,
                       linewidth=0, antialiased=False)
surf._facecolors2d = surf._facecolor3d
surf._edgecolors2d = surf._edgecolor3d
ax.zaxis.set_major_locator(LinearLocator(10))
ax.zaxis.set_major_formatter(FormatStrFormatter('%.02f'))


def update(num, data, line):
    for i in range(len(data)):
        line[i][0].set_data(data[i][:2, :num])
        line[i][0].set_3d_properties(data[i][2, :num])


def plot(csv_files, colors, labels):
    data = [[] for i in range(len(csv_files))]
    line = [[] for i in range(len(csv_files))]

    i = 0
    for csv_file in csv_files:

        my_data = genfromtxt(csv_file, delimiter=',')

        x = my_data[:, 0]
        y = my_data[:, 1]
        z = my_data[:, 2]

        N = 1000
        data[i] = np.array([x, y, z])
        line[i] = ax.plot(x, y, z, label=labels[i])

        i = i + 1

    ani = animation.FuncAnimation(fig, update, N, fargs=(
        data, line), interval=1000/N, blit=False)
    ax.legend(labels, loc='upper right')

    plt.show()


plot(['output/gd.csv', 'output/rmsprop.csv', 'output/rmsprop_momentum.csv'], [
    'ro', 'go', '-b'], ['GD', 'RMSProp', 'RMSProp_Momentum'])
