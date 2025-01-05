# WASM Flock

A Rust library for flock/boid simulation with WebAssembly support. This library implements Craig Reynolds' Boids algorithm for simulating flocking behavior, and can be used in both native Rust applications and web browsers via WebAssembly.

## Features

- Configurable flocking behavior (alignment, cohesion, separation)
- WebAssembly support for browser integration
- Efficient vector mathematics using `glam`
- Screen wrapping for infinite space simulation
- Customizable flock parameters

## Usage

### In Rust

```rust
use wasm_flock::Flock;

// Create a new flock with canvas dimensions and number of boids
let mut flock = Flock::new(800.0, 600.0, 50);

// Update the simulation each frame
flock.update();
```

### In WebAssembly (JavaScript)

```javascript
import init, { Flock } from 'wasm-flock';

async function run() {
    // Initialize the WASM module
    await init();

    // Create a new flock
    const flock = new Flock(800, 600, 50);

    function animate() {
        // Update the simulation
        flock.update();

        // Get boid positions as Float32Array
        const positions = flock.get_boids();
        
        // Draw boids using positions array
        // positions[i] = x coordinate
        // positions[i + 1] = y coordinate
        
        requestAnimationFrame(animate);
    }

    animate();
}

run();
```

## Development

This project uses [mise](https://mise.jdx.dev/) for task management. Here are the available tasks:

### Setup

```bash
# Install dependencies
mise run install-deps

# Initial setup
mise run setup
```

### Building

```bash
# Native build
mise run build

# WebAssembly build
mise run build-web

# Platform-specific builds
mise run build-linux
mise run build-windows
mise run build-macos
```

### Testing

```bash
# Run tests
mise run test

# Run tests in watch mode
mise run test-watch
```

### Development Server

```bash
# Start development environment (builds WASM and serves web example)
mise run dev
```

### Other Tasks

- `mise run check` - Run format check, clippy, and security audit
- `mise run format` - Format code
- `mise run clean` - Clean build artifacts
- `mise run doc` - Generate documentation
- `mise run benchmark` - Run benchmarks
- `mise run update-deps` - Update dependencies
- `mise run release` - Prepare for release
- `mise run ci` - Run CI checks

See `mise.toml` for all available tasks and their descriptions.

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
