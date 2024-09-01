trait HuffBaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> u32;
}

struct HuffLeafNode {
    element: char,
    weight: u32,
}

impl HuffLeafNode {
    fn new(element: char, weight: u32) -> Self {
        HuffLeafNode { element, weight }
    }

    fn value(&self) -> char {
        self.element
    }
}

impl HuffBaseNode for HuffLeafNode {
    fn is_leaf(&self) -> bool {
        true
    }

    fn weight(&self) -> u32 {
        self.weight
    }
}

struct HuffInternalNode {
    weight: u32,
    left: Box<dyn HuffBaseNode>,
    right: Box<dyn HuffBaseNode>,
}

impl HuffInternalNode {
    fn new(left: Box<dyn HuffBaseNode>, right: Box<dyn HuffBaseNode>, weight: u32) {
        HuffInternalNode { left, right, weight }
    }

    fn left(&self) -> &dyn HuffBaseNode {
        &*self.left
    }

    fn right(&self) -> &dyn HuffBaseNode {
        &*self.right
    }
}

impl HuffBaseNode for HuffInternalNode {
    fn is_leaf(&self) -> bool {
        false
    }

    fn weight(&self) -> u32 {
        self.weight
    }
}
