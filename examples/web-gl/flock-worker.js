import init, { Flock, Species } from '/pkg/wasm_flock.js';

let flock;
const MAX_BOIDS = 3000;
const BOIDS_PER_SPECIES = 50;

async function initializeFlock(width, height) {
    await init();
    
    // Create flock with default species
    flock = new Flock(width, height, BOIDS_PER_SPECIES);
    
    // Add two random species
    const randomSpecies1 = Species.random();
    const randomSpecies2 = Species.random();
    
    const species1Id = flock.add_species(randomSpecies1);
    const species2Id = flock.add_species(randomSpecies2);
    
    // Add boids for each random species
    flock.add_boids(BOIDS_PER_SPECIES, species1Id);
    flock.add_boids(BOIDS_PER_SPECIES, species2Id);

    return { species1Id, species2Id };
}

self.onmessage = async function(e) {
    const { type, data } = e.data;

    switch (type) {
        case 'init':
            const { width, height } = data;
            const speciesIds = await initializeFlock(width, height);
            self.postMessage({ type: 'initialized', data: speciesIds });
            break;

        case 'update':
            if(!flock) break;
            const updateStart = performance.now();
            flock.update();
            const positions = flock.get_boids();
            const updateTime = performance.now() - updateStart;
            
            self.postMessage({ 
                type: 'positions', 
                data: { 
                    positions,
                    updateTime
                }
            });
            break;

        case 'addBoidAt':
            const { x, y, speciesId } = data;
            if (flock.len() >= MAX_BOIDS) {
                flock.remove_random_boid();
            }
            flock.add_boid_at(x, y, speciesId);
            self.postMessage({ 
                type: 'flockSize', 
                data: flock.len() 
            });
            break;

        case 'setSize':
            const { size } = data;
            flock.set_size(size);
            self.postMessage({ 
                type: 'flockSize', 
                data: flock.len() 
            });
            break;
    }
};
