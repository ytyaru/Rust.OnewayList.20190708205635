#[derive(Debug,PartialEq)]
struct Node {
    item: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}
impl Node {
    fn new(item: i32) -> Node { Node { item: item, next: None, prev: None } }
    fn item(&self) -> i32 { self.item }
    fn next(&self) -> &Option<Box<Node>> { &self.next }
    fn prev(&self) -> &Option<Box<Node>> { &self.prev }
}
struct TwoWayList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}
impl TwoWayList {
    fn new() -> TwoWayList { TwoWayList { head: None, tail: None } }
    fn push(&mut self, item: i32) {
        let mut node = Node::new(item);
        if self.head == None {
            node.next = None;
            node.prev = None;
            self.head = Some(Box::new(node)); // error[E0382]: use of moved value: `node`
            self.tail = Some(Box::new(node)); // error[E0382]: use of moved value: `node`
        } else {
            node.next = None;
            node.prev = Some(Box::new(*self.tail.unwrap()));
            self.tail = Some(Box::new(node));
        }
    }
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
        assert_eq!(*node.next(), None);
        assert_eq!(*node.prev(), None);
    }
    #[test]
    fn TwoWayList_new() {
        let list = TwoWayList::new();
        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
    }

}
