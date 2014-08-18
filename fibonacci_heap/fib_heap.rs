/*
 * Chris Piraino
 *
 * Fibonacci Heap
 */
#![crate_name = "fib_heap"]
#![crate_type = "lib"]

extern crate collections;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::{Cell, RefCell};
use collections::dlist::DList;
use collections::Deque;

pub type FibEntry<K,V> = Rc<FibNode<K,V>>;

// Have a trait so that we can implement methods on FibEntry. (error E0116])
// Can I do this a different way?
trait FibHeapEntry {
    fn rank(&self) -> uint;
}

impl<K,V> FibHeapEntry for FibEntry<K,V> {
    fn rank(&self) -> uint {
        self.children.borrow().len()
    }
}

#[deriving(Clone)]
struct FibNode<K,V> {
    parent: Option<Weak<FibNode<K,V>>>,
    children: RefCell<DList<FibEntry<K,V>>>,
    // Rank is the length of children
    marked: Cell<bool>,
    key: RefCell<K>,
    value: V
}

impl<K: PartialOrd,V> PartialOrd for FibNode<K,V> {
    fn partial_cmp(&self, other: &FibNode<K,V>) -> Option<Ordering> {
        self.key.borrow().partial_cmp(other.key.borrow().deref())
    }
}

impl<K: PartialEq,V> PartialEq for FibNode<K,V> {
    fn eq(&self, other: &FibNode<K,V>) -> bool {
        self.key.eq(&other.key)
    }
}

pub struct FibHeap<K,V> {
    // The minimum element is always contained at the top of the first root.
    roots: DList<FibEntry<K,V>>
}

impl<K: PartialOrd + Clone, V: Clone> FibHeap<K,V> {
    pub fn new() -> FibHeap<K,V> {
        FibHeap { roots: DList::new() }
    }

    // Must return a pointer to the specific entry for O(1) delete_node
    // and delete_key.
    pub fn insert(&self, k: K, v: V) -> FibEntry<K,V> {
        let node = FibNode {
            parent: None,
            children: RefCell::new(DList::new()),
            marked: Cell::new(false),
            key: RefCell::new(k),
            value: v
        };
        let rc_node = Rc::new(node);
        let ret = rc_node.clone();
        let new_heap = FibHeap::new();
        new_heap.roots.push(rc_node);
        self.meld(new_heap);
        ret
    }
    pub fn meld(&self, other: FibHeap<K,V>) -> FibHeap<K,V> {
        if self.roots.is_empty() {
            self.roots.append(other.roots);
        } else if self.find_min().val0() <= other.find_min().val0() {
            self.roots.append(other.roots);
        } else {
            self.roots.prepend(other.roots);
        }
        *self
    }
    pub fn find_min(&self) -> (K, V) {
        match self.roots.front() {
            Some(n) => (n.key.borrow().clone(), n.value.clone()),
            None => fail!("Fibonacci heap is empty")
        }
    }
    pub fn delete_min(&self) -> (K, V) {
        match self.roots.pop_front() {
            None => fail!("Fibonacci heap is empty"),
            Some(min_node) => {

            }
        }
    }
}
