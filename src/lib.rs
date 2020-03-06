pub mod imagecrop;
pub use imagecrop::ImageCrop;

use std::path::Path;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


pub fn crop_image(input_path: &str, output_path: &str) {
    let mut image = ImageCrop::open(&Path::new(input_path))
        .expect(&format!("Failed to read input file: '{}'", input_path));

    let (top_left_corner, bottom_right_corner) = image.calculate_corners();

    println!("Cropping rectangle (({0}, {1}), ({2}, {3})) from {4} to {5}",
        top_left_corner.x,
        top_left_corner.y,
        bottom_right_corner.x,
        bottom_right_corner.y,
        input_path,
        output_path
    );

    let sub_image = image.original.crop(
        top_left_corner.x,
        top_left_corner.y,
        bottom_right_corner.x - top_left_corner.x,
        bottom_right_corner.y - top_left_corner.y,
    );

    sub_image.save(output_path)
        .expect(&format!(
            "Failed to save input file: '{}' to output path: '{}'",
                input_path,
                output_path)
        );
}

#[pyfunction]
pub fn calculate_corners(input_path: &str) -> PyResult<((u32, u32), (u32, u32))> {
    let image = ImageCrop::open(&Path::new(input_path))
        .expect(&format!("Failed to read input file: '{}'", input_path));

    let (top_left_corner, bottom_right_corner) = image.calculate_corners();
    Ok((
        (top_left_corner.x, top_left_corner.y),
        (bottom_right_corner.x, bottom_right_corner.y)
    ))
}

#[pymodule]
pub fn libauto_image_cropper(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(calculate_corners))?;
    Ok(())
}
