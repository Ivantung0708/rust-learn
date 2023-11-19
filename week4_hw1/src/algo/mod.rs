// NOTE: crate is the root
use crate::node;
// TODO: read the usage of VecDeque
use std::collections::VecDeque;

#[allow(dead_code, unused_variables)]
pub fn depth_first_search<T>(node: &node::NodeRef<T>, value: T) -> Option<node::NodeRef<T>>
where
    // NOTE: think about why we need this trait bounds
    T: PartialEq + Clone,
{
    // TODO: find elements in a tree
    if node.borrow().data == value {
        return Some(node.clone());
    }
    for child in &node.borrow().children {
        let ans: Option<node::NodeRef<T>> = depth_first_search(child, value.clone());
        match ans {
            None => continue,
            Some(ref _t) => return ans,
        }
    }
    return None;
}

#[allow(dead_code, unused_variables)]
pub fn breadth_first_search<T>(node: &node::NodeRef<T>, value: T) -> Option<node::NodeRef<T>>
where
    T: PartialEq + Clone,
{
    // TODO: find elements in a tree with BFS
    // NOTE: VecDeque is used
    let mut v:VecDeque<node::NodeRef<T>> = VecDeque::new();
    v.push_back(node.clone());
    while !v.is_empty() {
        let current_node = v.pop_front().unwrap();
        if current_node.borrow().data == value {
            return Some(current_node);
        }
        for nodes in &current_node.borrow().children {
            v.push_back(nodes.clone());
        }
    }
    return None;
}

#[allow(dead_code, unused_variables)]
pub fn traverse_tree<T, F>(node: &node::NodeRef<T>, callback: F)
where
    T: Clone,
    F:Fn(node::NodeRef<T>) -> () + Copy,
    // TODO: set the trait bounds for closure
    // F: <SOMETHING> + Copy,
    // NOTE: think about why we need Copy (hint: the closure is passed to next function)
{
    // TODO: traverse a tree with DFS and apply a callback closure to each node
    callback(node.clone());
    for child in &node.borrow().children {
        traverse_tree(child,callback);
    }
}