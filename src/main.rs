extern crate image;

//use std::fs::File;
use std::path::Path;
use image::{
		GenericImage,
//		imageops
};

fn get_top_left(path: &str) -> u32 {
    let im = image::open(&Path::new(path)).unwrap();
	for x in 0..(im.dimensions().1) {
		for y in 0..(im.dimensions().0) {
			let col = im.get_pixel(y, x);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return y;
			}
		}
	}
	unreachable!();
}

fn get_top_right(path: &str) -> u32 {
    let im = image::open(&Path::new(path)).unwrap();
	for x in 0..(im.dimensions().0) {
		for y in 0..(im.dimensions().1) {
			let col = im.get_pixel(x, y);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return y;
			}
		}
	}
	unreachable!();
}

fn get_lower_left(path: &str) -> u32 {
    let im = image::open(&Path::new(path)).unwrap();
	let mut x = im.dimensions().1 as i32 - 1;
	let mut y = im.dimensions().0 as i32 - 1;
	while x >= 0 {
		while y >= 0 {
			let col = im.get_pixel(y as u32, x as u32);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return y as u32 + 1;
			}
			x -= 1;
			y -= 1;
		}
	}
	unreachable!();
}

fn get_lower_right(path: &str) -> u32 {
    let im = image::open(&Path::new(path)).unwrap();
	let mut x = im.dimensions().0 as i32 - 1;
	let mut y = im.dimensions().1 as i32 - 1;
	while x >= 0 {
		while y >= 0 {
			let col = im.get_pixel(x as u32, y as u32);
			if col[0] != 255 && col[1] != 255 && col[2] != 255 {
				return y as u32 + 1;
			}
			x -= 1;
			y -= 1;
		}
	}
	unreachable!();
}

fn main() {
    let path = "test.png";

	let a = get_top_left(path);
	let b = get_top_right(path);
	let x = get_lower_left(path);
	let y = get_lower_right(path);
	println!("({0}, {1}, {2}, {3})", a, b, x, y)

   /*let mut im = image::open(&Path::new(path)).unwrap();
	let subim = imageops::crop(&mut im, a, b, x, y);

    let ref mut fout = File::create(&Path::new("result.png")).unwrap();

    let _ = subim.save(fout, image::PNG).unwrap();*/
}
