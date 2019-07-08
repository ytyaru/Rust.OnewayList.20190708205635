#[derive(Debug,PartialEq)]
struct Node<T> {
    item: T,
    next: Box<Option<Node<T>>>,
    prev: Box<Option<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Node<T> { Node { item: item, next: Box::new(None), prev: Box::new(None) } }
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
    fn Node_new() {
        let node = Node::new('a');
        assert_eq!(node.item, 'a');
        assert_eq!(node.next, Box::new(None));
        assert_eq!(node.prev, Box::new(None));
        assert_eq!(*node.next, None);
        assert_eq!(*node.prev, None);
    }
}
