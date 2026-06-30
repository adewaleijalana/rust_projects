use rand::rngs::ThreadRng;

use crate::particle::Particle;

pub struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}
