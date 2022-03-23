// THIS TRAIT IS NOT CURRENTLY USED, but is being kept in case I refactor the code and implement
// examples of circular mazes

// A trait for graphs that can have up to four edges from a Node. The edges are represented
// by the directions north, south, east, west
// N is the node type
// pub trait FourEdge<N> {
//     // returns the node to the north of the given `node`, if any
//     fn north(&self, node: &N) -> Option<&N>;
//
//     // returns the node to the south of the given `node`, if any
//     fn south(&self, node: &N) -> Option<&N>;
//
//     // returns the node to the east of the given `node`, if any
//     fn east(&self, node: &N) -> Option<&N>;
//
//     // returns the node to the west of the given `node`, if any
//     fn west(&self, node: &N) -> Option<&N>;
// }