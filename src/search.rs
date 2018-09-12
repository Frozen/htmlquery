//use hierarchy::Token;
//use node::Node;
//use hierarchy::Hierarchy;
//
//struct All {
//    tokens: Vec<Token>,
//}
//
//impl All {
//    pub fn new(tokens: Vec<Token>) -> All {
//        All { tokens }
//    }
//
//    pub fn len(&self) -> usize {
//        self.tokens.len()
//    }
//
//    pub fn is_empty(&self) -> bool {
//        self.len() == 0
//    }
//}
//
//impl<'a> IntoIterator for All {
//    type Item = Node;
//    type IntoIter = AllIterator<'a>;
//    //    type IntoIter = AllIterator<'a>;
//
//    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
//        self.tokens
//    }
//}
//
//struct AllIterator<'a> {
//    id: usize,
//    lst: &'a [Token],
//    h: Hierarchy<Node>,
//}
//
//impl<'a> Iterator for AllIterator<'a> {
//    type Item = ();
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if let Some(token) = self.lst.get(self.id) {
//            self.id += 1;
//            return Some(self.h[token]);
//        }
//        return None;
//    }
//}
