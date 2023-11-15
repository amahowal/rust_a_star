pub mod occupancy_grid;

fn main() {
    // TODO: how to default obstacle count to zero?
    let occu_grid = crate::occupancy_grid::OccuGrid::new(10, 30);
    // occu_grid.initialize_grid();
    //occu_grid.add_obstacles(obstacle_count: 3);
}


// Create a randomized occupancy grid
// Input: width, length, number of obstacles
//     randomize x,y location of obstacles and height magnitude
//     randomize width magnitude with max of min(length,width), slopes off bell curve?
