use std::fs;
use super::image::Image;

static IMAGE_HEADER_LENGTH: usize = 16;

pub fn load_images(file: &str) -> Vec<Image> {
  let raw_images = fs::read(file).expect("Data not found");
  format_images(raw_images)
}

pub fn load_labels(file: &str) -> Vec<u8> {
  let raw_labels: Vec<u8> = fs::read(file).expect("Labels not found");
  let labels = raw_labels[8..].to_vec();
  labels
}

fn format_images(raw_data: Vec<u8>) -> Vec<Image> {
  let mut images: Vec<Image> = Vec::new();
  let rows = 28;
  let columns = 28;

  for img_start in (IMAGE_HEADER_LENGTH..raw_data.len()).step_by(rows * columns) {
      let mut img = Vec::with_capacity(rows);
      for row in 0..rows {
          let mut img_row = Vec::with_capacity(columns);
          let row_start = img_start + row * columns;
          let row_slice = &raw_data[row_start..(row_start+columns)];
          img_row.extend_from_slice(row_slice);
          img.push(img_row);
      }
      images.push(Image::new(img));
  }
  images
}