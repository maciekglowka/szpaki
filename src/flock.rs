use crate::vector;
use rand::Rng;

const MAX_V: f32 = 4.0;
const NEIGHBOURHOOD: f32 = 30.0;
const SEPARATION_DIST: f32 = 5.0;
const MAX_WIND: f32 = 3.0;
const MAX_ANGLE_CHANGE: f32 = 0.125 * std::f32::consts::PI;

pub struct Flock {
    pub boids: Vec<Boid>,
    wind: vector::Vector2,
    center: vector::Vector2
}

impl Flock {
    pub fn new(count: usize, area_size: f32) -> Flock {
        let mut boids = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            boids.push(
                Boid::new(
                    vector::Vector2::new(rng.gen_range(0.0..area_size),rng.gen_range(0.0..area_size)),
                    vector::Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                )
            );
        }
        let mut rng = rand::thread_rng();
        Flock{
            boids: boids,
            wind: vector::Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            center: vector::Vector2::new(area_size/2.0, area_size/2.0)
        }
    }
    pub fn update(&mut self) {
        let mut new_boids = Vec::with_capacity(self.boids.len());

        let mut rng = rand::thread_rng();
        self.wind += vector::Vector2::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
        self.wind.clamp(MAX_WIND);

        for b in &self.boids {
            let nb = b.get_updated(&self.boids, self.wind, self.center);
            new_boids.push(nb);
        };
        self.boids = new_boids;
    }
}

#[derive(PartialEq)]
pub struct Boid {
    pub position: vector::Vector2,
    pub velocity: vector::Vector2
}

impl Boid {
    pub fn new(position: vector::Vector2, velocity: vector::Vector2) -> Boid {
        Boid{position: position, velocity: velocity}
    }
    pub fn get_updated(&self, others: &Vec<Boid>, wind: vector::Vector2, center: vector::Vector2) -> Boid {
        let mut neighbours = Vec::new();

        for b in others {
            if b == self {continue};
            if b.position.dist(self.position) < NEIGHBOURHOOD {neighbours.push(b)}
        }
        
        let vc = self.cohesion(&neighbours);
        let va = self.alignment(&neighbours);
        let vs = self.separation(&neighbours);
        let vr = self.roosting(center);
        let mut v = self.velocity + va + vs * 10.0 + vr * 0.1 + vc * 0.2;
        // let mut v = self.velocity + vc + vs;
        v.clamp(MAX_V);

        let a = self.velocity.angle(v);
        if a > MAX_ANGLE_CHANGE {
            v.rotate(MAX_ANGLE_CHANGE-a);
        } else if a < -MAX_ANGLE_CHANGE {
            v.rotate(-MAX_ANGLE_CHANGE-a)
        }

        // v+=wind;
        Boid::new(self.position + v, v)
    }

    fn roosting(&self, center: vector::Vector2) -> vector::Vector2 {
        center - self.position
    }
    
    fn cohesion(&self, others: &Vec<&Boid>) -> vector::Vector2 {
        let center = self.get_center(&others);
        center - self.position
    }
    fn separation(&self, others: &Vec<&Boid>) -> vector::Vector2 {
        let mut v = vector::Vector2::new(0.0, 0.0);

        for b in others {
            if b.position.dist(self.position) > SEPARATION_DIST {
                continue;
            }
            let mut d = b.position - self.position;
            if d.x != 0.0 {d.x = SEPARATION_DIST/d.x}
            if d.y != 0.0 {d.y = SEPARATION_DIST/d.y}
            v -= d;
        }
        v
    }
    fn alignment(&self, others: &Vec<&Boid>) -> vector::Vector2 {
        let mut v = vector::Vector2::new(0.0, 0.0);
        for b in others {
            v += b.velocity;
        }
        v
    }
    fn get_center(&self, others: &Vec<&Boid>) -> vector::Vector2 {
        let mut c = vector::Vector2::new(0.0, 0.0);
        for b in others {
            c += b.position;
        }
        c / others.len() as f32
    }
}