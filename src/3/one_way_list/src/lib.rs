#[derive(Debug,PartialEq)]
struct Node<T> {
    item: T,
    next: Box<Option<Node<T>>>,
    prev: Box<Option<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Node<T> { Node { item: item, next: Box::new(None), prev: Box::new(None) } }
    fn next(&self) -> T { self.next.unwrap().item } // error[E0507]: cannot move out of borrowed content
    fn prev(&self) -> T { self.prev.unwrap().item }
//    fn next(&self) -> &T { &self.next.unwrap().item } // error[E0515]: cannot return reference to temporary value
//    fn prev(&self) -> &T { &self.prev.unwrap().item }
//    fn next(&'a self) -> Node<T: 'a> { &self.next.unwrap() } // error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `:`
//    fn prev(&'a self) -> Node<T: 'a> { &self.prev.unwrap() }
//    fn next(&self) -> &Node<T> { &self.next.unwrap() } // error[E0515]: cannot return reference to temporary value
//    fn prev(&self) -> &Node<T> { &self.prev.unwrap() }
//    fn next(&self) -> Option<Node<T>> { *self.next } // error[E0507]: cannot move out of borrowed content
//    fn prev(&self) -> Option<Node<T>> { *self.prev }
//    fn next(&self) -> Box<Option<Node<T>>> { self.next } // error[E0507]: cannot move out of borrowed content
//    fn prev(&self) -> Box<Option<Node<T>>> { self.prev }
//    fn next(&mut self) -> Box<Option<Node<T>>> { self.next } // error[E0507]: cannot move out of borrowed content
//    fn prev(&mut self) -> Box<Option<Node<T>>> { self.prev }
//    fn next(self) -> Box<Option<Node<T>>> { self.next } // error[E0382]: use of moved value: `node`
//    fn prev(self) -> Box<Option<Node<T>>> { self.prev }
//    fn next(&self) -> Box<Option<Node<T>>> { Box::new(self.next) } // error[E0308]: mismatched types
//    fn prev(&self) -> Box<Option<Node<T>>> { Box::new(self.prev) }
//    fn next(&self) -> &Box<Option<Node<T>>> { &self.next } // error[E0308]: mismatched types
//    fn prev(&self) -> &Box<Option<Node<T>>> { &self.prev }

    /*
    fn append(&self, item: T) where T: std::cmp::PartialEq {
//        let mut leaf = self;
        let mut leaf = Box::new(Some(self));
        loop {
            if None == *leaf.unwrap().next() { self.next = Box::new(Some(Node::new(item))); break; }
            else { leaf = leaf.unwrap().next(); }
        }
    }
//    fn insert(&self, item: T) {
//    }
    */
}
/*
struct Node<T> {
    item: T,
    next: Box<Option<Node>>,
    prev: Box<Option<Node>>,
}
impl Node {
    fn new(item: T) -> Node<T> { Node<T> { item: item, next: None, prev: None } }
    fn next(&self) -> Box<Node> { self.next }
    fn prev(&self) -> Box<Node> { self.prev }
    fn append(&self, item: T) {
        let mut leaf = self;
        loop {
            if None == leaf.next() { self.next = item; break; }
            else { leaf = leaf.next(); }
        }
    }
//    fn insert(&self, item: T) {
//    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new_char() {
        let node = Node::new('a');
        assert_eq!(node.item, 'a');
        assert_eq!(node.next, Box::new(None));
        assert_eq!(node.prev, Box::new(None));
        assert_eq!(*node.next, None);
        assert_eq!(*node.prev, None);
        assert_eq!(*node.next(), None);
        assert_eq!(*node.prev(), None);
    }
}
