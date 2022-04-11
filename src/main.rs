use minifb;
use std::env;

mod flock;
mod vector;

const SIZE: usize = 640;

fn draw_boid(buffer: &mut Vec<u32>, boid: &flock::Boid) {
    if boid.position.x<2.0 || boid.position.y<2.0 || boid.position.x>=SIZE as f32 - 2.0 || boid.position.y>=SIZE as f32 - 2.0 {return;}
    buffer[boid.position.y as usize * SIZE + boid.position.x as usize] = u32::MAX;
    let mut d = boid.velocity;
    d.normalize();
    let head = boid.position + d;
    buffer[head.y as usize * SIZE + head.x as usize] = u32::MAX;
}

fn main() {
    let mut count = 1500;
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            match &args[1].parse() {
                Ok(n) => {count = *n;},
                Err(_) => {}
            };
        },
        _ => {}
    }

    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let mut window = minifb::Window::new(
        "BOIDS", SIZE, SIZE, minifb::WindowOptions::default()
    ).expect("Window creation failed!");

    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    let mut flock = flock::Flock::new(count, SIZE as f32);

    while window.is_open() {
        buffer = vec![0; SIZE * SIZE];

        for b in &flock.boids {
            draw_boid(&mut buffer, &b);
        }
        flock.update();
        window.update_with_buffer(&buffer, SIZE, SIZE).expect("Window update failed!");
    }    
}