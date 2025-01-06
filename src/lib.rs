mod quadtree;
mod species;
mod boid;
mod flock;

pub use species::Species;
pub use boid::Boid;
pub use flock::Flock;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_creation() {
        let mut flock = Flock::new(800.0, 600.0, 0);
        let species_id = flock.add_species(Species::default());
        let boid = boid::Boid::new_with_species_id(100.0, 100.0, species_id);
        assert_eq!(boid.get_x(), 100.0);
        assert_eq!(boid.get_y(), 100.0);
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
        
        let mut flock = Flock::new(800.0, 600.0, 0);
        let species_id = flock.add_species(species);
        flock.add_boids(5, species_id);
        assert_eq!(flock.len(), 6);
    }
}
