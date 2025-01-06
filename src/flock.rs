use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use glam::Vec2;
use rand::Rng;

use crate::boid::Boid;
use crate::species::Species;
use crate::quadtree::{QuadTree, Boundary};

#[wasm_bindgen]
pub struct Flock {
    boids: Vec<Boid>,
    species: HashMap<usize, Species>,
    next_species_id: usize,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl Flock {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, count: usize) -> Self {
        let mut flock = Self {
            boids: Vec::new(),
            species: HashMap::new(),
            next_species_id: 0,
            width,
            height,
        };

        let default_species_id = flock.add_species(Species::default());
        flock.add_boids(count, default_species_id);

        flock
    }

    pub fn add_species(&mut self, species: Species) -> usize {
        let id = self.next_species_id;
        self.species.insert(id, species);
        self.next_species_id += 1;
        id
    }

    pub fn update_species(&mut self, species_id: usize, species: Species) -> bool {
        if self.species.contains_key(&species_id) {
            self.species.insert(species_id, species);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn add_boids(&mut self, count: usize, species_id: usize) {
        if !self.species.contains_key(&species_id) {
            return;
        }

        let mut rng = rand::thread_rng();
        let spawn_multiplier = self.species[&species_id].spawn_rate_multiplier;
        let adjusted_count = (count as f32 * spawn_multiplier).round() as usize;
        
        let new_boids: Vec<Boid> = (0..adjusted_count)
            .map(|_| {
                Boid::new_with_species_id(
                    rng.gen::<f32>() * self.width,
                    rng.gen::<f32>() * self.height,
                    species_id,
                )
            })
            .collect();
        self.boids.extend(new_boids);
    }

    #[wasm_bindgen]
    pub fn add_boid_at(&mut self, x: f32, y: f32, species_id: usize) {
        if !self.species.contains_key(&species_id) {
            return;
        }
        self.boids.push(Boid::new_with_species_id(x, y, species_id));
    }

    #[wasm_bindgen]
    pub fn remove_random_boid(&mut self) {
        if !self.boids.is_empty() {
            let index = rand::thread_rng().gen_range(0..self.boids.len());
            self.boids.swap_remove(index);
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        let boundary = Boundary {
            x: self.width / 2.0,
            y: self.height / 2.0,
            width: self.width,
            height: self.height,
        };
        
        // Pre-calculate quadtree capacity based on flock size
        let capacity = (self.boids.len() as f32).sqrt().ceil() as usize;
        let mut quadtree = QuadTree::new(boundary, capacity);

        // Build quadtree
        for (i, boid) in self.boids.iter().enumerate() {
            quadtree.insert(boid.position, i);
        }

        // Pre-allocate nearby vector with estimated capacity
        let mut nearby = Vec::with_capacity(32);
        
        // Calculate forces for all boids first
        let mut forces = Vec::with_capacity(self.boids.len());
        for boid in &self.boids {
            let species = &self.species[&boid.species_id];
            
            let query_boundary = Boundary {
                x: boid.position.x,
                y: boid.position.y,
                width: species.alignment_radius * 2.0,
                height: species.alignment_radius * 2.0,
            };
            
            nearby.clear();
            quadtree.query(&query_boundary, &mut nearby);

            // Calculate forces
            forces.push((
                boid.calculate_forces(&nearby, &self.boids, species),
                species.max_speed
            ));
        }

        // Update positions with calculated forces
        for (i, boid) in self.boids.iter_mut().enumerate() {
            let (force, max_speed) = forces[i];
            
            // Apply force and update position
            boid.apply_force(force);
            boid.velocity += boid.acceleration;
            boid.velocity = boid.velocity.clamp_length_max(max_speed);
            boid.position += boid.velocity;
            boid.acceleration = Vec2::ZERO;

            // Wrap around screen edges
            boid.position.x = (boid.position.x + self.width) % self.width;
            boid.position.y = (boid.position.y + self.height) % self.height;
        }
    }

    #[wasm_bindgen]
    pub fn get_boids(&self) -> js_sys::Float32Array {
        // Pre-allocate with exact capacity
        let capacity = self.boids.len() * 7;
        let mut data = Vec::with_capacity(capacity);
        
        // Process boids in chunks for better cache utilization
        for chunk in self.boids.chunks(64) {
            for boid in chunk {
                let species = &self.species[&boid.species_id];
                let color = species.color;
                
                data.extend_from_slice(&[
                    boid.position.x,
                    boid.position.y,
                    species.size,
                    ((color >> 24) & 0xFF) as f32 / 255.0,
                    ((color >> 16) & 0xFF) as f32 / 255.0,
                    ((color >> 8) & 0xFF) as f32 / 255.0,
                    (color & 0xFF) as f32 / 255.0,
                ]);
            }
        }
        js_sys::Float32Array::from(&data[..])
    }

    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.boids.len()
    }

    #[wasm_bindgen]
    pub fn set_size(&mut self, target_size: usize) {
        let current_size = self.boids.len();
        if target_size > current_size {
            // Add more boids
            self.add_boids(target_size - current_size, 0);
        } else if target_size < current_size {
            // Remove excess boids
            self.boids.truncate(target_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_creation() {
        let mut flock = Flock::new(800.0, 600.0, 0);
        let species_id = flock.add_species(Species::default());
        let boid = Boid::new_with_species_id(100.0, 100.0, species_id);
        assert_eq!(boid.position.x, 100.0);
        assert_eq!(boid.position.y, 100.0);
    }

    #[test]
    fn test_flock_creation() {
        let flock = Flock::new(800.0, 600.0, 10);
        assert_eq!(flock.len(), 10);
    }

    #[test]
    fn test_species_creation() {
        let species = Species::new(
            40.0, // alignment_radius
            45.0, // cohesion_radius
            20.0, // separation_radius
            1.2,  // alignment_force
            0.8,  // cohesion_force
            1.5,  // separation_force
            3.0,  // max_speed
            0.2,  // max_force
            6.0,  // size
            0xFF0000FF, // color (red)
            1.2,  // spawn_rate_multiplier
        );
        
        assert_eq!(species.alignment_radius, 40.0);
        assert_eq!(species.cohesion_radius, 45.0);
        assert_eq!(species.separation_radius, 20.0);
        assert_eq!(species.alignment_force, 1.2);
        assert_eq!(species.cohesion_force, 0.8);
        assert_eq!(species.separation_force, 1.5);
        assert_eq!(species.max_speed, 3.0);
        assert_eq!(species.max_force, 0.2);
        assert_eq!(species.size, 6.0);
        assert_eq!(species.color, 0xFF0000FF);
        assert_eq!(species.spawn_rate_multiplier, 1.2);
    }

    #[test]
    fn test_multiple_species() {
        let mut flock = Flock::new(800.0, 600.0, 10);
        let fast_species = Species::new(
            50.0, 50.0, 25.0,  // radii
            1.0, 1.0, 1.0,     // forces
            6.0, 0.2,          // speed/force
            8.0,               // size
            0x00FF00FF,        // color (green)
            1.5,               // spawn multiplier
        );
        let species_id = flock.add_species(fast_species);
        flock.add_boids(5, species_id);
        
        // With spawn_rate_multiplier of 1.5, 5 requested boids should become 7 or 8
        assert!(flock.len() >= 17 && flock.len() <= 18);
    }

    #[test]
    fn test_species_update() {
        let mut flock = Flock::new(800.0, 600.0, 10);
        let species_id = flock.add_species(Species::default());
        let updated_species = Species::new(
            60.0, 60.0, 30.0,  // radii
            1.2, 0.8, 1.5,     // forces
            5.0, 0.3,          // speed/force
            7.0,               // size
            0x0000FFFF,        // color (blue)
            1.0,               // spawn multiplier
        );
        assert!(flock.update_species(species_id, updated_species));
    }
}
