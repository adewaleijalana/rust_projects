use rand::rngs::ThreadRng;

use crate::particle::Particle;

pub struct World {
    pub current_turn: u64,
    pub particles: Vec<Box<Particle>>,
    pub height: f64,
    pub width: f64,
    pub rng: ThreadRng,
}
