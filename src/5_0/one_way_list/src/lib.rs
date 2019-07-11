use std::rc::Rc;
#[derive(Debug,PartialEq)]
struct Node {
    item: i32,
    next: Option<Rc<Node>>,
    prev: Option<Rc<Node>>,
}
impl Node {
    fn new(item: i32) -> Node { Node { item: item, next: None, prev: None } }
    fn item(&self) -> i32 { self.item }
    fn next(&self) -> &Option<Rc<Node>> { &self.next }
    fn prev(&self) -> &Option<Rc<Node>> { &self.prev }
}
struct TwoWayList {
    head: Option<Rc<Node>>,
    tail: Option<Rc<Node>>,
}
impl TwoWayList {
    fn new() -> TwoWayList { TwoWayList { head: None, tail: None } }
    fn push(&mut self, item: i32) {
        let mut node = Node::new(item);
        if self.head == None {
            node.next = None;
            node.prev = None;
            let rc = Rc::new(node);
//            self.head = Some(rc);
            self.head = Some(Rc::clone(&rc));
            self.tail = Some(Rc::clone(&rc)); // error[E0507]: cannot move out of borrowed content
//            self.head = Some(Rc::new(node));
//            self.tail = Some(Rc::clone(&self.head.unwrap())); // error[E0507]: cannot move out of borrowed content
//            self.tail = Some(Rc::clone(self.head.unwrap())); // error[E0308]: mismatched types
//            self.tail = Some(Rc::clone(&self.head.unwrap())); // error[E0507]: cannot move out of borrowed content
        } else {
            node.next = None;
//            node.prev = Some(Rc::new(*self.tail.unwrap())); // error[E0507]: cannot move out of an `Rc`
//            node.prev = self.tail; // error[E0507]: cannot move out of borrowed content
//            node.prev = Some(Rc::clone(&self.tail.unwrap())); // error[E0507]: cannot move out of borrowed content

//            node.prev = Some(Rc::clone(&self.tail.unwrap())); // error[E0507]: cannot move out of borrowed content
//            self.tail = Some(Rc::new(node));
//            let rc = self.tail.unwrap(); // error[E0507]: cannot move out of borrowed content
//            let rc = &self.tail.unwrap(); // error[E0507]: cannot move out of borrowed content
//            let rc = *self.tail.unwrap(); // error[E0308]: mismatched types
//            let tail = &self.tail;
//            let a = &tail;
//            let rc = &tail.unwrap();
//            let rc = self.tail.unwrap(); // error[E0507]: cannot move out of borrowed content
            let rc = self.tail.as_ref().unwrap();
            node.prev = Some(Rc::clone(&rc));
            self.tail = Some(Rc::new(node));
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
