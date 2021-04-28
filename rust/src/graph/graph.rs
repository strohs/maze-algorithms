use crate::graph::node::Node;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ops::Index;
use std::fmt::{Display, Formatter};
use std::slice::{ChunksExact, Iter};
use crate::graph::four_edge::FourEdge;

type MazeNode = Node<usize>;

#[derive(Debug)]
struct Graph<'a> {
    // stores the nodes of this graph. Each nodes value is it's row/col position converted to
    // a one-dimensional index
    nodes: Vec<MazeNode>,
    rows: usize,
    cols: usize,

    // stores the links between nodes, its a HashMap of &MazeNode => Vec<&MazeNode>
    links: RefCell<HashMap<&'a MazeNode, Vec<&'a MazeNode>>>
}


impl<'a> Graph<'a> {

    /// returns a new graph with the specified row,col dimensions. Every Node in the graph will
    /// have a weight of 1 and there will be no links between any nodes.
    pub fn new(rows: usize, cols: usize) -> Self {
        let nodes: Vec<MazeNode> = (0..(rows * cols)).map(|i| Node::new(i, 1)).collect();
        let links: RefCell<HashMap<&MazeNode, Vec<&MazeNode>>> = RefCell::new(HashMap::new());

        Graph {
            nodes,
            rows,
            cols,
            links,
        }
    }

    /// returns the row/col dimensions of this graph
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// returns a one-dimensional index based on the given row, col values
    /// `col_dim` is the number of columns in the graph
    fn idx1d(row: usize, col:usize, col_dim: usize) -> usize { row * col_dim + col }

    /// generates a two dimensional index from a one-dimensional index. Returns it as a
    /// `(row, col)` tuple
    fn idx2d(index: usize, col_dim:usize) -> (usize, usize) {
        (index / col_dim, index % col_dim)
    }

    pub fn get(&self, r:usize, c:usize) -> &MazeNode {
        let index = Graph::idx1d(r, c, self.cols);
        self.nodes.get(index).unwrap()
    }

    /// create a link between the two nodes in the graph. This will essentially create a passageway
    /// between them.
    /// `bi_link` creates a bi-directional link if it is `true`. Which means then in addition to
    /// creating a link from node1 => node2,  a link is also created from node2 => node1
    pub fn link(&self, node1: &'a MazeNode, node2: &'a MazeNode, bi_link: bool) {
        let mut links = self.links.borrow_mut();
        links.entry(node1).or_insert(vec![]).push(node2);
        if bi_link {
            links.entry(node2).or_insert(vec![]).push(node1);
        }
    }

    // returns the nodes that `node` has links to in a new Vector.
    // In this particular graph, each node can have at most 4 links, or edges, to another Node
    pub fn links(&self, node: &MazeNode) -> Option<Vec<&MazeNode>> {
        match self.links.borrow().get(node) {
            Some(linked_nodes) => {
                Some(linked_nodes.to_vec())
            },
            None => None,
        }
    }

    /// returns `true` if `node1` has a link to `node2`, else `false`
    pub fn has_link(&self, node1: &MazeNode, node2: &MazeNode) -> bool {
        match self.links.borrow().get(node1) {
            Some(node_links) => node_links.contains(&node2),
            None => false,
        }
    }

    /// returns the neighbors of the `node`. Neighbors are the nodes adjacent to `node` but NOT
    /// necessarily linked to `node`. To get the linked nodes, use the `links()` function
    pub fn neighbors(&self, node: &MazeNode) -> Vec<&MazeNode> {
        let mut neighbors = vec![];
        neighbors.push(self.north(node));
        neighbors.push(self.east(node));
        neighbors.push(self.south(node));
        neighbors.push(self.west(node));


        neighbors.into_iter().flatten().collect()
    }

    /// returns an immutable iterator over the *rows* of this grid
    pub fn iter_rows(&self) -> ChunksExact<'_, MazeNode> {
        self.nodes.chunks_exact(self.cols)
    }

    /// returns an immutable iterator over this Graph's Nodes in row order
    pub fn iter_nodes(&self) -> Iter<'_, MazeNode> {
        self.nodes.iter()
    }
}

impl FourEdge<MazeNode> for Graph<'_> {

    fn north(&self, node: &MazeNode) -> Option<&MazeNode> {
        let node_index = *node.value();
        if node_index > self.cols {
            self.nodes.get(node_index - self.cols)
        } else {
            None
        }
    }

    fn south(&self, node: &MazeNode) -> Option<&MazeNode> {
        self.nodes.get(*node.value() + self.cols)
    }

    fn east(&self, node: &MazeNode) -> Option<&MazeNode> {
        // if the node is not at the eastern edge of the graph
        if *node.value() % self.cols + 1 != self.cols {
            self.nodes.get(*node.value() + 1)
        } else {
            None
        }
    }

    fn west(&self, node: &MazeNode) -> Option<&MazeNode> {
        // if node is not on the western edge of the graph
        if *node.value() % self.cols != 0 {
            self.nodes.get(*node.value() - 1)
        } else {
            None
        }
    }
}


/// allows indexing into this graph using a single usize valu that represents the one-dimensional
/// index of the Node you wish to retrieve
impl Index<usize> for Graph<'_> {
    type Output = MazeNode;

    /// returns the Node at the specified index, `idx`, in this Graph. The `idx` should be a
    /// one-dimensional index for the node to be retrieved
    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx]
    }
}


/// pretty prints the Graph to standard out
impl Display for Graph<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write the top wall of the graph
        writeln!(f, "+{}", "----+".repeat(self.cols))?;

        for row in self.iter_rows() {
            // top holds the node's 'bodies' (blank spaces) and eastern walls
            let mut top = String::from("|");
            // bottom holds the cell's southern wall and corners ('+') sign
            let mut bottom = String::from("+");

            for node in row.iter() {
                // determine if an eastern wall should be drawn
                match self.east(node) {
                    Some(east_node) if self.has_link(node, east_node) => top.push_str("     "),
                    _ => top.push_str("    |"),
                }

                // determine if a southern wall should be drawn
                match self.south(node) {
                    Some(south_node) if self.has_link(node, south_node) => {
                        bottom.push_str("    +")
                    }
                    _ => bottom.push_str("----+"),
                }
            }

            writeln!(f, "{}", top)?;
            writeln!(f, "{}", bottom)?;
        }

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::graph::four_edge::FourEdge;

    #[test]
    fn create_new_graph_with_9_nodes() {
        let graph = Graph::new(3, 3);
        assert_eq!(graph.nodes.len(), 9);
    }

    #[test]
    fn new_graph_with_all_nodes_of_weight_1() {
        let graph = Graph::new(3, 3);
        for node in &graph.nodes {
            assert_eq!(node.weight(), 1);
        }
    }

    #[test]
    fn graph_has_3x4_dimension() {
        let graph = Graph::new(3, 4);
        let (r, c) = graph.dimensions();
        assert_eq!(r, 3);
        assert_eq!(c, 4);
    }

    #[test]
    fn should_bi_link_two_nodes() {
        let graph = Graph::new(3, 3);
        let n1 = graph.get(0, 0);
        let n2 = graph.get(0, 1);
        graph.link(n1, n2, true);
        assert_eq!(graph.links.borrow().get(n1).unwrap()[0], n2);
        assert_eq!(graph.links.borrow().get(n2).unwrap()[0], n1);
    }

    #[test]
    fn should_get_links() {
        let graph = Graph::new(3, 3);
        let n00 = graph.get(0, 0);
        let n01 = graph.get(0, 1);
        let n10 = graph.get(1, 0);
        let n02 = graph.get(0, 2);
        graph.link(n00, n01, true);
        graph.link(n00, n10, true);
        let n0_links = graph.links(n00);
        assert_eq!(n0_links.unwrap().len(), 2);
    }

    #[test]
    fn should_index_into_graph() {
        let graph = Graph::new(3, 3);
        // get the node at row 1, column 1, whos one-dimensional index should = 4;
        let node11 = &graph[4];
        assert_eq!(*node11.value(), 4);
    }

    #[test]
    fn node_0_should_not_have_north_neighbor() {
        let graph = Graph::new(3, 3);
        let node0 = &graph[0];
        assert_eq!(graph.north(node0), None);
    }

    #[test]
    fn node_0_should_not_have_west_neighbor() {
        let graph = Graph::new(3, 3);
        let node0 = &graph[0];
        assert_eq!(graph.west(node0), None);
    }

    #[test]
    fn node_6_should_have_north_neighbor() {
        let graph = Graph::new(3, 3);
        let node6 = &graph[6];
        assert_eq!(graph.north(node6), Some(&graph[3]));
    }

    #[test]
    fn node_2_should_not_have_east_neighbor() {
        let graph = Graph::new(3, 3);
        let node = &graph[2];
        assert_eq!(graph.east(node), None);
    }

    #[test]
    fn node_3_should_have_east_neighbor() {
        let graph = Graph::new(3, 3);
        let node = &graph[3];
        assert_eq!(graph.east(node), Some(&graph[4]));
    }

    #[test]
    fn node_8_should_not_have_east_neighbor() {
        let graph = Graph::new(3, 3);
        let node = &graph[8];
        assert_eq!(graph.east(node), None);
    }

    #[test]
    fn node_4_should_have_all_neighbors() {
        let graph = Graph::new(3, 3);
        let node = &graph[4];
        assert_eq!(graph.east(node), Some(&graph[5]));
        assert_eq!(graph.north(node), Some(&graph[1]));
        assert_eq!(graph.west(node), Some(&graph[3]));
        assert_eq!(graph.south(node), Some(&graph[7]));
    }

    #[test]
    fn should_display_maze_with_link_from_00_to_01() {
        let graph = Graph::new(4, 4);
        let n00 = graph.get(0, 0);
        let n01 = graph.get(0, 1);
        graph.link(n00, n01, true);
        println!("{}", &graph);
    }

}