use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Species {
    // Behavior radii
    pub(crate) alignment_radius: f32,
    pub(crate) cohesion_radius: f32,
    pub(crate) separation_radius: f32,
    
    // Force multipliers
    pub(crate) alignment_force: f32,
    pub(crate) cohesion_force: f32,
    pub(crate) separation_force: f32,
    
    // Movement constraints
    pub(crate) max_speed: f32,
    pub(crate) max_force: f32,
    
    // Visual properties
    pub(crate) size: f32,
    pub(crate) color: u32, // RGBA color as a single u32
    
    // Spawn properties
    pub(crate) spawn_rate_multiplier: f32,

    // Inter-species interaction multipliers
    pub(crate) other_species_alignment_multiplier: f32,
    pub(crate) other_species_cohesion_multiplier: f32,
    pub(crate) other_species_separation_multiplier: f32,
}

#[wasm_bindgen]
impl Species {
    #[wasm_bindgen(constructor)]
    pub fn new(
        alignment_radius: f32,
        cohesion_radius: f32,
        separation_radius: f32,
        alignment_force: f32,
        cohesion_force: f32,
        separation_force: f32,
        max_speed: f32,
        max_force: f32,
        size: f32,
        color: u32,
        spawn_rate_multiplier: f32,
        other_species_alignment_multiplier: f32,
        other_species_cohesion_multiplier: f32,
        other_species_separation_multiplier: f32,
    ) -> Self {
        Self {
            alignment_radius,
            cohesion_radius,
            separation_radius,
            alignment_force,
            cohesion_force,
            separation_force,
            max_speed,
            max_force,
            size,
            color,
            spawn_rate_multiplier,
            other_species_alignment_multiplier,
            other_species_cohesion_multiplier,
            other_species_separation_multiplier,
        }
    }

    pub fn default() -> Self {
        Self {
            alignment_radius: 50.0,
            cohesion_radius: 50.0,
            separation_radius: 25.0,
            alignment_force: 1.0,
            cohesion_force: 1.0,
            separation_force: 1.0,
            max_speed: 4.0,
            max_force: 0.1,
            size: 5.0,
            color: 0xFF0000FF, // Red color
            spawn_rate_multiplier: 1.0,
            other_species_alignment_multiplier: 0.5,
            other_species_cohesion_multiplier: 0.5,
            other_species_separation_multiplier: 1.5,
        }
    }

    #[wasm_bindgen]
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate random color components
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let color = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF;

        Self {
            // Radii between 25 and 100
            alignment_radius: rng.gen_range(25.0..100.0),
            cohesion_radius: rng.gen_range(25.0..100.0),
            separation_radius: rng.gen_range(15.0..50.0),
            
            // Forces between 0.5 and 2.0
            alignment_force: rng.gen_range(0.5..2.0),
            cohesion_force: rng.gen_range(0.5..2.0),
            separation_force: rng.gen_range(0.5..2.0),
            
            // Speed between 2.0 and 6.0
            max_speed: rng.gen_range(2.0..6.0),
            // Force between 0.05 and 0.2
            max_force: rng.gen_range(0.05..0.2),
            
            // Size between 3.0 and 8.0
            size: rng.gen_range(3.0..8.0),
            // Random RGB color with full opacity
            color,
            
            // Spawn rate between 0.5 and 2.0
            spawn_rate_multiplier: rng.gen_range(0.5..2.0),
            
            // Random inter-species interaction multipliers
            other_species_alignment_multiplier: rng.gen_range(0.2..1.0),
            other_species_cohesion_multiplier: rng.gen_range(0.2..1.0),
            other_species_separation_multiplier: rng.gen_range(1.0..2.0),
        }
    }
}
