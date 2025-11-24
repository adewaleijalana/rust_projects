#![allow(unused, dead_code)]

use fitness::GymWorkout;
use fitness::cardio::cardio_tool::CardioTool;
use fitness::cardio::exercise::Exercise as CardioExercise;
use fitness::weightlifting::Exercise as WeightliftingExercise;

fn main() {
    let cardio = CardioExercise::new("Monda".to_string(), CardioTool::Treadmill, 30);
    let weightlifting = WeightliftingExercise::new("Rose".to_string(), 10);
    let workout = GymWorkout::new(cardio, weightlifting);
    println!("{:#?}", workout);
}
