pub enum Language {
  ENGLISH(String),
  SPANISH,
  SWEDISH,
  RUSSIAN
}

pub struct Point{
  pub x: i32,
  pub y: i32,
}

impl Point {
    pub fn new(x: i32,y: i32) -> Self{
      Point { x, y}
    }
}


pub fn pattern_match(lang: &Language){
  match lang{
    Language::ENGLISH(text) => println!("I can speak English {text}"),
    Language::SPANISH => println!("I can speak Spanish"),
    Language::SWEDISH => println!("I can speak Swedish"),
    Language::RUSSIAN => println!("I can speak Russina"),
  }
}

pub fn match_struct(point: &Point) {
  match point {
      Point { x:0, y } => println!("X is zero, y is {y}"),
      Point { x, y: 0 } => println!("X is {x}, y is zero"),
      Point { x, y } => println!("X is {x}, y is {y}"),
  }
    
}
