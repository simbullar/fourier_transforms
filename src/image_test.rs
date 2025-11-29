use image::GenericImageView;

mod DFT;
mod FFT;

fn process_image(image_path: &str) -> Vec<Vec<(f64, f64)>> {
    let img = image::open(image_path).unwrap();

    let (width, height) = img.dimensions();
    let mut img_representation: Vec<Vec<(f64, f64)>> =
        vec![vec![(0.0, 0.0); width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0[0] as f64;
            img_representation[y as usize][x as usize] = (pixel, 0.0);
        }
    }
    img_representation
}

fn main() {
    let image_path = "test_data/dog.jpg";
    let image_representation = process_image(image_path);

    let dft_result = DFT::dft_2d(image_representation); // or fft_2d
}
