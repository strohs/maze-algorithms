use std::hash::{Hash, Hasher};

/// Node is the type of elements that can be stored in a graph. Every node has a value of some
/// type `V`, plus a weight.
///
#[derive(Debug)]
pub struct Node<V: Hash + PartialEq + Eq> {
    value: V,
    weight: isize,
}

impl<V: Hash + PartialEq + Eq> Node<V> {

    // constructs a new Node
    pub fn new(value: V, weight: isize) -> Self {
        Node {
            value,
            weight,
        }
    }

    // returns the value of this node
    pub fn value(&self) -> &V {
        &self.value
    }

    // returns the weight of the node
    pub fn weight(&self) -> isize {
        self.weight
    }

    pub fn set_value(&mut self, new_value: V) {
        self.value = new_value;
    }

    pub fn set_weight(&mut self, new_weight: isize) {
        self.weight = new_weight;
    }
}


impl<V: Hash + PartialEq + Eq> PartialEq for Node<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Hash + PartialEq + Eq> Eq for Node<V> {}

// HASH impl
impl<V: Hash + PartialEq + Eq> Hash for Node<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}



#[cfg(test)]
mod tests {
    use super::Node;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn should_create_new_node() {
        let node = Node::new(1, 125);
        assert_eq!(node.value, 1);
        assert_eq!(node.weight, 125);
    }

    #[test]
    fn equal_values_should_hash_to_equal_hashes() {
        let mut hasher = DefaultHasher::new();
        let node1 = Node::new(1, 111);
        let node2 = Node::new(1, 222);
        node1.hash(&mut hasher);

        let mut hasher2 = DefaultHasher::new();
        node2.hash(&mut hasher2);
        assert_eq!(hasher.finish(), hasher2.finish());
    }
}