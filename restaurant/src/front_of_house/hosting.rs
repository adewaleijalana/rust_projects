use super::kitchen::Tools;

pub fn add_to_waitlist(tools: &Tools) {
  match tools {
      Tools::KNIFE => println!("{:?}", Tools::KNIFE),
      _ => println!("Only using knife")
  }
  
}