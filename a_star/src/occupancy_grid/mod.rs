// Create a randomized occupancy grid
// Input: width, length, number of obstacles
//     randomize x,y location of obstacles and height magnitude
//     randomize width magnitude with max of min(length,width), slopes off bell curve?
use rand::Rng;
use std::convert::TryFrom;
//use grid::*;
use grid::Grid;

// let mut grid = grid![[1,2,3]
//                     [4,5,6]];
// assert_eq!(grid, Grid::from_vec(vec![1,2,3,4,5,6],3));
//assert_eq!(grid.get(0,2), Some(&3));
//assert_eq!(grid[1][1], 5);
//assert_eq!(grid.size(), (2,3));
//grid.push_row(vec![7,8,9]);
//assert_eq!(grid, grid![[1,2,3][4,5,6][7,8,9]])

pub struct OccuGrid {
    pub width: u32,
    pub length: u32,
    pub obstacle_count: u32,
    pub grid: Grid<u8>
}

impl OccuGrid {
    pub fn new(width: u32, length: u32) -> Self {
        // Turning u32s into usize
        let uwidth = usize::try_from(width).unwrap();
        let ulength = usize::try_from(length).unwrap();
        // Creating the actual grid instance
        let grid : Grid<u8> = Grid::new(uwidth,ulength);
        println!("The width is {width}");
        println!("The length is {length}");
        Self { width: width, length: length, obstacle_count: 0, grid: grid}
    }
    pub fn add_obstacles(&self, obstacle_count: u32) {
        let obs_x: u32 = 0;
        let obs_y: u32 = 0;
        println!("Added obstacle at ({obs_x},{obs_y})");
    }
}
    // fn generate_obstacles(&self, obstacle_count) -> u32 {
    //    let secret_number = rand::thread_rng().gen_range(1..=100);
    //    self.width * self.height
    //}
