use graphics::math::Vec2d;
use rand::{RngExt, rng};

use crate::world::World;

pub struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Particle {
        let mut rng = rng();
        let x = rng.random_range(0.0..=world.width);
        let y = world.height;
        let x_velocity = 0.0;
        let y_velocity = rng.random_range(-2.0..0.0);
        let x_acceleration = 0.0;
        let y_acceleration = rng.random_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),
            color: [1.0, 1.0, 1.0, 0.99],
        }
    }
}
