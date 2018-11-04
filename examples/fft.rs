use math_util::fft;
use math_util::fft::TransformDirection;
use nalgebra::DMatrix;
use num::complex::Complex;
use std::f32;

fn main() {
    let loaded_image = image_util::to_matrix(&image_util::load_image("test").unwrap());
    let shape = loaded_image.shape();
    let len = loaded_image.len();

    let freq_window = DMatrix::from_fn(shape.0, shape.1, |x, y| {
        let dx = if x < shape.0 / 2 { x } else { shape.0 - x } as f32;
        let dy = if y < shape.1 / 2 { y } else { shape.1 - y } as f32;
        Complex::new(f32::max(0.0, 1.0 - (dx * dx + dy * dy).sqrt() / 20.0), 0.0)
    });
    let transformed = fft::transform_2d(loaded_image, TransformDirection::Forward).component_mul(&freq_window);

    let norm_factor = Complex::new(len as f32, 0.0);
    let output = fft::transform_2d(transformed, TransformDirection::Backward) / norm_factor;

    image_util::save_image(&image_util::to_image(&output), "out-fft").unwrap();
}
