pub mod occupancy_grid;
pub mod a_star;

fn main() {
    // Create a randomized occupancy grid
    let mut occu_grid = crate::occupancy_grid::OccuGrid::new(10, 30);
    occu_grid.add_obstacles();

    // Find a way through
}

