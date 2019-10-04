use image::{FilterType, GenericImage, Pixel, ImageBuffer, Rgba, Primitive};
use image::DynamicImage;
use image::imageops;
use std::collections::HashSet;

use saveFile;
use std::hash::Hash;

fn gradient_magnitude(x_gradient: &ImageBuffer<Rgba<u8>, Vec<u8>>, y_gradient: &ImageBuffer<Rgba<u8>, Vec<u8>>, x_pos: u32, y_pos: u32) -> f64
{
    let x_grad_val = x_gradient.get_pixel(x_pos, y_pos).data[0] as f64;
    let y_grad_val = y_gradient.get_pixel(x_pos, y_pos).data[0] as f64;

    (x_grad_val*x_grad_val + y_grad_val*y_grad_val).sqrt()
}

// Implementation based on: https://docs.opencv.org/trunk/da/d22/tutorial_py_canny.html and
//                          https://en.wikipedia.org/wiki/Canny_edge_detector
//
// sigma is a parameter for the Gaussian filter
// min_val and max_val are parameters for the Canny Edge Detector
pub fn edge_detect(image_path: &str, sigma: f32, min_val: f64, max_val: f64) {
    // Get image and process it for edge detection
    let img = image::open(image_path).expect("Opening image failed");
    println!("Processing image...");
    let processed_img = img.grayscale().blur(sigma);

    // Calculate gradients using Sobel's Edge Detection Operator
    println!("Calculating gradients...");
    let x_gradient = imageops::filter3x3(&processed_img, &[-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]);
    let y_gradient = imageops::filter3x3(&processed_img, &[-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0]);

    // Make resulting image based on the Sobel image ... x_gradient.dimensions() == y_gradient.dimensions()
    let (width,height) = x_gradient.dimensions();
    let mut result_img = DynamicImage::new_rgba8(width, height);

    // Perform Non-Maximum Suppression
    println!("Non-Maximum Supression...");
    for x in 0..width {
        for y in 0..height {
            // Because gradients are grayscaled, each color chanel has the same value
            let x_grad_val = x_gradient.get_pixel(x, y).data[0];
            let y_grad_val = y_gradient.get_pixel(x, y).data[0];

            let magnitude = gradient_magnitude(&x_gradient, &y_gradient, x, y);
            let angle = (y_grad_val as f64).atan2(x_grad_val as f64);

            // Move along gradient to see if current pixel is non-maximal
            let mut is_maximal = true;
            let step_size = 2.0;

            let forward_x = x as f64 + step_size*angle.cos();
            let forward_y = y as f64 + step_size*angle.sin();

            // Do bound check and check if pixel in forward direction of gradient is bigger than the current pixel
            if forward_x >= 0.0 && forward_y >= 0.0 && forward_x < width as f64 && forward_y < height as f64 {
                let forward_x = forward_x as u32;
                let forward_y = forward_y as u32;
                if gradient_magnitude(&x_gradient, &y_gradient, forward_x, forward_y) >= magnitude {
                    is_maximal = false;
                }
            }

            let backward_x = x as f64 - step_size*angle.cos();
            let backward_y = y as f64 - step_size*angle.sin();
            // Do bound check and check if pixel in backward direction of gradient is bigger than the current pixel
            if backward_x >= 0.0 && backward_y >= 0.0 && backward_x < width as f64 && backward_y < height as f64 {
                let backward_x = backward_x as u32;
                let backward_y = backward_y as u32;
                if gradient_magnitude(&x_gradient, &y_gradient, backward_x, backward_y) >= magnitude {
                    is_maximal = false;
                }
            }

            if !is_maximal {
                // Zero out non-maximal pixels
                result_img.put_pixel(x, y, Rgba([0, 0, 0, 0]))
            } else {
                let magnitude_int = magnitude as u8;
                // Edge pixels with magnitude under the given minimum value are disregarded as edge pixels
                if magnitude <= min_val {
                    result_img.put_pixel(x, y, Rgba([0, 0, 0, 0]))
                } else {
                    result_img.put_pixel(x, y, Rgba([magnitude_int, magnitude_int, magnitude_int, 0]))
                }
            }

        }
    }

    // Perform Hysteresis Thresholding
    println!("Hysteresis Thresholding...");
    let mut sure_edge_set: HashSet<(u32, u32)> = HashSet::new();
    // Find all edge pixels that are definitely edge pixels
    for x in 0..width {
        for y in 0..height {
            let edge_val = result_img.get_pixel(x, y).data[0];
            if edge_val as f64 >= max_val || sure_edge_set.contains(&(x, y)) {
                // Remember the pixel
                sure_edge_set.insert((x, y));

                let x = x as i64;
                let y = y as i64;
                // Make neighboring non-zero pixels also sure edges
                let neighbors = [(x + 1, y), (x + 1, y + 1), (x, y + 1), (x - 1, y + 1), (x - 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1)];
                // Iterate through each neighbor
                for (neighbor_x, neighbor_y) in &neighbors {
                    // Do bound checks and use floats to prevent subtraction overflow
                    if *neighbor_x >= 0 && *neighbor_y >= 0 && result_img.in_bounds(*neighbor_x as u32, *neighbor_y as u32) {
                        let neighbor_pixel_val = result_img.get_pixel(*neighbor_x as u32, *neighbor_y as u32).data[0];
                        if neighbor_pixel_val > 0 {
                            sure_edge_set.insert((*neighbor_x as u32, *neighbor_y as u32));
                        }
                    }
                }
            }
        }
    }
    // Discard non-edge pixels
    for x in 0..width {
        for y in 0..height {
            if !sure_edge_set.contains(&(x, y)) {
                result_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            } else {
                result_img.put_pixel(x, y, Rgba([255, 255, 255, 0]));
            }
        }
    }


    println!("Saving images...");
    saveFile(&result_img, format!("{}.jpg", image_path).as_str(), "Canny");
}