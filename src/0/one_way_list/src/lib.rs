#[derive(Debug,PartialEq)]
struct Node {
    item: i32,
    next: Box<Option<Node>>,
    prev: Box<Option<Node>>,
}
impl Node {
    fn new(item: i32) -> Node { Node { item: item, next: Box::new(None), prev: Box::new(None) } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new() {
        let node = Node::new(100);
        assert_eq!(node.item, 100);
        assert_eq!(node.next, Box::new(None));
        assert_eq!(node.prev, Box::new(None));
        assert_eq!(*node.next, None);
        assert_eq!(*node.prev, None);
    }
}
