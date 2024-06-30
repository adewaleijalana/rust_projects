mod front_of_house;


pub use crate::front_of_house::{hosting, kitchen};

pub fn eat_at_restaurant() {
    let tool = kitchen::Tools::KNIFE;
    hosting::add_to_waitlist(&tool);
}