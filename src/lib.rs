pub mod imagecrop;
pub use imagecrop::ImageCrop;

#[cfg(feature = "python-binding")]
mod python_binding {
    use crate::imagecrop::ImageCrop;

    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    use std::path::Path;

    #[pyfunction]
    fn calculate_corners(input_path: &str) -> PyResult<((u32, u32), (u32, u32))> {
        let image = ImageCrop::open(&Path::new(input_path))
            .expect(&format!("Failed to read input file: '{}'", input_path));

        let (top_left_corner, bottom_right_corner) = image.calculate_corners();
        Ok((
            (top_left_corner.x, top_left_corner.y),
            (bottom_right_corner.x, bottom_right_corner.y)
        ))
    }

    #[pymodule]
    fn libauto_image_cropper(_py: Python, module: &PyModule) -> PyResult<()> {
        module.add_wrapped(wrap_pyfunction!(calculate_corners))?;
        Ok(())
    }
}
