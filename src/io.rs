use image::{self, DynamicImage, ImageBuffer, Rgb};
use std::fs::File;

pub fn openImage(i: &str) -> DynamicImage {
    image::open(i).expect("Opening image failed")
}

pub fn saveImage(img: &DynamicImage, i: &str, operation: &str) {
    let outputPath = generateFilename(i, operation);
    let mut out = File::create(outputPath.0).unwrap();
    let format = match outputPath.1.to_lowercase().as_ref() {
        "jpg" | "jpeg" => Some(image::JPEG),
        "png" => Some(image::PNG),
        "gif" => Some(image::GIF),
        "bmp" => Some(image::BMP),
        "ico" => Some(image::ICO),
        _ => None,
    };
    if let Some(format) = format {
        img.save(&mut out, format).expect("Saving image failed");
    } else {
        println!("Unsupported file format");
    }
}

pub fn saveBuffer(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>, i: &str, operation: &str) {
    let outputPath = generateFilename(i, operation);
    buffer.save(outputPath.0).unwrap();
}

fn generateFilename(i: &str, operation: &str) -> (String, String) {
    let mut outputPath: String = i.chars().take(i.len() - 4).collect();
    let ext: String = i.chars().skip(i.len() - 3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    (outputPath, ext)
}
