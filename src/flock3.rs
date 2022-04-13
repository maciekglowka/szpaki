use crate::{utils, vector};
use rand::Rng;

// const MAX_DZ: f32 = 0.5;
// const MAX_WIND: f32 = 3.0;
// const MAX_ANGLE_CHANGE: f32 = 0.125 * std::f32::consts::PI;

fn random_vector3(min: f32, max: f32) -> vector::Vector3 {
    let mut rng = rand::thread_rng();
    vector::Vector3{
        x: rng.gen_range(min..max),
        y: rng.gen_range(min..max),
        z: rng.gen_range(min..max)
    }
}

fn vertical_clamp(v: vector::Vector3, max_dz: f32) -> vector::Vector3 {
    let l = v.len();
    if l == 0.0 {return v};
    if v.z / l > max_dz {
        return vector::Vector3::new(v.x, v.y, l * max_dz)
    } else if v.z / l < -max_dz {
        return vector::Vector3::new(v.x, v.y, -l * max_dz)
    };
    v
}

pub struct Flock {
    pub boids: Vec<Boid>,
    wind: vector::Vector3,
    center: vector::Vector3
}

impl Flock {
    pub fn new(config: &utils::Config) -> Flock {
        let mut boids = Vec::with_capacity(config.count);

        for _ in 0..config.count {
            boids.push(
                Boid::new(
                    random_vector3(0.0, config.size as f32),
                    random_vector3(-1.0, 1.0)
                )
            );
        }
        Flock{
            boids: boids,
            wind: random_vector3(-1.0, 1.0),
            center: vector::Vector3::new(config.size as f32/2.0, config.size as f32/2.0, config.size as f32/2.0)
        }
    }
    pub fn update(&mut self, config: &utils::Config) {
        let mut new_boids = Vec::with_capacity(self.boids.len());

        self.wind += random_vector3(-0.5, 0.5);
        vertical_clamp(self.wind, config.delta_z_max);
        self.wind.clamp(config.wind);

        for b in &self.boids {
            let nb = b.get_updated(&self.boids, self.wind, self.center, config);
            new_boids.push(nb);
        };
        self.boids = new_boids;
    }
}

#[derive(PartialEq)]
pub struct Boid {
    pub position: vector::Vector3,
    pub velocity: vector::Vector3
}

impl Boid {
    pub fn new(position: vector::Vector3, velocity: vector::Vector3) -> Boid {
        Boid{position: position, velocity: velocity}
    }
    pub fn get_updated(&self, others: &Vec<Boid>, wind: vector::Vector3, center: vector::Vector3, config: &utils::Config) -> Boid {
        let mut neighbours = Vec::new();

        for b in others {
            if b == self {continue};
            if b.position.dist(self.position) < config.neighbourhood {neighbours.push(b)}
        }
        
        let va = self.alignment(&neighbours);
        let vc = self.cohesion(&neighbours);
        let vr = self.roosting(center);
        let vs = self.separation(&neighbours, config);
        // let mut v = self.velocity + va + 10.0 * vs + 0.1 * vr + 0.2 * vc;
        let mut v = self.velocity +
            config.alignment * va +
            config.cohesion * vc +
            config.roosting * vr +
            config.separation * vs;

        v.clamp(config.velocity);
        v = vertical_clamp(v, config.delta_z_max);

        // let a = self.velocity.angle(v);
        // if a > MAX_ANGLE_CHANGE {
        //     v.rotate(MAX_ANGLE_CHANGE-a);
        // } else if a < -MAX_ANGLE_CHANGE {
        //     v.rotate(-MAX_ANGLE_CHANGE-a)
        // }

        v += wind;
        Boid::new(self.position + v, v)
    }

    fn roosting(&self, center: vector::Vector3) -> vector::Vector3 {
        (center - self.position).normalized()
    }
    
    fn cohesion(&self, others: &Vec<&Boid>) -> vector::Vector3 {
        let center = self.get_neighbourhood_center(&others);
        (center - self.position).normalized()
    }
    fn separation(&self, others: &Vec<&Boid>, config: &utils::Config) -> vector::Vector3 {
        let mut v = vector::Vector3::new(0.0, 0.0, 0.0);

        for b in others {
            if b.position.dist(self.position) > config.separation_range {
                continue;
            }
            let mut d = b.position - self.position;
            if d.x != 0.0 {d.x = config.separation_range/d.x}
            if d.y != 0.0 {d.y = config.separation_range/d.y}
            v -= d;
        }
        v
    }
    fn alignment(&self, others: &Vec<&Boid>) -> vector::Vector3 {
        let mut v = vector::Vector3::new(0.0, 0.0, 0.0);
        for b in others {
            v += b.velocity;
        }
        v.normalized()
    }
    fn get_neighbourhood_center(&self, others: &Vec<&Boid>) -> vector::Vector3 {
        if others.len() == 0 { return self.position;}
        let mut c = vector::Vector3::new(0.0, 0.0, 0.0);
        for b in others {
            c += b.position;
        }
        c / others.len() as f32
    }
}