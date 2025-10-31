pub mod cardio_tool;
pub mod exercise;

// pub use cardio_tool::CardioTool;
// pub use exercise::Exercise;

const PERSONAL_TRAINER: &str = "Carl Cardio";

pub fn ask_about_program() {
    println!("The cardio trainer is {PERSONAL_TRAINER}");
}
