use std::fmt;

#[derive(Debug)]
pub struct Image {
    image: Vec<Vec<u8>>
}

impl Image {
  pub fn new(image: Vec<Vec<u8>>) -> Image {
      Image { image }
  }
}

impl fmt::Display for Image {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "\n")?;
      for row in self.image.iter() {
          let temp: Vec<&str> = row.clone().iter_mut().map(|x| {
              if *x > 0 {"8"} 
              else {" "}
          }).collect();
          write!(f, "{:?}\n", temp )?;
      }
      Ok(())
  }
}