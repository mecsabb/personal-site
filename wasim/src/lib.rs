use wasm_bindgen::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Generic grid that can handle different cell types
#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new_with_default(width: usize, height: usize, default_value: T) -> Self {
        let cells = vec![default_value; width * height];
        Self { cells, width, height }
    }
    
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.cells[y * self.width + x])
        } else {
            None
        }
    }
    
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = value;
        }
    }
    
    pub fn cells(&self) -> &[T] {
        &self.cells
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
}

// Specialized constructor for boolean grids (Conway)
impl Grid<bool> {
    pub fn new_random_bool(width: usize, height: usize) -> Self {
        let mut cells = vec![false; width * height];
        let mut rng = StdRng::from_entropy();

        // Fill with random pattern - roughly 30% alive
        for i in 0..cells.len() {
            let random_value: f64 = rng.gen();
            cells[i] = random_value < 0.3;
        }

        Self { cells, width, height }
    }
}

// Specialized constructor for f32 grids (Worms)
impl Grid<f32> {
    pub fn new_random_f32(width: usize, height: usize) -> Self {
        let mut cells = vec![0.0; width * height];
        let mut rng = StdRng::from_entropy();

        // Fill with random pattern - values between 0.0 and 1.0
        for i in 0..cells.len() {
            cells[i] = rng.gen::<f32>();
        }

        Self { cells, width, height }
    }
}

// Conway's Game of Life simulation
#[wasm_bindgen]
pub struct ConwaySimulation {
    grid: Grid<bool>,
}

#[wasm_bindgen]
impl ConwaySimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> ConwaySimulation {
        console_error_panic_hook::set_once();
        console_log!("Conway's Game of Life simulation initialized: {}x{}", width, height);
        
        ConwaySimulation {
            grid: Grid::new_random_bool(width, height),
        }
    }
    
    pub fn cells(&self) -> js_sys::Uint8Array {
        let cells: Vec<u8> = self.grid.cells().iter().map(|&b| if b { 1 } else { 0 }).collect();
        js_sys::Uint8Array::from(&cells[..])
    }
    
    pub fn tick(&mut self) {
        let mut next_cells = self.grid.cells().to_vec();
        
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let current = *self.grid.get(x, y).unwrap_or(&false);
                let live_neighbors = self.count_neighbors(x, y);
                
                let next_state = match (current, live_neighbors) {
                    (true, 2) | (true, 3) => true,  // Survive
                    (false, 3) => true,             // Birth
                    _ => false,                     // Death
                };
                
                next_cells[y * self.grid.width() + x] = next_state;
            }
        }
        
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                self.grid.set(x, y, next_cells[y * self.grid.width() + x]);
            }
        }
    }
    
    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        for dy in -1i32..=1i32 {
            for dx in -1i32..=1i32 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = (x as i32 + dx + self.grid.width() as i32) % self.grid.width() as i32;
                let ny = (y as i32 + dy + self.grid.height() as i32) % self.grid.height() as i32;
                
                if *self.grid.get(nx as usize, ny as usize).unwrap_or(&false) {
                    count += 1;
                }
            }
        }
        
        count
    }
}

// Worms simulation
#[wasm_bindgen]
pub struct WormsSimulation {
    grid: Grid<f32>,
}

#[wasm_bindgen]
impl WormsSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> WormsSimulation {
        console_error_panic_hook::set_once();
        console_log!("Worms simulation initialized: {}x{}", width, height);
        
        WormsSimulation {
            grid: Grid::new_random_f32(width, height),
        }
    }
    
    pub fn cells(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(self.grid.cells())
    }
    
    pub fn tick(&mut self) {
        let width = self.grid.width();
        let height = self.grid.height();
        
        // Work directly with the flat array to avoid allocations
        let input_cells = self.grid.cells();
        let mut output_cells = vec![0.0f32; width * height];
        
        // Apply worms step directly on flat arrays
        worms_step_optimized(input_cells, &mut output_cells, width, height);
        
        // Update grid in place
        for i in 0..output_cells.len() {
            let x = i % width;
            let y = i / width;
            self.grid.set(x, y, output_cells[i]);
        }
    }
}

// Backward compatibility wrapper - keep the old Simulation name pointing to Conway
#[wasm_bindgen]
pub struct Simulation {
    inner: ConwaySimulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Simulation {
        Simulation {
            inner: ConwaySimulation::new(width, height),
        }
    }
    
    pub fn cells(&self) -> js_sys::Uint8Array {
        self.inner.cells()
    }
    
    pub fn tick(&mut self) {
        self.inner.tick();
    }
}

// Optimized version that works with flat arrays to avoid allocations
pub fn worms_step_optimized(input: &[f32], output: &mut [f32], width: usize, height: usize) {
    // 3×3 kernel from the preset (row-major).
    const K: [[f32; 3]; 3] = [
        [ 0.68, -0.90,  0.68],
        [-0.90, -0.66, -0.90],
        [ 0.68, -0.90,  0.68],
    ];

    // Activation: 1 - 2^(-0.6 * x^2)
    #[inline]
    fn inverse_gaussian(x: f32) -> f32 {
        1.0 - 2.0f32.powf(-0.6 * x * x)
    }

    #[inline]
    fn get_wrap_flat(input: &[f32], x: isize, y: isize, width: isize, height: isize) -> f32 {
        let xx = ((x % width) + width) % width;
        let yy = ((y % height) + height) % height;
        input[(yy * width + xx) as usize]
    }

    for y in 0..height {
        let yy = y as isize;
        for x in 0..width {
            let xx = x as isize;
            let w = width as isize;
            let h = height as isize;

            // Unrolled 3×3 convolution around (x,y)
            let s =
                K[0][0] * get_wrap_flat(input, xx - 1, yy - 1, w, h) +
                K[0][1] * get_wrap_flat(input, xx + 0, yy - 1, w, h) +
                K[0][2] * get_wrap_flat(input, xx + 1, yy - 1, w, h) +
                K[1][0] * get_wrap_flat(input, xx - 1, yy + 0, w, h) +
                K[1][1] * get_wrap_flat(input, xx + 0, yy + 0, w, h) +
                K[1][2] * get_wrap_flat(input, xx + 1, yy + 0, w, h) +
                K[2][0] * get_wrap_flat(input, xx - 1, yy + 1, w, h) +
                K[2][1] * get_wrap_flat(input, xx + 0, yy + 1, w, h) +
                K[2][2] * get_wrap_flat(input, xx + 1, yy + 1, w, h);

            output[y * width + x] = inverse_gaussian(s);
        }
    }
}

pub fn worms_step(input: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let h = input.len();
    assert!(h > 0, "empty grid");
    let w = input[0].len();
    assert!(input.iter().all(|row| row.len() == w), "ragged grid");

    // 3×3 kernel from the preset (row-major).
    const K: [[f32; 3]; 3] = [
        [ 0.68, -0.90,  0.68],
        [-0.90, -0.66, -0.90],
        [ 0.68, -0.90,  0.68],
    ];

    // Activation: 1 - 2^(-0.6 * x^2)
    #[inline]
    fn inverse_gaussian(x: f32) -> f32 {
        // Using powf for clarity; you could use (-(0.6 * x * x) * std::f32::consts::LN_2).exp()
        1.0 - 2.0f32.powf(-0.6 * x * x)
    }

    #[inline]
    fn get_wrap(input: &[Vec<f32>], x: isize, y: isize) -> f32 {
        let h = input.len() as isize;
        let w = input[0].len() as isize;
        let xx = ((x % w) + w) % w;
        let yy = ((y % h) + h) % h;
        input[yy as usize][xx as usize]
    }

    let mut out = vec![vec![0.0f32; w]; h];

    for y in 0..h {
        let yy = y as isize;
        for x in 0..w {
            let xx = x as isize;

            // Unrolled 3×3 convolution around (x,y)
            let s =
                K[0][0] * get_wrap(input, xx - 1, yy - 1) +
                K[0][1] * get_wrap(input, xx + 0, yy - 1) +
                K[0][2] * get_wrap(input, xx + 1, yy - 1) +
                K[1][0] * get_wrap(input, xx - 1, yy + 0) +
                K[1][1] * get_wrap(input, xx + 0, yy + 0) +
                K[1][2] * get_wrap(input, xx + 1, yy + 0) +
                K[2][0] * get_wrap(input, xx - 1, yy + 1) +
                K[2][1] * get_wrap(input, xx + 0, yy + 1) +
                K[2][2] * get_wrap(input, xx + 1, yy + 1);

            out[y][x] = inverse_gaussian(s);
        }
    }

    out
}
