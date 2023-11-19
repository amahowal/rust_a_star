// A node is a location in 2D space (i,j)
//     g is the cost to go back to the beginning
//     h is the heuristic cost to the goal
//     f is the sum of g and h
//
// An open node is one for which we have calculated the h, g and f values
use num;

#[derive(Debug)]
struct Node {
    i: u32,
    j: u32,
    g: u32,
    h: u32,
    f: u32,
}

impl Node {
    // TODO: check out other heuristics
    // TODO: when you initialize a node calculate h
    fn calc_h(&mut self) {
        // This sqrt rounds towards zero, change if we need more precision
        self.h = num::integer::sqrt(10*(self.i.pow(2)+self.j.pow(2)));
    }
    fn calc_f(&mut self) {
        self.f = self.h + self.g;
    }
}

fn is_goal(active_node: &Node, goal_node: &Node) -> bool {
    if active_node.i == goal_node.i && active_node.j == goal_node.j {
        true
    } else { false }
}

fn generate_successor_nodes(active_node: &Node) -> Vec<Node> {
    let mut successor_nodes: Vec<Node> = Vec::new();
    for idx in 0..7 {
        let mut next_node = Node { i: 0, j: 0, g: 0, h: 0, f: 0 };
        // i
        if idx == 0 || idx == 3 || idx == 5 {
            next_node.i = active_node.i + 1;
        } else if idx == 2 || idx == 4 || idx == 7 {
            next_node.i = active_node.i - 1;
        }
        // j
        if idx == 0 || idx == 1 || idx == 2 {
            next_node.j = active_node.j + 1;
        } else if idx == 5 || idx == 6 || idx == 7 {
            next_node.j = active_node.j - 1;
        }
        // g
        if idx == 0 || idx == 2 || idx == 5 || idx == 7 {
            next_node.g = active_node.g + 14;
        } else if idx == 1 || idx == 3 || idx == 4 || idx == 6 {
            next_node.g = active_node.g + 10;
        }
        next_node.calc_h();
        next_node.calc_f();
        successor_nodes.push(next_node);
    }
    successor_nodes
}

// Check all of the candidate successor nodes to make sure they are not
//     1. Out of bounds
//     2. In the open set
//     3. In the closed set
fn is_valid(dims: &(u32, u32), open_nodes: &Vec<Node>, closed_nodes: &Vec<Node>) -> bool {
    // make sure the successor nodes aren't out of bounds
    // make sure the successor nodes aren't in the open set
    // make sure the successor nodes aren't in the closed set
    true
}

pub fn aStarPlan(grid: &crate::occupancy_grid::OccuGrid) -> u32 {
    // initialize the goal node (rows and cols are 0-index)
    let goal_node = Node { i: grid.width-1, j: grid.length-1, g: 0, h: 0, f: 0 };
    dbg!(&goal_node.i);
    dbg!(&goal_node.j);
    // initialize the open list
    let mut open_nodes: Vec<Node> = Vec::new();
    // initialize the starting node, g=0, and calculate H, then add to the openNode set
    let mut origin_node = Node { i: 0, j: 0, g: 0, h: 0, f: 0 };
    origin_node.calc_h();
    open_nodes.push(origin_node);

    // while the open node set is not empty
    let mut active_node: Node;
    let mut counter: u32 = 0;
    while open_nodes.len() > 0 {
        counter += 1;
        // sort the open set from highest to lowest f value
        dbg!(&open_nodes[0]);
        // pop the parent off the open set with the lowest f value
        active_node = open_nodes.pop().unwrap();

        // TODO: when to check if is goal?
        if is_goal(&active_node, &goal_node) {
            // check for goal
            return counter
        } else {
            // generate successor nodes
            let mut successor_nodes = generate_successor_nodes(&active_node);
            // loop around successor nodes and add them to the open nodes if valid and not goal
            open_nodes.append(&mut successor_nodes);
        }
    }
    // TODO: how to return a number or a failure? return an error probably?
    //       probably a result?
    return 0
}
