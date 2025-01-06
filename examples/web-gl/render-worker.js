// WebGL context and program info
let gl;
let program;
let positionBuffer;
let instanceExt;

// Vertex shader - transforms positions and handles rotation and size
const vsSource = `
attribute vec2 aPosition;
attribute vec2 aOffset;
attribute vec4 aColor;
attribute float aAngle;
attribute float aSize;

uniform vec2 uResolution;

varying vec4 vColor;
varying vec2 vPosition;

void main() {
    // Scale the vertex by the boid size
    vec2 scaledPosition = aPosition * aSize;
    
    // Rotate the vertex by the negated angle to match canvas rotation direction
    float s = sin(-aAngle);
    float c = cos(-aAngle);
    mat2 rotation = mat2(c, -s, s, c);
    vec2 rotatedPosition = rotation * scaledPosition;
    
    // Add the offset (boid position)
    vec2 position = rotatedPosition + aOffset;
    
    // Convert to clip space
    vec2 zeroToOne = position / uResolution;
    vec2 zeroToTwo = zeroToOne * 2.0;
    vec2 clipSpace = zeroToTwo - 1.0;
    
    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    vColor = aColor;
    vPosition = rotatedPosition;
}
`;

// Fragment shader - handles neon glow effect
const fsSource = `
precision highp float;
varying vec4 vColor;
varying vec2 vPosition;

void main() {
    // Calculate distance from center
    float distFromCenter = length(vPosition);
    float normalizedDist = length(vPosition / 2.0); // Normalize based on unit size
    
    // Base color with enhanced brightness
    vec3 baseColor = vColor.rgb * 5.0;  // Increased base brightness further
    
    // Initialize with no fill
    vec3 finalColor = vec3(0.0);
    
    // Strong edge glow
    float edgeWidth = 0.1;  // Width of the edge line
    float edgeDist = abs(normalizedDist - 0.9);  // Distance from the edge
    float edgeIntensity = smoothstep(edgeWidth, 0.0, edgeDist);
    
    // Enhanced outer glow with wider spread
    float glowStrength = exp(-normalizedDist * 0.999) * 6.0;  // Increased glow multiplier, reduced falloff further
    vec3 glowColor = baseColor * vec3(1.6, 1.6, 2.0);  // Enhanced blue shift and intensity
    
    // Combine edge and glow with stronger edge
    finalColor += glowColor * (edgeIntensity * 3.0 + glowStrength);
    
    // Add bloom effect
    float bloom = exp(-normalizedDist * 0.5) * 2.0;
    finalColor += baseColor * bloom;
    
    // Adjust alpha for smooth edges while maintaining strong edge visibility
    float alpha = max(edgeIntensity, glowStrength * 0.7) * vColor.a;
    
    gl_FragColor = vec4(finalColor, alpha);
}
`;

// Triangle vertices for a unit-sized boid (will be scaled by size)
const triangleVertices = new Float32Array([
    2.0, 0,           // Tip
    -1.0, 1.0,     // Back left
    -1.0, -1.0     // Back right
]);

// Initialize WebGL context and shaders
function initGL(canvas) {
    // Get WebGL context
    gl = canvas.getContext('webgl', {
        alpha: false,
        depth: false,
        stencil: false,
        antialias: true,
        preserveDrawingBuffer: false
    });
    
    if (!gl) {
        throw new Error('WebGL not supported');
    }

    // Get the instanced arrays extension
    instanceExt = gl.getExtension('ANGLE_instanced_arrays');
    if (!instanceExt) {
        throw new Error('ANGLE_instanced_arrays extension not supported');
    }

    // Create shader program
    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vsSource);
    const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fsSource);
    program = createProgram(gl, vertexShader, fragmentShader);

    // Create buffers
    positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, triangleVertices, gl.STATIC_DRAW);

    // Set clear color to dark for better neon effect
    gl.clearColor(0.01, 0.01, 0.02, 1.0); // Made slightly darker
    
    // Enable additive blending for glow effect
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE);

    return program;
}

// Create and compile a shader
function createShader(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        const info = gl.getShaderInfoLog(shader);
        gl.deleteShader(shader);
        throw new Error('Shader compilation error: ' + info);
    }
    return shader;
}

// Create and link a shader program
function createProgram(gl, vertexShader, fragmentShader) {
    const program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        const info = gl.getProgramInfoLog(program);
        throw new Error('Program linking error: ' + info);
    }
    return program;
}

// Handle messages from main thread
self.onmessage = function(e) {
    const { type, data } = e.data;

    switch (type) {
        case 'init':
            const canvas = e.data.canvas;
            initGL(canvas);
            
            // Get attribute locations
            const attributes = {
                position: gl.getAttribLocation(program, 'aPosition'),
                offset: gl.getAttribLocation(program, 'aOffset'),
                color: gl.getAttribLocation(program, 'aColor'),
                angle: gl.getAttribLocation(program, 'aAngle'),
                size: gl.getAttribLocation(program, 'aSize')
            };
            
            // Get uniform locations
            const uniforms = {
                resolution: gl.getUniformLocation(program, 'uResolution')
            };
            
            // Set viewport and resolution
            gl.viewport(0, 0, canvas.width, canvas.height);
            gl.useProgram(program);
            gl.uniform2f(uniforms.resolution, canvas.width, canvas.height);
            break;

        case 'render':
            const drawStart = performance.now();
            const { positions } = data;
            
            // Clear the canvas
            gl.clear(gl.COLOR_BUFFER_BIT);
            
            if (positions.length > 0) {
                // Create instance data arrays
                const numBoids = positions.length / 8;
                const offsets = new Float32Array(numBoids * 2);
                const colors = new Float32Array(numBoids * 4);
                const angles = new Float32Array(numBoids);
                const sizes = new Float32Array(numBoids);
                
                // Fill instance arrays
                for (let i = 0; i < positions.length; i += 8) {
                    const boidIndex = i / 8;
                    offsets[boidIndex * 2] = positions[i];     // x
                    offsets[boidIndex * 2 + 1] = positions[i + 1]; // y
                    sizes[boidIndex] = positions[i + 2];       // size
                    angles[boidIndex] = positions[i + 3];      // angle
                    colors[boidIndex * 4] = positions[i + 4];     // r
                    colors[boidIndex * 4 + 1] = positions[i + 5]; // g
                    colors[boidIndex * 4 + 2] = positions[i + 6]; // b
                    colors[boidIndex * 4 + 3] = positions[i + 7]; // a
                }
                
                // Bind and setup buffers
                gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
                gl.enableVertexAttribArray(0);
                gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
                
                const offsetBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, offsetBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, offsets, gl.STREAM_DRAW);
                gl.enableVertexAttribArray(1);
                gl.vertexAttribPointer(1, 2, gl.FLOAT, false, 0, 0);
                instanceExt.vertexAttribDivisorANGLE(1, 1);
                
                const colorBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, colors, gl.STREAM_DRAW);
                gl.enableVertexAttribArray(2);
                gl.vertexAttribPointer(2, 4, gl.FLOAT, false, 0, 0);
                instanceExt.vertexAttribDivisorANGLE(2, 1);
                
                const angleBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, angleBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, angles, gl.STREAM_DRAW);
                gl.enableVertexAttribArray(3);
                gl.vertexAttribPointer(3, 1, gl.FLOAT, false, 0, 0);
                instanceExt.vertexAttribDivisorANGLE(3, 1);

                const sizeBuffer = gl.createBuffer();
                gl.bindBuffer(gl.ARRAY_BUFFER, sizeBuffer);
                gl.bufferData(gl.ARRAY_BUFFER, sizes, gl.STREAM_DRAW);
                gl.enableVertexAttribArray(4);
                gl.vertexAttribPointer(4, 1, gl.FLOAT, false, 0, 0);
                instanceExt.vertexAttribDivisorANGLE(4, 1);
                
                // Draw instances with LINE_LOOP for outline
                instanceExt.drawArraysInstancedANGLE(gl.LINE_LOOP, 0, 3, numBoids);
                
                // Cleanup
                gl.deleteBuffer(offsetBuffer);
                gl.deleteBuffer(colorBuffer);
                gl.deleteBuffer(angleBuffer);
                gl.deleteBuffer(sizeBuffer);
            }
            
            const drawTime = performance.now() - drawStart;
            self.postMessage({ type: 'drawTime', data: drawTime });
            break;
    }
};
