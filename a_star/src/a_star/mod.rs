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
    fn calc_g(&mut self) {
        self.g = self.h + self.f;
    }
}

fn isGoal(activeNode: &Node, goalNode: &Node) {
}

fn generateSuccessorNodes(activeNode: &Node) {
}

pub fn aStarPlan(grid: &crate::occupancy_grid::OccuGrid) {
    // initialize the goal node (rows and cols are 0-index)
    let goalNode = Node { i: grid.width-1, j: grid.length-1, g: 0, h: 0, f: 0 };
    dbg!(&goalNode.i);
    dbg!(&goalNode.j);
    // initialize the open list
    let mut openNodes: Vec<Node> = Vec::new();
    // initialize the starting node, g=0, and calculate H, then add to the openNode set
    let mut originNode = Node { i: 0, j: 0, g: 0, h: 0, f: 0 };
    originNode.calc_h();
    // while the open node set is not empty
    while openNodes.len() > 0 {
        // pick the parent off the open set with the lowest f value
        //activeNode = openNodes.pop();
        // check for goal
        //isGoal(&activeNode, &goalNode);
        // generate successor nodes
        //openNode.append(generateSuccessorNodes(&activeNode));
        openNodes.remove(0);
    }
}
