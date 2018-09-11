use hierarchy::Hierarchy;
use node::Node;
use walker::Walker;
use index::Index;

pub struct DOM {
    pub h: Hierarchy<Node>,
}

impl DOM {
    pub(crate) fn new(store: Hierarchy<Node>) -> DOM {
        DOM { h: store }
    }
}

//pub struct DomIterator {
//    root_node: Rc<Node>,
//    cur_node: Rc<Node>,
//    index: usize,
//}

//impl Iterator for DomIterator {
//    type Item = Node;
//
//    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//        unimplemented!()
//    }
//}
