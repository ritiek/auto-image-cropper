#!/usr/bin/env python

from PIL import Image
from sys import argv

if len(argv) == 3:
	path = argv[1]
	output = argv[2]
else:
	print("usage: " + argv[0] + " <input_image> <output_image>")
	exit()

def getTopLeft(path):
	im = Image.open(path)
	for x in range(im.size[1]):
		for y in range(im.size[0]):
			color = im.getpixel((y, x))
			if not color[0] == 255 and not color[1] == 255 and not color[2] == 255:
				return x # top

def getTopRight(path):
	im = Image.open(path)
	for x in range(im.size[0]):
		for y in range(im.size[1]):
			color = im.getpixel((x, y))
			if not color[0] == 255 and not color[1] == 255 and not color[2] == 255:
				return x # left

def getLowerLeft(path):
	im = Image.open(path)
	for x in range(im.size[1]-1, 0, -1):
		for y in range(im.size[0]-1, 0, -1):
			color = im.getpixel((y, x))
			if not color[0] == 255 and not color[1] == 255 and not color[2] == 255:
				return (x + 1) # down

def getLowerRight(path):
	im = Image.open(path)
	for x in range(im.size[0]-1, 0, -1):
		for y in range(im.size[1]-1, 0, -1):
			color = im.getpixel((x, y))
			if not color[0] == 255 and not color[1] == 255 and not color[2] == 255:
				return (x + 1) # right

b, a = getTopLeft(path), getTopRight(path)
y, x = getLowerLeft(path), getLowerRight(path)
strings = "(" + str(a) + ", " + str(b) + ", " + str(x) + ", " + str(y) + ")"
print("Cropping area " + strings + " from " + path + " to " + output)

im = Image.open(path)
cropped = im.crop((a, b, x, y))
cropped.save(output)
