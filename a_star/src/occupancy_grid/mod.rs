// Create a randomized occupancy grid
// Input: width, length, number of obstacles
//     randomize x,y location of obstacles and height magnitude
//     randomize width magnitude with max of min(length,width), slopes off bell curve?
use rand::Rng;
use std::convert::TryFrom;
//use grid::*;
use grid::Grid;

// Define Occupancy Grid
pub struct OccuGrid {
    pub width: u32,
    pub length: u32,
    pub obstacle_count: u32,
    pub grid: Grid<u8>
}

// Implement Occupancy Grid
impl OccuGrid {
    pub fn new(width: u32, length: u32) -> Self {
        // Turning u32s into usize
        let uwidth = usize::try_from(width).unwrap();
        let ulength = usize::try_from(length).unwrap();
        // Creating the actual grid instance
        let mut grid : Grid<u8> = Grid::new(uwidth,ulength);
        println!("The width is {width}");
        println!("The length is {length}");
        // Fill the grid with zeros
        grid.fill(0);
        // Returns the OccuGrid
        Self { width: width, length: length, obstacle_count: 0, grid: grid}
    }
    pub fn add_obstacles(&mut self) {
        // Fill the grid with random stuff
        for cell in self.grid.iter_mut() {
            *cell = rand::thread_rng().gen_range(1..=100);
        }
        // Print stuff out from the grid
        for ((row, col), i) in self.grid.indexed_iter() {
            println!("value at row {row} and column {col} is: {i}");
        }
    }
}
