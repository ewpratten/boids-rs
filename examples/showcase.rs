use boids::{Boid2D, Flock};
use cgmath::Vector2;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Boids showcase")
        .msaa_4x()
        .vsync()
        .build();

    // Create a flock of boids
    let mut flock = Flock {
        boids: (0..10000)
            .map(|_| Boid2D::new(Vector2::new(400.0, 300.0)))
            .collect(),
        ..Default::default()
    };

    // Set up profiling
    let _puffin_server =
        puffin_http::Server::new(&format!("localhost:{}", puffin_http::DEFAULT_PORT)).unwrap();
    puffin::set_scopes_on(true);

    while !rl.window_should_close() {
        puffin::profile_scope!("frame");
        puffin::GlobalProfiler::lock().new_frame();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Render every boid
        for boid in &mut flock.boids {
            d.draw_circle(
                boid.position.x as i32,
                boid.position.y as i32,
                5.0,
                Color::BLACK,
            );
        }

        // Update the flock
        flock.update();

        // Teleport any boid that is out of bounds
        for boid in &mut flock.boids {
            if boid.position.x < 0.0 {
                boid.position.x = 800.0;
            } else if boid.position.x > 800.0 {
                boid.position.x = 0.0;
            }
            if boid.position.y < 0.0 {
                boid.position.y = 600.0;
            } else if boid.position.y > 600.0 {
                boid.position.y = 0.0;
            }
        }
    }
}
