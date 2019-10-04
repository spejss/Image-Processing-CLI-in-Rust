extern crate clap;
extern crate image;

mod canny;

use clap::{Arg,App,SubCommand};
use std::fs::File;
use image::{FilterType, GenericImage, Pixel,ImageBuffer,Rgb};
use image::DynamicImage;
use std::path::Path;
use std::collections::HashMap;

fn main() {
     let matches = App::new("IPCLI ")
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
    //println!("Transforming the image: {}", imagePath);

    let operation = matches.value_of("operation").unwrap_or("empty");
    //println!("Using operation: {}", operation);

    let value = matches.value_of("value").unwrap_or("empty");
    //println!("Value: {}", value);

    match operation.as_ref(){
        "average" => {averageColor(imagePath)}
        "copy" => {copy(imagePath)}
        "thumbnail" => {
            let size: u32 = value.parse().unwrap();
            createThumnbail(imagePath, size)}
        "blur" => {
            let v : f32 = value.parse().unwrap();
            gaussianBlur(imagePath,v)}
        "brighten" => {
            let v: i32 = value.parse().unwrap();
            brighten(imagePath,v)}
        "huerotate" => {
            let v: i32 = value.parse().unwrap();
            huerotate(imagePath,v)}
        "contrast" => {
            let v: f32 = value.parse().unwrap();
            contrast(imagePath,v);}
        "grayscale" => {
            grayscale(imagePath);
        }
        "invert" => {
            invert(imagePath);
        }
        "histogramGrayscale" => {histogramGrayscale(imagePath)}
        "histogram" => {histogram(imagePath)}
        "binaryTreshold" => {
            let v: u8 = value.parse().unwrap();
            binary_treshold(imagePath,v);
            }
        "canny" => {
            // Perform input checking
            let values: Vec<&str> = value.split_whitespace().collect();
            if let (Ok(sigma), Ok(min_val), Ok(max_val)) = (values[0].parse::<f32>(), values[1].parse::<f64>(), values[2].parse::<f64>()) {
                canny::edge_detect(imagePath, sigma, min_val, max_val);
            } else {
                println!("Canny Edge Detection Requires 3 Parameters: sigma min_val max_val")
            }
        }
        _ => {println!("Not implemented yet!")}
    }
}

fn averageColor(i: &str){
    let img = image::open(i).expect("Opening image failed");
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let (width,height) = img.dimensions();
    for x in 0..(width){
        for y in 0..(height){
            let px = img.get_pixel(x,y);
            let rgb = px.to_rgb();
            r = (r as u32 + rgb.data[0] as u32)/2;
            g = (g as u32 + rgb.data[1] as u32)/2;
            b = (b as u32 + rgb.data[2] as u32)/2;
            println!("R G B {} {} {}", r,g,b);
        }
    }
    println!("Average color is: RGB {} {} {}",r,g,b);
}

fn createThumnbail(i: &str, size: u32){
    let operation = "Thumbnail";
    let img = image::open(i).expect("Opening image failed");
    let thumbnail = img.resize(size,size, FilterType::Lanczos3);
    saveFile(&thumbnail, &i, &operation);

}

fn copy(i: &str){
    let operation = "Copy";
    let img = image::open(i).expect("Opening image failed");
    saveFile(&img, &i, &operation);
}

fn gaussianBlur(i: &str, v: f32){
    let operation = "GuassianBlur";
    let img = image::open(i).expect("Opening image failed");
    let blurred = img.blur(v);
    saveFile(&blurred, &i, &operation);
}

fn brighten(i: &str,v: i32){
    let operation = "Brighten";
    let img = image::open(i).expect("Opening image failed");
    let brightened = img.brighten(v);
    saveFile(&brightened, &i, &operation);
}

fn huerotate(i: &str, v: i32){
    let operation = "Huerotate";
    let img = image::open(i).expect("Opening image failed");
    let huerotated = img.huerotate(v);
    saveFile(&huerotated, &i, &operation);
}

fn contrast(i: &str, v: f32){
    let operation = "AdjustContrast";
    let img = image::open(i).expect("Opening image failed");
    let contrast = img.adjust_contrast(v);
    saveFile(&contrast, &i, &operation);
}

fn grayscale(i: &str){
    let operation = "Grayscale";
    let img = image::open(i).expect("Opening image failed");
    let grayscale = img.grayscale();
    saveFile(&grayscale, &i, &operation);
}

fn invert(i: &str){
    let operation = "Invert";
    let mut img = image::open(i).expect("Opening image failed");
    img.invert();
    saveFile(&img, &i, &operation);
}

fn binary_treshold(i: &str, low: u8){
    let operation = "BinaryTreshold";
    let img = image::open(i).expect("Opening image failed");
    let mut grayscale = img.grayscale();
    let (width,height) = img.dimensions();
    // create image to draw the image after binary treshold
    let mut image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    for w in 0..(width){
        for h in 0..(height){
            let pixel = grayscale.get_pixel(w,h);
            let rgb = pixel.to_rgb();
            let intensity = rgb.data[0];
            if low < intensity {
                image.get_pixel_mut(w,h).data = [255,255,255];
            }else{
                image.get_pixel_mut(w,h,).data = [0,0,0];
            }
        }
    }
    let mut outputPath: String = i.chars().take(i.len()-4).collect();
    let ext: String = i.chars().skip(i.len()-3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    image.save(outputPath).unwrap();
}

/// Generate histogram for grayscale images
fn histogramGrayscale(i: &str){
    let operation = "HistogramGrayscale";
    // size of the output image containing the histogram
    let WIDTH = 255;
    let HEIGHT = 200;

    // open image and convert it to grayscale
    let img = image::open(i).expect("Opening image failed");
    let grayscale = img.grayscale();

    // create a hashmap for storing the number of occurences of each intensity
    let mut occurences = HashMap::new();
    // and fill it with a placeholder value
    for fill in 0..255{
        &occurences.insert(fill,0);        
    }

    // iterate over each pixel in the image and count the occurences of each intensity
    let (width,height) = grayscale.dimensions();
    for w in 0..(width){
        for h in 0..(height){
            let pixel = grayscale.get_pixel(w,h);
            let rgb = pixel.to_rgb();
            let intensity = rgb.data[0];
            match occurences.get(&intensity){
                Some(&oc) => {
                    let mut current = oc;
                    current = current + 1;
                    &occurences.insert(intensity,current);
                    }
                _ => {&occurences.insert(w as u8,0);}
            }
        }
    }

    // find highest value of occurences, so that we can use it as 100% in the histogram
    let mut maxValue = 0;
    for (occurences, &value) in occurences.iter() {
        if(value > maxValue){
            maxValue = value;
        }
    }

    // create image to draw the histogram on
    let mut image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);

    // dirty hack: fill the image with white pixels
    for w in 0..WIDTH{
        for h in 0..HEIGHT{
            image.get_pixel_mut(w,h).data = [255,255,255];
        } 
    }

    // dirty hack: intensity index
    let mut cc: u32 = 0;
    // Potential bug: 254 and 255 cause panic!
    for i in 0..253{
        match occurences.get(&i) {
            Some(&value) => {
                let mut height = ((value as f32 / maxValue as f32) * 300.0) as u8;
                let mut pixel = image.get_pixel_mut(cc, (HEIGHT-1) - height as u32);
                pixel.data = [0,0,0];
            },
            _ => println!("Value out of bounds #BarryBonds"),
        }
        cc = cc + 1;
    }

    let mut outputPath: String = i.chars().take(i.len()-4).collect();
    let ext: String = i.chars().skip(i.len()-3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    image.save(outputPath).unwrap();
}

/// Generate histogram for grayscale images
fn histogram(i: &str){
    let operation = "Histogram";
    // size of the output image containing the histogram
    let WIDTH = 255;
    let HEIGHT = 200;

    // open image and convert it to grayscale
    let img = image::open(i).expect("Opening image failed");

    // create a hashmap for storing the number of occurences of each intensity
    let mut occurencesR = HashMap::new();
    let mut occurencesG = HashMap::new();
    let mut occurencesB = HashMap::new();
    // and fill it with a placeholder value
    for fill in 0..255{
        &occurencesR.insert(fill,0);   
        &occurencesG.insert(fill,0);  
        &occurencesB.insert(fill,0);       
    }

    // iterate over each pixel in the image and count the occurences of each intensity
    let (width,height) = img.dimensions();
    for w in 0..(width){
        for h in 0..(height){
            let pixel = img.get_pixel(w,h);
            let rgb = pixel.to_rgb();
            let intensityR = rgb.data[0];
            let intensityG = rgb.data[1];
            let intensityB = rgb.data[2];
            match occurencesR.get(&intensityR){
                Some(&oc) => {
                    let mut current = oc;
                    current = current + 1;
                    &occurencesR.insert(intensityR,current);
                    }
                _ => {&occurencesR.insert(w as u8,0);}
            }
            match occurencesG.get(&intensityG){
                Some(&oc) => {
                    let mut current = oc;
                    current = current + 1;
                    &occurencesG.insert(intensityG,current);
                    }
                _ => {&occurencesG.insert(w as u8,0);}
            }
            match occurencesB.get(&intensityB){
                Some(&oc) => {
                    let mut current = oc;
                    current = current + 1;
                    &occurencesB.insert(intensityB,current);
                    }
                _ => {&occurencesB.insert(w as u8,0);}
            }
        }
    }

    // find highest value of occurences, so that we can use it as 100% in the histogram
    let mut maxValueR = 0;
    let mut maxValueG = 0;
    let mut maxValueB = 0;
    for (occurencesR, &value) in occurencesR.iter() {
        if(value > maxValueR){
            maxValueR = value;
        }
    }
    for (occurencesG, &value) in occurencesG.iter() {
        if(value > maxValueG){
            maxValueG = value;
        }
    }
    for (occurencesB, &value) in occurencesB.iter() {
        if(value > maxValueB){
            maxValueB = value;
        }
    }

    // create image to draw the histogram on
    let mut image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);

    // dirty hack: fill the image with white pixels
    for w in 0..WIDTH{
        for h in 0..HEIGHT{
            image.get_pixel_mut(w,h).data = [255,255,255];
        } 
    }

    // dirty hack: intensity index
    let mut cc: u32 = 0;
    // Potential bug: 254 and 255 cause panic!
    for i in 0..253{
        match occurencesR.get(&i) {
            Some(&value) => {
                let mut height = ((value as f32 / maxValueR as f32) * 200.0) as u8;
                let mut pixel = image.get_pixel_mut(cc, (HEIGHT-1) - height as u32);
                pixel.data = [255,0,0];
            },
            _ => println!("Value out of bounds #BarryBonds"),
        }
        match occurencesG.get(&i) {
            Some(&value) => {
                let mut height = ((value as f32 / maxValueG as f32) * 200.0) as u8;
                let mut pixel = image.get_pixel_mut(cc, (HEIGHT-1) - height as u32);
                pixel.data = [0,255,0];
            },
            _ => println!("Value out of bounds #BarryBonds"),
        }
        match occurencesB.get(&i) {
            Some(&value) => {
                let mut height = ((value as f32 / maxValueB as f32) * 200.0) as u8;
                let mut pixel = image.get_pixel_mut(cc, (HEIGHT-1) - height as u32);
                pixel.data = [0,0,255];
            },
            _ => println!("Value out of bounds #BarryBonds"),
        }
        cc = cc + 1;
    }

    let mut outputPath: String = i.chars().take(i.len()-4).collect();
    let ext: String = i.chars().skip(i.len()-3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    image.save(outputPath).unwrap();
}

fn saveFile(img: &DynamicImage, i: &str, operation: &str){
    let mut outputPath: String = i.chars().take(i.len()-4).collect();
    let ext: String = i.chars().skip(i.len()-3).take(3).collect();
    outputPath.push_str(operation);
    outputPath.push_str(".");
    outputPath.push_str(&ext);
    println!("Output path: {}", outputPath);
    let mut out = File::create(outputPath).unwrap();
    match ext.as_ref() {
        "jpg" | "JPG" | "jpeg" | "JPEG" => {img.save(&mut out, image::JPEG).expect("Saving image failed");}
        "png" | "PNG" => {img.save(&mut out, image::PNG).expect("Saving image failed");}
        "gif" | "GIF" => {img.save(&mut out, image::GIF).expect("Saving image failed");}
        "bmp" | "BMP" => {img.save(&mut out, image::BMP).expect("Saving image failed");}
        "ico" | "ICO" => {img.save(&mut out, image::ICO).expect("Saving image failed");}
        _ => {println!("Unsupported file format")}
    }
}

