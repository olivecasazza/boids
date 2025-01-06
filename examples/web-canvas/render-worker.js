// Create an OffscreenCanvas for rendering
let canvas;
let ctx;

self.onmessage = function(e) {
    const { type, data } = e.data;

    switch (type) {
        case 'init':
            canvas = e.data.canvas;
            ctx = canvas.getContext('2d');
            break;

        case 'render':
            const { positions } = data;
            const drawStart = performance.now();
            
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            
            // Group boids by color for batch rendering
            const colorGroups = new Map();
            
            for (let i = 0; i < positions.length; i += 8) {
                const x = positions[i];
                const y = positions[i + 1];
                const size = positions[i + 2];
                const angle = positions[i + 3];
                const color = `rgba(${
                    Math.floor(positions[i + 4] * 255)},${
                    Math.floor(positions[i + 5] * 255)},${
                    Math.floor(positions[i + 6] * 255)},${
                    positions[i + 7]})`;
                
                if (!colorGroups.has(color)) {
                    colorGroups.set(color, []);
                }
                colorGroups.get(color).push({x, y, size, angle});
            }
            
            // Batch render boids by color
            for (const [color, boids] of colorGroups) {
                ctx.fillStyle = color;
                ctx.beginPath();
                for (const {x, y, size, angle} of boids) {
                    // Draw triangle aligned with velocity
                    const tipX = x + Math.cos(angle) * size * 2;
                    const tipY = y + Math.sin(angle) * size * 2;
                    const backX1 = x + Math.cos(angle + 2.3) * size;
                    const backY1 = y + Math.sin(angle + 2.3) * size;
                    const backX2 = x + Math.cos(angle - 2.3) * size;
                    const backY2 = y + Math.sin(angle - 2.3) * size;
                    
                    ctx.moveTo(tipX, tipY);
                    ctx.lineTo(backX1, backY1);
                    ctx.lineTo(backX2, backY2);
                    ctx.closePath();
                }
                ctx.fill();
            }

            const drawTime = performance.now() - drawStart;
            self.postMessage({ type: 'drawTime', data: drawTime });
            break;
    }
};
