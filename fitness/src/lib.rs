#![allow(unused, dead_code)]

mod diet {
    const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program() {
        println!("The nutritionist is {NUTRITIONIST}");
    }
}

pub mod cardio;
pub mod weightlifting;

use cardio::exercise::Exercise as CardioExercise;
use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new(cardio: CardioExercise, weightlifting: WeightliftingExercise) -> Self {
        weightlifting::ask_about_program();
        cardio::ask_about_program();
        diet::ask_about_program();
        Self {
            cardio,
            weightlifting,
        }
    }
}
