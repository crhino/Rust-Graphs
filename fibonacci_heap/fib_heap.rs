/*
 * Chris Piraino
 *
 * Fibonacci Heap
 */
use std::rc::Rc;
use std::rc::Rc::Weak;
use collections::dlist::DList;

pub type FibEntry<K,V> Rc<FibNode<K,V>>

struct FibNode<K,V> {
    parent: Weak<FibNode>,
    children: DList<FibEntry>,
    rank: int,
    marked: bool,
    key: K,
    value: V
}

pub struct FibHeap<K,V> {
    roots: DList<FibEntry>
}

impl<K,V> FibHeap<K,V> {
    pub fn new() -> FibHeap<K,V> {
        FibHeap { roots: DList::new() }
    }

    // Must return a pointer to the specific entry for O(1) delete_node
    // and delete_key.
    pub fn insert(&self, key: K, val: V) -> FibEntry<k,V> {

    }
    pub fn meld(&self, other: FibHeap<K,V>) -> FibHeap<K,V> {

    }
}
