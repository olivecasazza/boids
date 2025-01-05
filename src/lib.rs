use glam::Vec2;
use rand::Rng;
use wasm_bindgen::prelude::*;

// Constants for flocking behavior
const ALIGNMENT_RADIUS: f32 = 50.0;
const COHESION_RADIUS: f32 = 50.0;
const SEPARATION_RADIUS: f32 = 25.0;
const MAX_SPEED: f32 = 4.0;
const MAX_FORCE: f32 = 0.1;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

#[wasm_bindgen]
impl Boid {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 2.0;

        Self {
            position: Vec2::new(x, y),
            velocity,
            acceleration: Vec2::ZERO,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.position.x
    }

    pub fn get_y(&self) -> f32 {
        self.position.y
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    fn align(&self, boids: &[Boid]) -> Vec2 {
        let mut steering = Vec2::ZERO;
        let mut total = 0;

        for other in boids {
            let d = self.position.distance(other.position);
            if d > 0.0 && d < ALIGNMENT_RADIUS {
                steering += other.velocity;
                total += 1;
            }
        }

        if total > 0 {
            steering /= total as f32;
            steering = steering.normalize_or_zero() * MAX_SPEED;
            steering -= self.velocity;
            steering = steering.clamp_length_max(MAX_FORCE);
        }

        steering
    }

    fn cohesion(&self, boids: &[Boid]) -> Vec2 {
        let mut steering = Vec2::ZERO;
        let mut total = 0;

        for other in boids {
            let d = self.position.distance(other.position);
            if d > 0.0 && d < COHESION_RADIUS {
                steering += other.position;
                total += 1;
            }
        }

        if total > 0 {
            steering /= total as f32;
            steering -= self.position;
            steering = steering.normalize_or_zero() * MAX_SPEED;
            steering -= self.velocity;
            steering = steering.clamp_length_max(MAX_FORCE);
        }

        steering
    }

    fn separation(&self, boids: &[Boid]) -> Vec2 {
        let mut steering = Vec2::ZERO;
        let mut total = 0;

        for other in boids {
            let d = self.position.distance(other.position);
            if d > 0.0 && d < SEPARATION_RADIUS {
                let mut diff = self.position - other.position;
                diff /= d * d;
                steering += diff;
                total += 1;
            }
        }

        if total > 0 {
            steering /= total as f32;
            steering = steering.normalize_or_zero() * MAX_SPEED;
            steering -= self.velocity;
            steering = steering.clamp_length_max(MAX_FORCE);
        }

        steering
    }
}

#[wasm_bindgen]
pub struct Flock {
    boids: Vec<Boid>,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl Flock {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let boids = (0..count)
            .map(|_| {
                Boid::new(
                    rng.gen::<f32>() * width,
                    rng.gen::<f32>() * height,
                )
            })
            .collect();

        Self {
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let boids_clone = self.boids.clone();
        
        for boid in &mut self.boids {
            let alignment = boid.align(&boids_clone);
            let cohesion = boid.cohesion(&boids_clone);
            let separation = boid.separation(&boids_clone);

            boid.apply_force(alignment);
            boid.apply_force(cohesion);
            boid.apply_force(separation);

            // Update physics
            boid.velocity += boid.acceleration;
            boid.velocity = boid.velocity.clamp_length_max(MAX_SPEED);
            boid.position += boid.velocity;
            boid.acceleration = Vec2::ZERO;

            // Wrap around screen
            boid.position.x = (boid.position.x + self.width) % self.width;
            boid.position.y = (boid.position.y + self.height) % self.height;
        }
    }

    pub fn get_boids(&self) -> js_sys::Float32Array {
        let mut positions = Vec::with_capacity(self.boids.len() * 2);
        for boid in &self.boids {
            positions.push(boid.position.x);
            positions.push(boid.position.y);
        }
        js_sys::Float32Array::from(&positions[..])
    }

    pub fn len(&self) -> usize {
        self.boids.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_creation() {
        let boid = Boid::new(100.0, 100.0);
        assert_eq!(boid.position.x, 100.0);
        assert_eq!(boid.position.y, 100.0);
    }

    #[test]
    fn test_flock_creation() {
        let flock = Flock::new(800.0, 600.0, 10);
        assert_eq!(flock.len(), 10);
    }
}
