use rand::{rng, seq::IndexedRandom};

use crate::{dwarf::Dwarf, elf::Elf, enchanter::Enchanter, human::Human, thing::Thing};

pub mod dwarf;
pub mod elf;
pub mod enchanter;
pub mod human;
pub mod thing;


pub fn run() {

  let mut it = Thing::Sword;

  let dwarf = Dwarf {};

  let elf = Elf {};

  let human = Human {};

  let party: Vec<&dyn Enchanter> = vec![&dwarf, &elf, &human];

  let spell_caster = party.choose(&mut rng()).unwrap();

  spell_caster.enchant(&mut it);
    
}
