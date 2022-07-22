use std::collections::HashMap;
use std::ops::Index;
use std::fmt::Write;
use crate::maze::grid_node::GridNode;
use crate::maze::grid_maze::GridMaze;

/// Distances is a helper struct that holds how far every node in a Maze is from a `root` cell.
/// This distance information can be used by shortest-path algorithms (like Dijkstra's)
pub struct Distances {
    // root is the starting node im a maze
    root: GridNode,
    // stores the 'distance' from a GridNode, to the `root` cell.
    nodes: HashMap<GridNode, i32>,
}


impl Distances {

    /// returns a new Distance struct with the specified `root` GridNode as the root of
    /// the returned distance struct.
    pub fn new(root: GridNode) -> Self {
        let mut nodes = HashMap::new();
        // the root is inserted into the hashmap with a distance of 0 from itself
        nodes.insert(root, 0);

        Self { root, nodes }
    }


    /// returns the distance information for the given `node`. Returns `None` if
    /// the cell is not contained within Distances
    pub fn get(&self, node: &GridNode) -> Option<&i32> {
        self.nodes.get(node)
    }


    /// insert the given `node` with `distance` into this struct
    pub fn insert(&mut self, node: GridNode, distance: i32) {
        self.nodes.insert(node, distance);
    }
}

/// Allows indexing Distances using a `GridNode` struct and returning the distance of that
/// GridNode from the `root` node.
impl Index<GridNode> for Distances {
    type Output = i32;

    fn index(&self, node: GridNode) -> &Self::Output {
        &self.nodes[&node]
    }
}


/// A helper function that returns a "pretty printed" String containing the distance value of each
/// node from the root node. The distances values are printed as hexadecimal values.
/// Useful for debugging purposes
#[allow(dead_code)]
pub fn overlay_distances(maze: &GridMaze, distances: &Distances) -> String {
    let mut buf = String::new();
    let (_rows, cols) = maze.dimensions();

    // write the top wall of the maze
    let _ = writeln!(buf, "+{}", "----+".repeat(cols));

    for row in maze.iter_rows() {
        // top holds the cell 'bodies' (blank spaces) and eastern walls
        let mut top = String::from("|");
        // bottom holds the cell's southern wall and corners ('+') sign
        let mut bottom = String::from("+");

        for curr_node in row.iter() {
            let dist = *distances.get(curr_node).unwrap_or(&0);
            // the body of the node will display the distance from the root
            // determine if an eastern wall should be drawn
            match maze.east(curr_node) {
                Some(east_pos) if maze.has_link(curr_node, &east_pos) => {
                    let _ = write!(top, "  {:2x} ", dist);
                }
                _ => {
                    let _ = write!(top, "  {:2x}|", dist);
                }
            }

            // determine if a southern wall should be drawn
            match maze.south(curr_node) {
                Some(south_pos) if maze.has_link(curr_node, &south_pos) => {
                    bottom.push_str("    +")
                }
                _ => bottom.push_str("----+"),
            }
        }

        let _ = writeln!(buf, "{}\n", &top);
        let _ = writeln!(buf, "{}\n", &bottom);
    }
    buf
}
