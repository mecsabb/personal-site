import init, { ConwaySimulation } from './wasim.js';

const CELL_SIZE = 2;
const GRID_WIDTH = 300;
const GRID_HEIGHT = 60;

async function initConway() {
    try {
        const wasmModule = await init();
        console.log('WASM module loaded successfully');
        
        const canvas = document.getElementById('conway-canvas');
        if (!canvas) {
            console.error('Canvas element not found');
            return;
        }
        
        const ctx = canvas.getContext('2d');
        const simulation = new ConwaySimulation(GRID_WIDTH, GRID_HEIGHT);
        console.log('Simulation created');
        
        // Set canvas internal resolution
        canvas.width = GRID_WIDTH * CELL_SIZE;
        canvas.height = GRID_HEIGHT * CELL_SIZE;
        
            console.log(`Canvas size: ${canvas.width}x${canvas.height}`);
        
        function render() {
            const cells = simulation.cells();
            
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.fillStyle = '#000000';
            
            for (let row = 0; row < GRID_HEIGHT; row++) {
                for (let col = 0; col < GRID_WIDTH; col++) {
                    const idx = row * GRID_WIDTH + col;
                    if (cells[idx] === 1) {
                        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
                    }
                }
            }
        }
        
        function tick() {
            simulation.tick();
            render();
            setTimeout(tick, 100); // ~10 FPS
        }
        
        // Initial render and start animation
        render();
        tick();
        
    } catch (error) {
        console.error('Failed to initialize Conway simulation:', error);
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', initConway);