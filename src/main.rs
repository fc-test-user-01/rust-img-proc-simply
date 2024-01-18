extern crate image;

use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Open an image file
    let img = image::open("input.jpg")?;

    // Define maximum values as a tuple (max_file_size, max_width, max_height)
    let (max_file_size, max_width, max_height) = (2 * 1024, 200, 200);

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    let resized_img: DynamicImage;

    if width > max_width || height > max_height {
        // Resize the image to the desired dimensions (1:1 aspect ratio)
        resized_img = img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3);
    } else {
        // Use the original image when it's smaller than max_width and max_height
        resized_img = img.clone();
    }

    // Reduce the image quality to fit within the max file size
    let mut quality = 100;
    loop {
        let mut output = fs::File::create("/workspace/dist/output.jpg")?;
        resized_img.write_to(&mut output, ImageOutputFormat::Jpeg(quality))?;

        // Check the size of the output file
        let size_in_bytes = fs::metadata("/workspace/dist/output.jpg")?.len();

        // Check if the size exceeds max_file_size KiB
        if size_in_bytes > max_file_size {
            // Reduce the quality for the next iteration
            quality -= 5;
            if quality < 5 {
                break;
            }
        } else {
            break;
        }
    }

    Ok(())
}
