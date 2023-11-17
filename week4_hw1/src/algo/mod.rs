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
    todo!()
}

#[allow(dead_code, unused_variables)]
pub fn breadth_first_search<T>(node: &node::NodeRef<T>, value: T) -> Option<node::NodeRef<T>>
where
    T: PartialEq + Clone,
{
    // TODO: find elements in a tree with BFS
    // NOTE: VecDeque is used
    todo!()
}

#[allow(dead_code, unused_variables)]
pub fn traverse_tree<T, F>(node: &node::NodeRef<T>, callback: F)
where
    T: Clone,
    // TODO: set the trait bounds for closure
    // F: <SOMETHING> + Copy,
    // NOTE: think about why we need Copy (hint: the closure is passed to next function)
{
    // TODO: traverse a tree with DFS and apply a callback closure to each node
    todo!()
}