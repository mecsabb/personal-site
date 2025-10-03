import init, { WormsSimulation } from './wasim.js';

const CELL_SIZE = 1; // Each simulation cell = 1 pixel
const GRID_WIDTH = 1200; // Much higher resolution for smoother patterns
const GRID_HEIGHT = 240; // Maintain 5:1 aspect ratio (1200:240)

async function initWorms() {
    try {
        const wasmModule = await init();
        console.log('WASM module loaded successfully');
        
        const canvas = document.getElementById('worms-canvas');
        if (!canvas) {
            console.error('Canvas element not found');
            return;
        }
        
        const ctx = canvas.getContext('2d');
        const simulation = new WormsSimulation(GRID_WIDTH, GRID_HEIGHT);
        console.log('Worms simulation created');
        
        // Set canvas internal resolution - same size as Conway but denser pixels
        canvas.width = GRID_WIDTH * CELL_SIZE;
        canvas.height = GRID_HEIGHT * CELL_SIZE;
        
        // The CSS will handle the display size (100% width, 120px height)
        // This ensures proper scaling and responsive behavior
        console.log(`Canvas internal resolution: ${canvas.width}x${canvas.height}`);
        
        function render() {
            const cells = simulation.cells();
            
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            
            for (let row = 0; row < GRID_HEIGHT; row++) {
                for (let col = 0; col < GRID_WIDTH; col++) {
                    const idx = row * GRID_WIDTH + col;
                    const value = cells[idx];
                    
                    // Convert f32 value to grayscale with better contrast
                    // Apply a slight curve to enhance the visual patterns
                    const intensity = Math.max(0, Math.min(1, value));
                    const enhanced = Math.pow(intensity, 0.8); // Slight gamma correction for better contrast
                    const colorValue = Math.floor(enhanced * 255);
                    
                    if (intensity > 0.005) { // Lower threshold for finer details
                        ctx.fillStyle = `rgb(${colorValue}, ${colorValue}, ${colorValue})`;
                        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
                    }
                }
            }
        }
        
        let frameCount = 0;
        let lastTime = performance.now();
        
        function tick() {
            const startTime = performance.now();
            simulation.tick();
            const tickTime = performance.now();
            render();
            const renderTime = performance.now();
            
            frameCount++;
            if (frameCount % 60 === 0) { // Log performance every 60 frames
                const fps = 60000 / (renderTime - lastTime);
                console.log(`FPS: ${fps.toFixed(1)}, Tick: ${(tickTime - startTime).toFixed(2)}ms, Render: ${(renderTime - tickTime).toFixed(2)}ms`);
                lastTime = renderTime;
            }
            
            setTimeout(tick, 8);
        }
        
        // Initial render and start animation
        render();
        tick();
        
    } catch (error) {
        console.error('Failed to initialize Worms simulation:', error);
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', initWorms);
