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
            self.head = Some(Rc::clone(&rc));
            self.tail = Some(Rc::clone(&rc));
        } else {
            /*
            node.next = None;
            let rc = self.tail.as_ref().unwrap();
            node.prev = Some(Rc::clone(&rc));
            self.tail = Some(Rc::new(node));
            */
            /*
            node.next = None;
            let tail = self.tail.as_ref().unwrap();
//            let tail = self.tail.unwrap();
//            node.prev = Some(Rc::clone(&tail));
            node.prev = Some(Rc::clone(&tail));
            let new = Rc::new(node);
            self.tail = Some(new);
            tail.next = Some(Rc::clone(&new))
            */
            /*
            node.next = None;
            let mut old_tail;
//            self.tail = ::std::mem::replace(&mut old_tail, node); // self.tailにnodeが入る。self.tailの代入前の値はold_tailに入る
            let new = Rc::new(node);
            self.tail = ::std::mem::replace(&mut old_tail, Some(new)); // self.tailにnodeが入る。self.tailの代入前の値はold_tailに入る
//            node.prev = Some(Rc::clone(&old_tail)); // error[E0308]: mismatched types
            node.prev = Some(Rc::clone(&old_tail.unwrap())); // error[E0381]: borrow of possibly uninitialized variable: `old_tail`
//            node.prev = Some(Rc::clone(&old_tail.as_ref().unwrap()));
//            self.tail = Some(Rc::new(node));
//            self.tail = Some(Rc::clone(&new));
            */
            node.next = None;
            let mut new: Option<Rc<Node>>;
            let old_tail = ::std::mem::replace(&mut self.tail, Some(Rc::new(node)));;
//            self.tail.prev = old_tail; // error[E0609]: no field `prev` on type `std::option::Option<std::rc::Rc<Node>>`
            self.tail.unwrap().prev = Rc::clone(&old_tail); // error[E0308]: mismatched types
//            old_tail.next = self.tail;
            old_tail.unwrap().next = self.tail; // error[E0594]: cannot assign to data in a `&` reference
            let new = Rc::new(node);
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
    #[test]
    fn TwoWayList_push() {
        let mut list = TwoWayList::new();
        let root = Node::new(0);
        let n1 = Node::new(1);

        list.push(0);
        assert_eq!(list.head.as_ref().unwrap().item(), 0);
        assert_eq!(list.tail.as_ref().unwrap().item(), 0);
        assert_eq!(**list.head.as_ref().unwrap(), Node { item: 0, next: None, prev: None });

        list.push(1);
        assert_eq!(list.head.as_ref().unwrap().item(), 0);
        assert_eq!(list.tail.as_ref().unwrap().item(), 1);

        let next = list.head.as_ref().unwrap().next();
        assert_eq!(next.as_ref(), None);
//        assert_eq!(next.as_ref().unwrap().item(), 1);
//        let prev = list.head.as_ref().unwrap().prev();
//        assert_eq!(prev.as_ref(), None);
//        assert_eq!(prev.as_ref().unwrap().item(), 1);
        /*
        assert_eq!(prev.as_ref().unwrap().item(), None);
//        assert_eq!(list.head.as_ref().unwrap().next().item(), 1);
        assert_eq!(list.head.as_ref().unwrap().prev(), None);
        assert_eq!(list.tail.as_ref().unwrap().next(), None);
        assert_eq!(list.tail.as_ref().unwrap().prev().item(), 0);
        assert_eq!(**list.head.as_ref().unwrap(), 
            Node { item: 0, 
                next: Some(Rc::new(Node { item: 1, next: None, prev: Node { item: 0, next: None, .. }})), 
                prev: None });
        */
    }
}
