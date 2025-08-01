use piston::WindowSettings;
use piston_window::{clear, rectangle, PistonWindow};

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn create_window() {

  let mut window: PistonWindow = WindowSettings::new("Snake Game!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            clear([1.5, 1.0, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                [10.0, 10.0, 100.0, 100.0], // rectangle
                c.transform, g);

                rectangle(GREEN,
                    [50.0, 50.0, 50.0, 50.0], // rectangle
                    c.transform, g);
        });
    }
    
}