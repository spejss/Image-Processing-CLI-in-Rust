use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use io::{openImage, saveBuffer};
use std::cmp;

const WIDTH: u32 = 255;
const HEIGHT: u32 = 200;

/// Generate histogram of grayscale intensity
pub fn grayscale(i: &str) {
    let operation = "HistogramGrayscale";
    let img = openImage(i);

    let grayscale = img.grayscale();

    // create a vector for storing the number of occurrences of each intensity
    let mut occurrences = vec![0; 255];

    // iterate over each pixel in the image and count the occurrences of each intensity
    let (width, height) = grayscale.dimensions();
    for w in 0..width {
        for h in 0..height {
            let pixel = grayscale.get_pixel(w, h);
            let rgb = pixel.to_rgb();
            occurrences[rgb.data[0] as usize] += 1;
        }
    }

    // find highest value of occurrences, so that we can use it as 100% in the histogram
    let maxValue = *occurrences.iter().max().unwrap();

    let mut buffer = createBuffer();
    draw(&mut buffer, &occurrences, maxValue, [0, 0, 0]);

    saveBuffer(&buffer, &i, &operation);
}

/// Generate histogram of RGB values
pub fn rgb(i: &str) {
    let operation = "Histogram";
    let img = openImage(i);

    // create a vector for storing the number of occurrences of each intensity
    let mut occurrencesR = vec![0; 255];
    let mut occurrencesG = vec![0; 255];
    let mut occurrencesB = vec![0; 255];

    // iterate over each pixel in the image and count the occurrences of each intensity
    let (width, height) = img.dimensions();
    for w in 0..width {
        for h in 0..height {
            let pixel = img.get_pixel(w, h);
            let rgb = pixel.to_rgb();
            occurrencesR[rgb.data[0] as usize] += 1;
            occurrencesG[rgb.data[1] as usize] += 1;
            occurrencesB[rgb.data[2] as usize] += 1;
        }
    }

    // find highest value of occurrences, so that we can use it as 100% in the histogram
    let maxValueR = *occurrencesR.iter().max().unwrap();
    let maxValueG = *occurrencesG.iter().max().unwrap();
    let maxValueB = *occurrencesB.iter().max().unwrap();

    let mut buffer = createBuffer();
    draw(&mut buffer, &occurrencesR, maxValueR, [255, 0, 0]);
    draw(&mut buffer, &occurrencesG, maxValueG, [0, 255, 0]);
    draw(&mut buffer, &occurrencesB, maxValueB, [0, 0, 255]);

    saveBuffer(&buffer, &i, &operation);
}

fn createBuffer() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            buffer.get_pixel_mut(w, h).data = [255, 255, 255];
        }
    }
    buffer
}

fn draw(buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, values: &[u32], max: u32, data: [u8; 3]) {
    for i in 0..255_u8 {
        let mut height = cmp::min(
            ((values[usize::from(i)] as f32 / max as f32) * 200.0) as u32,
            u32::from(u8::max_value()),
        ) as u8;
        let c = (HEIGHT - 1).saturating_sub(u32::from(height));
        let mut pixel = buffer.get_pixel_mut(u32::from(i), c);
        pixel.data = data;
    }
}
