use std::cell::RefCell;
use std::rc::{Rc, Weak};

// NOTE: type aliases
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;
pub type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

// NOTE: use pub to make fields public
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<WeakNodeRef<T>>,
    pub children: Vec<NodeRef<T>>,
}

#[allow(dead_code, unused_variables)]
impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        let v: Vec<NodeRef<T>> = Vec::new();
        Node {
            data: data,
            parent: None,
            children: v,
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn new<T>(data: T) -> NodeRef<T> {
    let v:Vec<NodeRef<T>> = Vec::new();
    let node = Node {
        data: data,
        parent: None,
        children: v,
    };
    Rc::new(RefCell::new(node))
}

#[allow(dead_code, unused_variables)]
pub fn append_child<T>(node: &NodeRef<T>, child: &NodeRef<T>) {
    child.borrow_mut().parent = Some(Rc::downgrade(&node));
    node.borrow_mut().children.push(Rc::clone(child));
}

#[allow(dead_code, unused_variables)]
pub fn get_child<T>(node: &NodeRef<T>, index: usize) -> Option<NodeRef<T>> {
    let child = node.borrow().children.get(index);
    let new_rc = match child {
        None => None,
        Some(T) => Some(Rc::clone(child.unwrap())),
    };
    new_rc
}

#[allow(dead_code, unused_variables)]
pub fn get_parent<T>(node: &NodeRef<T>) -> Option<NodeRef<T>> {
    let node_parent = node.borrow().parent;
    let new_rc = match node_parent {
        None => None,
        Some(T) => node_parent.unwrap().upgrade(),
    };
    new_rc
}