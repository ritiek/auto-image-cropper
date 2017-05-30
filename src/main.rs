extern crate clap;
extern crate image;

use std::fs;
//use std::env::args;
//use std::process::exit;
use std::path::Path;
use clap::{Arg, App};
use image::GenericImage;

fn get_top_left(input_path: &str) -> u32 {
	let im = image::open(&Path::new(input_path)).unwrap();
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

fn get_top_right(input_path: &str) -> u32 {
	let im = image::open(&Path::new(input_path)).unwrap();
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

fn get_lower_left(input_path: &str) -> u32 {
	let im = image::open(&Path::new(input_path)).unwrap();
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

fn get_lower_right(input_path: &str) -> u32 {
	let im = image::open(&Path::new(input_path)).unwrap();
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
	// Top left corner
	let (b, a) = (get_top_left(input_path), get_top_right(input_path));
	// Lower right corner
	let (y, x) = (get_lower_left(input_path), get_lower_right(input_path));

	println!("Cropping area ({0}, {1}, {2}, {3}) from {4} to {5}",
	a, b, x, y, input_path, output_path);

	let mut im = image::open(&Path::new(input_path)).unwrap();
	let subim = im.crop(a, b, x - a, y - b);

	let ref mut fout = fs::File::create(&Path::new(output_path)).unwrap();

	let _ = subim.save(fout, image::JPEG).unwrap();
}

fn main() {
	/* Hold Your Breath */
	let arguments = App::new("auto-image-cropper")
	                    .version("0.1.4")
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

		for file in input_files {
			let img_in = file.unwrap().path();
			let img_name = img_in.file_name();
			let img_out = Path::new(output_path).join(img_name.unwrap());

			crop_image(&(img_in.display().to_string()),
			           &(img_out.into_os_string().into_string().unwrap()));
		}
	}

}
