use crate::clothings::wearable::Wearable;

pub struct Tie {
    color: String,
}

impl Tie {
    pub fn new(color: String) -> Self {
        Self { color }
    }
}


impl Wearable for Tie {
    fn wear(&self) -> String {
        format!("{} tie", self.color)
    }
}
