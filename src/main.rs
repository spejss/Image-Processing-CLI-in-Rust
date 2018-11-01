#![allow(non_snake_case)]

extern crate clap;
extern crate image;

mod histogram;
mod io;

use clap::{App, Arg};
use image::{FilterType, GenericImage, Pixel, Rgb};
use io::{openImage, saveBuffer, saveImage};

fn main() {
    let matches = App::new("IPCLI")
        .version("0.1")
        .author("Mikolaj Wawrzyniak <mikolaj.wawrzyniak at fh-dortmund.de>")
        .about("Basic CLI for image processing")
        .arg(Arg::with_name("operation")
            .short("o")
            .long("operation")
            .help("Specifies operation to be done on the image")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("value")
            .short("v")
            .long("value")
            .help("Value for the transformation. To see what values are needed, check the documentation.")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("image")
            .short("i")
            .long("image")
            .value_name("FILE")
            .help("Opens specified image file and uses it for transformations.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let imagePath = matches.value_of("image").unwrap_or("empty");
    let operation = matches.value_of("operation").unwrap_or("empty");
    let value = matches.value_of("value").unwrap_or("empty");

    match operation {
        "average" => averageColor(imagePath),
        "copy" => copy(imagePath),
        "thumbnail" => {
            let size: u32 = value.parse().unwrap();
            createThumbnail(imagePath, size)
        }
        "blur" => {
            let v: f32 = value.parse().unwrap();
            gaussianBlur(imagePath, v)
        }
        "brighten" => {
            let v: i32 = value.parse().unwrap();
            brighten(imagePath, v)
        }
        "huerotate" => {
            let v: i32 = value.parse().unwrap();
            huerotate(imagePath, v)
        }
        "contrast" => {
            let v: f32 = value.parse().unwrap();
            contrast(imagePath, v);
        }
        "grayscale" => {
            grayscale(imagePath);
        }
        "invert" => {
            invert(imagePath);
        }
        "histogramGrayscale" => histogram::grayscale(imagePath),
        "histogram" => histogram::rgb(imagePath),
        "binaryThreshold" => {
            let v: u8 = value.parse().unwrap();
            binary_threshold(imagePath, v);
        }
        _ => println!("Operation not recognised!"),
    }
}

fn averageColor(i: &str) {
    let img = openImage(i);
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let rgb = px.to_rgb();
            r = (r + u32::from(rgb.data[0])) / 2;
            g = (g + u32::from(rgb.data[1])) / 2;
            b = (b + u32::from(rgb.data[2])) / 2;
        }
    }
    println!("Average color is: RGB {} {} {}", r, g, b);
}

fn createThumbnail(i: &str, size: u32) {
    let operation = "Thumbnail";
    let img = openImage(i);
    let thumbnail = img.resize(size, size, FilterType::Lanczos3);
    saveImage(&thumbnail, &i, &operation);
}

fn copy(i: &str) {
    let operation = "Copy";
    let img = openImage(i);
    saveImage(&img, &i, &operation);
}

fn gaussianBlur(i: &str, v: f32) {
    let operation = "GuassianBlur";
    let img = openImage(i);
    let blurred = img.blur(v);
    saveImage(&blurred, &i, &operation);
}

fn brighten(i: &str, v: i32) {
    let operation = "Brighten";
    let img = openImage(i);
    let brightened = img.brighten(v);
    saveImage(&brightened, &i, &operation);
}

fn huerotate(i: &str, v: i32) {
    let operation = "Huerotate";
    let img = openImage(i);
    let huerotated = img.huerotate(v);
    saveImage(&huerotated, &i, &operation);
}

fn contrast(i: &str, v: f32) {
    let operation = "AdjustContrast";
    let img = openImage(i);
    let contrast = img.adjust_contrast(v);
    saveImage(&contrast, &i, &operation);
}

fn grayscale(i: &str) {
    let operation = "Grayscale";
    let img = openImage(i);
    let grayscale = img.grayscale();
    saveImage(&grayscale, &i, &operation);
}

fn invert(i: &str) {
    let operation = "Invert";
    let mut img = openImage(i);
    img.invert();
    saveImage(&img, &i, &operation);
}

fn binary_threshold(i: &str, low: u8) {
    let operation = "BinaryThreshold";
    let img = openImage(i);
    let grayscale = img.grayscale();
    let (width, height) = img.dimensions();
    // create buffer to draw the image after binary threshold
    let mut buffer = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    for w in 0..width {
        for h in 0..height {
            let pixel = grayscale.get_pixel(w, h);
            let rgb = pixel.to_rgb();
            let intensity = rgb.data[0];
            if low < intensity {
                buffer.get_pixel_mut(w, h).data = [255, 255, 255];
            } else {
                buffer.get_pixel_mut(w, h).data = [0, 0, 0];
            }
        }
    }
    saveBuffer(&buffer, &i, &operation);
}
