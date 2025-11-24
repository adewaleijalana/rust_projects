
#[derive(Debug, Clone)]
pub struct Car {
  pub mpg: String,
  pub color: String,
  pub top_speed: u16,
}

impl Car {

  pub fn new(mpg: String, color: String, top_speed: u16) -> Self {
    Car { mpg, color, top_speed }
  }

  pub fn set_mpg(&mut self, mpg: String) {
    self.mpg = mpg;
  }

  pub fn set_color(&mut self, color: String) {
    self.color = color;
  }

  pub fn set_top_speed(&mut self, top_speed: u16) {
    self.top_speed = top_speed;
  }
}