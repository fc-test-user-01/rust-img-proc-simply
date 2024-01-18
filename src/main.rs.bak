use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
  let data_url = "data:image/jpeg;base64,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";
  // Define a regex pattern for the substring "cde"
  let pattern = Regex::new(r"data:image/.*;base64,").expect("Invalid regex pattern");
  let replaced_string = pattern.replace_all(&data_url, "");

  let decoded = general_purpose::STANDARD.decode(replaced_string.trim()).unwrap();

  // Create a DynamicImage from the decoded image data
  let img = image::load_from_memory(&decoded)?;

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
  let mut base64_string: String;

  loop {
      // Create a buffer to hold the image data
      let mut bytes: Vec<u8> = Vec::new();
      let mut buffer = Cursor::new(&mut bytes);
      resized_img.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(quality))?;
      let size_in_bytes = buffer.get_ref().len();
      // Get a reference to the underlying buffer
      let buffer_slice: &[u8] = buffer.get_ref().as_slice();
      // Encode the byte slice to a Base64-encoded string
      base64_string = general_purpose::STANDARD.encode(buffer_slice);

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
  println!("data:image/jpeg;base64,{}", base64_string);

  Ok(())
}
