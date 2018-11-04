use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::ImageFormat;
use image::ImageLuma8;
use image::ImageResult;
use nalgebra::DMatrix;
use nalgebra::Scalar;
use num::complex::Complex;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;

const U8_MAX: f32 = ::std::u8::MAX as f32;

pub fn load_image(path: &str) -> ImageResult<DynamicImage> {
    let path_with_extension = Path::new(path).with_extension("png");
    let open_file = BufReader::new(File::open(path_with_extension)?);
    image::load(open_file, ImageFormat::PNG)
}

pub fn save_image(image: &DynamicImage, path: &str) -> Result<()> {
    let path_with_extension = Path::new(path);
    image.save(path_with_extension)
}

pub trait FromRawPixel: Scalar {
    fn from_raw_pixel(from: u8) -> Self;
}

impl FromRawPixel for f32 {
    fn from_raw_pixel(from: u8) -> Self {
        f32::from(from) * 2.0 / U8_MAX - 1.0
    }
}

impl FromRawPixel for Complex<f32> {
    fn from_raw_pixel(from: u8) -> Self {
        Complex::new(f32::from_raw_pixel(from), 0.0)
    }
}

pub fn to_matrix<P: FromRawPixel>(image: &DynamicImage) -> DMatrix<P> {
    let as_luma8 = ImageLuma8(image.to_luma());
    DMatrix::from_iterator(
        num::cast(image.width()).unwrap(),
        num::cast(image.height()).unwrap(),
        as_luma8
            .raw_pixels()
            .into_iter()
            .map(FromRawPixel::from_raw_pixel),
    )
}

pub trait ToRawPixel: Scalar {
    fn to_raw_pixel(&self) -> u8;
}

impl ToRawPixel for f32 {
    fn to_raw_pixel(&self) -> u8 {
        num::cast((self + 1.0) / 2.0 * U8_MAX).unwrap()
    }
}

impl ToRawPixel for Complex<f32> {
    fn to_raw_pixel(&self) -> u8 {
        self.re.to_raw_pixel()
    }
}

pub fn to_image<P: ToRawPixel>(matrix: &DMatrix<P>) -> DynamicImage {
    let pixels = matrix
        .as_slice()
        .into_iter()
        .map(ToRawPixel::to_raw_pixel)
        .collect::<Vec<_>>();
    let buffer = ImageBuffer::from_raw(
        num::cast(matrix.nrows()).unwrap(),
        num::cast(matrix.ncols()).unwrap(),
        pixels,
    )
    .unwrap();
    ImageLuma8(buffer)
}
