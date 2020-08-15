#!/usr/bin/env python

from PIL import Image
import argparse

def get_arguments():
	parser = argparse.ArgumentParser()

	parser.add_argument(
		"-i",
		"--input",
		help="Path of Input Image"
	)
	parser.add_argument(
		"-o",
		"--output",
		help="Path to Output Image"
	)

	args = parser.parse_args()
	if not args.input or not args.output:
		parser.error("Please specify the path for image")

	return args


class AutoCrop:
	def __init__(self, img):
		self.img = img

	def get_top_left(self):
		for x in range(self.img.size[1]):
			for y in range(self.img.size[0]):
				color = self.img.getpixel((y, x))
				if color != (255, 255, 255):
					return x # top

	def get_top_right(self):
		for x in range(self.img.size[0]):
			for y in range(self.img.size[1]):
				color = self.img.getpixel((x, y))
				if color != (255, 255, 255):
					return x # left

	def get_lower_left(self):
		for x in range(self.img.size[1]-1, 0, -1):
			for y in range(self.img.size[0]-1, 0, -1):
				color = self.img.getpixel((y, x))
				if color != (255, 255, 255):
					return (x + 1) # down

	def get_lower_right(self):
		for x in range(self.img.size[0]-1, 0, -1):
			for y in range(self.img.size[1]-1, 0, -1):
				color = self.img.getpixel((x, y))
				if color != (255, 255, 255):
					return (x + 1) # right

	def new_image_coordinates(self):
		a = self.get_top_right()
		b = self.get_top_left() 
		x = self.get_lower_right()
		y = self.get_lower_left()

		return (a, b, x, y)


if __name__ == "__main__":
	args = get_arguments()
	path = args.input
	output = args.output

	img = Image.open(path)

	img_crop = AutoCrop(img)
	coordinates = img_crop.new_image_coordinates()

	print("Cropping area " + str(coordinates) + " from " + path + " to " + output)

	cropped = img.crop(coordinates)
	cropped.save(output)