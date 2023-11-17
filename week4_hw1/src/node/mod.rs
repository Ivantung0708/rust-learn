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
        todo!()
    }
}

#[allow(dead_code, unused_variables)]
pub fn new<T>(data: T) -> NodeRef<T> {
    todo!()
}

#[allow(dead_code, unused_variables)]
pub fn append_child<T>(node: &NodeRef<T>, child: &NodeRef<T>) {
    todo!()
}

#[allow(dead_code, unused_variables)]
pub fn get_child<T>(node: &NodeRef<T>, index: usize) -> Option<NodeRef<T>> {
    todo!()
}

#[allow(dead_code, unused_variables)]
pub fn get_parent<T>(node: &NodeRef<T>) -> Option<NodeRef<T>> {
    todo!()
}