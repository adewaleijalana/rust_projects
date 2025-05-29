use piston::WindowSettings;
use piston_window::{clear, PistonWindow};

pub fn create_window() {

  let mut window: PistonWindow = WindowSettings::new("Snake Game!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _d| {
            clear([1.5, 1.0, 0.5, 1.0], g);
        });
    }
    
}