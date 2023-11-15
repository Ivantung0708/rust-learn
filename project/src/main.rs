use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    data: i32,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            data: data,
            parent: None,
            children: Vec::new(),
        }
    }
}
fn append_child(node: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
    child.borrow_mut().parent = Some(Rc::downgrade(node));
    node.borrow_mut().children.push(Rc::clone(child));
}
fn get_child(node: &Rc<RefCell<Node>>,index: usize) -> Option<Rc<RefCell<Node>>> {
    node.borrow().children.get(index).map(|c| Rc::clone(c) )

}
fn get_parent(node: Rc<RefCell<Node>>) -> Option<Weak<Node>> {
    
}