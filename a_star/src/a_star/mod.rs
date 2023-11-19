// A node is a location in 2D space (i,j)
//     g is the cost to go back to the beginning
//     h is the heuristic cost to the goal
//     f is the sum of g and h
//
// An open node is one for which we have calculated the h, g and f values

#[derive(Debug)]
struct Node {
    i: u32,
    j: u32,
    g: f64,
    h: f64,
    f: f64,
}

// TODO: check out other heuristics
fn calcH(i: f64, j: f64) -> f64 {
    (10.0*(i.powf(2.0)+j.powf(2.0))).sqrt()
}

fn isGoal(activeNode: &Node, goalNode: &Node) {
}

fn generateSuccessorNodes(activeNode: &Node) {
}

pub fn aStarPlan(grid: &crate::occupancy_grid::OccuGrid) {
    // initialize the goal node
    let goalNode = Node { i: 0, j: 0, g: 0.0, h: 0.0, f: 0.0 };
    // initialize the open list
    let mut openNodes: Vec<Node> = Vec::new();
    // initialize the starting node, g=0, and calculate H
    //dbg!(&openNodes[0]);
    // while the open node set is >= 0
    //while openNodes.size() >= 0 {
        // pick the parent off the open set with the lowest f value
        //activeNode = openNodes.pop();
        // check for goal
        //isGoal(&activeNode, &goalNode);
        // generate successor nodes
        //openNode.append(generateSuccessorNodes(&activeNode));
    //}
}
