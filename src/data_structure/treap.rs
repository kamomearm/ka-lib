#![allow(unused)]
use std::{
    cell::RefCell,
    rc::{self, Rc, Weak},
};

pub trait SortedSet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> Option<T>;
}
#[derive(Debug, Clone)]
pub struct TreapNode<T> {
    val: T,
    p: usize,
    left: RefCell<Option<Rc<TreapNode<T>>>>,
    right: RefCell<Option<Rc<TreapNode<T>>>>,
    parent: RefCell<Option<Weak<TreapNode<T>>>>,
}
pub struct Treap<T> {
    root: Option<Rc<TreapNode<T>>>,
    size: usize,
}
impl<T> Treap<T> {
    fn rotate_right(u: TreapNode<T>) {
        let mut w = u.right.borrow_mut().take().unwrap();
        *w.parent.borrow_mut() = u.parent.borrow_mut().take();
    }
}
