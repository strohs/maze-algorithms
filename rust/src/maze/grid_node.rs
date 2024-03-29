use std::hash::{Hash, Hasher};


/// GridNode is the default 'node' type that can be stored in a maze. GridNodes contain a `pos`ition
/// field that is used to uniquely identify a node's position in a maze.
/// Additionally, GridNodes have a `weight` field that can be used to store cost calculations; useful for maze solvers,
/// path finding algorithms, etc..
#[derive(Debug, Copy, Clone)]
pub struct GridNode {
    pos: usize,
    weight: isize,
}

impl GridNode {

    /// constructs a new Node with the specified `pos` and `weight`
    pub fn new(pos: usize, weight: isize) -> Self {
        GridNode {
            pos,
            weight,
        }
    }

    /// returns the pos of this node
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// returns the weight of the node
    pub fn weight(&self) -> isize {
        self.weight
    }

    /// sets the position value of this node
    pub fn set_pos(&mut self, new_pos: usize) {
        self.pos = new_pos;
    }

    /// sets the weight of this node
    pub fn set_weight(&mut self, new_weight: isize) {
        self.weight = new_weight;
    }
}


impl PartialEq for GridNode {
    /// two grid nodes are considered equal if their respective `pos` fields are equal
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for GridNode {}

impl Hash for GridNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}


#[cfg(test)]
mod tests {
    use super::GridNode;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn should_create_new_node() {
        let node = GridNode::new(1, 125);
        assert_eq!(node.pos, 1);
        assert_eq!(node.weight, 125);
    }

    #[test]
    fn equal_pos_should_hash_to_equal_hashes() {
        let mut hasher = DefaultHasher::new();
        let node1 = GridNode::new(1, 111);
        let node2 = GridNode::new(1, 222);
        node1.hash(&mut hasher);

        let mut hasher2 = DefaultHasher::new();
        node2.hash(&mut hasher2);
        assert_eq!(hasher.finish(), hasher2.finish());
    }
}