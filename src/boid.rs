use glam::Vec2;
use wasm_bindgen::prelude::*;
use crate::species::Species;
use rand::Rng;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Boid {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
    pub(crate) acceleration: Vec2,
    pub(crate) species_id: usize,
}

#[wasm_bindgen]
impl Boid {
    pub(crate) fn new_with_species_id(x: f32, y: f32, species_id: usize) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 2.0;

        Self {
            position: Vec2::new(x, y),
            velocity,
            acceleration: Vec2::ZERO,
            species_id,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.position.x
    }

    pub fn get_y(&self) -> f32 {
        self.position.y
    }

    pub(crate) fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    // Combined force calculation for better performance
    pub(crate) fn calculate_forces(&self, nearby_boids: &[(Vec2, usize)], all_boids: &[Boid], species: &Species) -> Vec2 {
        if nearby_boids.is_empty() {
            return Vec2::ZERO;
        }

        // Split nearby boids into same species and other species
        let (same_species, other_species): (Vec<_>, Vec<_>) = nearby_boids.iter()
            .partition(|(_, index)| all_boids[*index].species_id == self.species_id);

        let mut total_force = Vec2::ZERO;

        // Process same species interactions
        if !same_species.is_empty() {
            let count = same_species.len() as f32;

            // Alignment - same species
            let avg_velocity: Vec2 = same_species.iter()
                .map(|&(_, index)| all_boids[index].velocity)
                .sum::<Vec2>() / count;
            
            let alignment = (avg_velocity.normalize_or_zero() * species.max_speed - self.velocity)
                .clamp_length_max(species.max_force) * species.alignment_force;

            // Cohesion - same species
            let center_of_mass: Vec2 = same_species.iter()
                .map(|&(pos, _)| pos)
                .sum::<Vec2>() / count;
            
            let cohesion = ((center_of_mass - self.position).normalize_or_zero() * species.max_speed - self.velocity)
                .clamp_length_max(species.max_force) * species.cohesion_force;

            // Separation - same species
            let mut separation = Vec2::ZERO;
            for &(pos, _) in &same_species {
                let offset = self.position - pos;
                let dist_sq = offset.length_squared();
                if dist_sq > 0.0 {
                    separation += offset / dist_sq;
                }
            }
            
            if !separation.is_nan() {
                separation = (separation.normalize_or_zero() * species.max_speed - self.velocity)
                    .clamp_length_max(species.max_force) * species.separation_force;
            }

            total_force += alignment + cohesion + separation;
        }

        // Process other species interactions
        if !other_species.is_empty() {
            let count = other_species.len() as f32;

            // Alignment - other species
            let avg_velocity: Vec2 = other_species.iter()
                .map(|&(_, index)| all_boids[index].velocity)
                .sum::<Vec2>() / count;
            
            let alignment = (avg_velocity.normalize_or_zero() * species.max_speed - self.velocity)
                .clamp_length_max(species.max_force) * species.alignment_force * species.other_species_alignment_multiplier;

            // Cohesion - other species
            let center_of_mass: Vec2 = other_species.iter()
                .map(|&(pos, _)| pos)
                .sum::<Vec2>() / count;
            
            let cohesion = ((center_of_mass - self.position).normalize_or_zero() * species.max_speed - self.velocity)
                .clamp_length_max(species.max_force) * species.cohesion_force * species.other_species_cohesion_multiplier;

            // Separation - other species
            let mut separation = Vec2::ZERO;
            for &(pos, _) in &other_species {
                let offset = self.position - pos;
                let dist_sq = offset.length_squared();
                if dist_sq > 0.0 {
                    separation += offset / dist_sq;
                }
            }
            
            if !separation.is_nan() {
                separation = (separation.normalize_or_zero() * species.max_speed - self.velocity)
                    .clamp_length_max(species.max_force) * species.separation_force * species.other_species_separation_multiplier;
            }

            total_force += alignment + cohesion + separation;
        }

        total_force
    }

    // Keep these methods for compatibility but make them use the combined calculation
    // pub(crate) fn align(&self, nearby_boids: &[(Vec2, usize)], all_boids: &[Boid], species: &Species) -> Vec2 {
    //     self.calculate_forces(nearby_boids, all_boids, species) * 0.33
    // }

    // pub(crate) fn cohesion(&self, nearby_boids: &[(Vec2, usize)], all_boids: &[Boid], species: &Species) -> Vec2 {
    //     self.calculate_forces(nearby_boids, all_boids, species) * 0.33
    // }

    // pub(crate) fn separation(&self, nearby_boids: &[(Vec2, usize)], all_boids: &[Boid], species: &Species) -> Vec2 {
    //     self.calculate_forces(nearby_boids, all_boids, species) * 0.34
    // }
}
