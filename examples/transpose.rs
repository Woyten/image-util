extern crate image_util;
extern crate nalgebra;

fn main() {
    let loaded_image = image_util::load_image("test").unwrap();
    let matrix = image_util::to_matrix::<f32>(&loaded_image);
    let transposed = matrix.transpose();
    let transposed_image = image_util::to_image(&transposed);
    image_util::save_image(&transposed_image, "out-transpose").unwrap();
}
