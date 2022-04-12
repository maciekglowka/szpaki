use clap::Parser;
use minifb;
use std::env;

mod flock3;
mod utils;
mod vector;

fn draw_boid(buffer: &mut Vec<u32>, config: &utils::Config, boid: &flock3::Boid) {
    // if boid.position.x<2.0 || boid.position.y<2.0 || boid.position.x>=SIZE as f32 - 2.0 || boid.position.y>=SIZE as f32 - 2.0 {return;}
    // buffer[boid.position.y as usize * SIZE + boid.position.x as usize] = u32::MAX;
    // let mut d = boid.velocity;
    // d.normalize();
    // let head = boid.position + d;
    // buffer[head.y as usize * SIZE + head.x as usize] = u32::MAX;

    if  boid.position.x<2.0 || boid.position.z<2.0 || boid.position.y<0.0 ||
        boid.position.x>=config.size as f32 - 2.0 || boid.position.z>=config.size as f32 - 2.0
        {return;}

    let gray = boid.position.y * (255.0 / config.size as f32);
    let gray = gray as u32;
    let col = (gray << 16) + (gray << 8) + gray;

    buffer[boid.position.z as usize * config.size + boid.position.x as usize] = col;
}

fn draw_boid_top(buffer: &mut Vec<u32>, config: &utils::Config, boid: &flock3::Boid) {
    if  boid.position.x<2.0 || boid.position.y<2.0 || boid.position.z<0.0 ||
        boid.position.x>=config.size as f32 - 2.0 || boid.position.y>=config.size as f32 - 2.0
        {return;}

    let gray = boid.position.z * (255.0 / config.size as f32);
    let gray = gray as u32;
    let col = (gray << 16) + (gray << 8) + gray;

    buffer[boid.position.y as usize * config.size + boid.position.x as usize] = col;
}

fn main() {
    let config = utils::Config::parse();

    let mut buffer: Vec<u32> = vec![0; config.size * config.size];

    let mut window = minifb::Window::new(
        "BOIDS", config.size, config.size, minifb::WindowOptions::default()
    ).expect("Window creation failed!");

    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    let mut flock = flock3::Flock::new(&config);

    while window.is_open() {
        buffer = vec![0; config.size * config.size];

        for b in &flock.boids {
            draw_boid_top(&mut buffer, &config, &b);
        }
        flock.update(&config);
        window.update_with_buffer(&buffer, config.size, config.size).expect("Window update failed!");
    }    
}