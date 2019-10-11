'''
======================
3D surface (color map)
======================

Demonstrates plotting a 3D surface colored with the coolwarm color map.
The surface is made opaque by using antialiased=False.

Also demonstrates using the LinearLocator and custom formatting for the
z axis tick labels.
'''
from numpy import genfromtxt
from mpl_toolkits.mplot3d import Axes3D
import matplotlib.pyplot as plt
from matplotlib import cm
from matplotlib.ticker import LinearLocator, FormatStrFormatter
from matplotlib.animation import FuncAnimation
import numpy as np

import mpl_toolkits.mplot3d.axes3d as p3
from matplotlib import animation


# ---------------------------------------------------
# Actual shape
# ---------------------------------------------------

fig = plt.figure()
ax = fig.gca(projection='3d')

# Make data.
X = np.arange(-10, 10, 0.25)
Y = np.arange(-10, 10, 0.25)
X, Y = np.meshgrid(X, Y)
# R = np.sqrt(X**2 + Y**2)
Z = 0.1*X**3-0.3*X*(Y**2)

# Plot the surface.
surf = ax.plot_surface(X, Y, Z, cmap=cm.coolwarm,
                       linewidth=0, antialiased=False)

# Customize the z axis.
# ax.set_zlim(-1.01, 1.01)
ax.zaxis.set_major_locator(LinearLocator(10))
ax.zaxis.set_major_formatter(FormatStrFormatter('%.02f'))


my_data = genfromtxt('out.csv', delimiter=',')

x = my_data[:, 0]
y = my_data[:, 1]
z = my_data[:, 2]

# ax.plot3D(x, y, z, 'gray')
# plt.show()

# ---------------------------------------------------
# Animated descent
# ---------------------------------------------------


def update(num, data, line, point):
    # if num < len(data[2, :num]):

    line.set_data(data[:2, :num])
    line.set_3d_properties(data[2, :num])
    if num < len(data[0]):
        point.set_data([data[0][num], data[1][num]])
        point.set_3d_properties(data[2][num])


N = 100
data = np.array([x, y, z])
line, = ax.plot(x, y, z)
point, = ax.plot([0], [0], [0], 'ro')

ani = animation.FuncAnimation(fig, update, N, fargs=(
    data, line, point), interval=10000/N, blit=False)
# ani.save('matplot003.gif', writer='imagemagick')


# draw a point


plt.show()
