use std::fmt::Debug;

use rand::{RngExt, rng};

use crate::thing::Thing;

pub trait Enchanter: Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let success_probability = self.competency();

        let spell_is_successful = rng().random_bool(success_probability);

        print!("{:?} mutters incoherently. ", self);

        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        } else {
            println!(
                "The {:?} fizzes, \
 then turns into a worthless trinket.",
                thing
            );
            *thing = Thing::Trinket {};
        }
    }
}
