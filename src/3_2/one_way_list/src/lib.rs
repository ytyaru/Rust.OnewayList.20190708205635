#[derive(Debug,PartialEq)]
struct Node {
    item: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}
impl Node {
    fn new(item: i32) -> Node { Node { item: item, next: None, prev: None } }
    fn item(&self) -> i32 { self.item }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new_char() {
        let node = Node::new(0);
        assert_eq!(node.item, 0);
        assert_eq!(node.next, None);
        assert_eq!(node.prev, None);
//        assert_eq!(node.next(), None);
//        assert_eq!(node.prev(), None);
    }
}
