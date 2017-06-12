extern crate clap;
extern crate image;
extern crate rayon;

use std::fs;
use std::fs::{DirEntry};
use std::path::Path;
use clap::{Arg, App};
use image::GenericImage;
use rayon::prelude::*;

fn get_top_left(im: &image::DynamicImage) -> u32 {
	for x in 0..(im.dimensions().1) {
		for y in 0..(im.dimensions().0) {
			let col = im.get_pixel(y, x);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return x;
			}
		}
	}
	unreachable!();
}

fn get_top_right(im: &image::DynamicImage) -> u32 {
	for x in 0..(im.dimensions().0) {
		for y in 0..(im.dimensions().1) {
			let col = im.get_pixel(x, y);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return x;
			}
		}
	}
	unreachable!();
}

fn get_lower_left(im: &image::DynamicImage) -> u32 {
	let mut x = im.dimensions().1 as i32 - 1;
	// Using while loop as there is no reliable way
	// to use custom steps in range() currently
	while x >= 0 {
		let mut y = im.dimensions().0 as i32 - 1;
		while y >= 0 {
			let col = im.get_pixel(y as u32, x as u32);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return x as u32 + 1;
			}
			y -= 1;
		}
		x -= 1;
	}
	unreachable!();
}

fn get_lower_right(im: &image::DynamicImage) -> u32 {
	let mut x = im.dimensions().0 as i32 - 1;
	// Using while loop as there is no reliable way
	// to use custom steps in range() currently
	while x >= 0 {
		let mut y = im.dimensions().1 as i32 - 1;
		while y >= 0 {
			let col = im.get_pixel(x as u32, y as u32);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return x as u32 + 1;
			}
			y -= 1;
		}
		x -= 1;
	}
	unreachable!();
}

fn crop_image(input_path: &str, output_path: &str) {
	// Load image:
	let mut image = image::open(&Path::new(input_path)).unwrap();
	// Top left corner
	let (b, a) = (get_top_left(&image), get_top_right(&image));
	// Lower right corner
	let (y, x) = (get_lower_left(&image), get_lower_right(&image));

	println!("Cropping area ({0}, {1}, {2}, {3}) from {4} to {5}",
	a, b, x, y, input_path, output_path);

	let subim = image.crop(a, b, x - a, y - b);

	let fout = fs::File::create(&Path::new(output_path)).unwrap();
	let ref mut fout = std::io::BufWriter::new(fout);

	let _ = subim.save(fout, image::JPEG).unwrap();
}

fn main() {
	/* Hold Your Breath */
	let arguments = App::new("auto-image-cropper")
	                    .version("0.1.5")
	                    .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
	                    .about("Removes extra white boundaries from images to correctly resize canvas.")

	                    .arg(Arg::with_name("input")
	                    .short("i")
	                    .long("input")
	                    .value_name("LOCATION")
	                    .required(true)
	                    .takes_value(true)
	                    .help("Location of input image/directory"))

	                    .arg(Arg::with_name("output")
	                    .short("o")
	                    .long("output")
	                    .value_name("LOCATION")
	                    .takes_value(true)
	                    .help("Location of output image/directory"))

	                    .get_matches();

	let input_path = arguments.value_of("input").unwrap();
	let output_path = arguments.value_of("output")
	                        .unwrap_or(input_path);
	let path_type = fs::metadata(input_path).unwrap();

	if path_type.is_file() {
		crop_image(input_path, output_path);

	} else {
		let input_files = fs::read_dir(input_path).unwrap();

		if !Path::new(output_path).exists() {
			let _ = fs::create_dir(output_path);
		}
		let mut files_to_crop: Vec<DirEntry> = Vec::with_capacity(10);
		for file in input_files {
			files_to_crop.push(file.unwrap());
		}
		println!("Vector length is {}", files_to_crop.len());

		files_to_crop
			.par_iter()
			.for_each(|file| {
				let img_in = file.path();
				let img_name = img_in.file_name();
				let img_out = Path::new(output_path).join(img_name.unwrap());

				crop_image(&(img_in.display().to_string()),
						&(img_out.into_os_string().into_string().unwrap()));
			});
	}

}
