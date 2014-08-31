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
    parent: RefCell<Option<Weak<FibNode<K,V>>>>,
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
    pub fn insert(&mut self, k: K, v: V) -> FibEntry<K,V> {
        let node = FibNode {
            parent: RefCell::new(None),
            children: RefCell::new(DList::new()),
            marked: Cell::new(false),
            key: RefCell::new(k),
            value: v
        };
        let rc_node = Rc::new(node);
        let ret = rc_node.clone();
        let mut new_heap = FibHeap::new();
        new_heap.roots.push(rc_node);
        self.meld(new_heap);
        ret
    }
    pub fn meld<'a>(&'a mut self, other: FibHeap<K,V>) -> &'a mut FibHeap<K,V> {
        if self.roots.is_empty() {
            self.roots.append(other.roots);
        } else if self.find_min().val0() <= other.find_min().val0() {
            self.roots.append(other.roots);
        } else {
            self.roots.prepend(other.roots);
        }
        self
    }
    pub fn find_min(&self) -> (K, V) {
        match self.roots.front() {
            Some(n) => (n.key.borrow().clone(), n.value.clone()),
            None => fail!("Fibonacci heap is empty")
        }
    }
    pub fn delete_min(&mut self) -> (K, V) {
        match self.roots.pop_front() {
            None => fail!("Fibonacci heap is empty"),
            Some(min_node) => {
                let return_value = (min_node.key.borrow().clone(), min_node.value.clone());

                // Add children of min node to root list.
                for n in min_node.children.borrow().iter() {
                    let mut parent = n.parent.borrow_mut();
                    *parent.deref_mut() = None
                }

                self.roots.append(min_node.children.borrow().clone());
                // Linking Step
                self.consolidate();

                // Find the new minimum root
                self.sort_roots();
                return_value
            }
        }
    }
    fn consolidate(&mut self) {
        if self.roots.len() < 2 {
            return
        }
        loop {
            match self.same_rank() {
                None => break,
                Some((n1, n2)) => {
                    if n1 < n2 {
                        self.link_and_insert(n1, n2);
                    } else {
                        self.link_and_insert(n2, n1);
                    }
                }
            }
        }
    }
    fn same_rank(&mut self) -> Option<(FibEntry<K,V>, FibEntry<K,V>)> {
        let node_to_check = self.roots.pop_front().unwrap();
        for _ in range(0, self.roots.len()) {
            if node_to_check.rank() == self.roots.front().unwrap().rank() {
                return Some((node_to_check, self.roots.pop_front().unwrap()));
            }
            self.roots.rotate_backward()
        }

        self.roots.push_front(node_to_check);
        None
    }
    fn link_and_insert(&mut self, root: FibEntry<K,V>, child: FibEntry<K,V>) {
        {
            let mut child_parent = child.parent.borrow_mut();
            *child_parent.deref_mut() = Some(root.clone().downgrade());
        }
        root.children.borrow_mut().push_front(child);
        self.roots.push_front(root);
    }
    fn sort_roots(&mut self) {
        if self.roots.len() < 1 {
            return
        }
       let mut min_node = self.roots.pop_front().unwrap();
       for _ in range(0, self.roots.len()) {
            if *self.roots.front().unwrap() < min_node {
                self.roots.push(min_node);
                min_node = self.roots.pop_front().unwrap();
                // Put the recently added node at front so that it will properly rotate backward.
                self.roots.rotate_forward();
            }
            self.roots.rotate_backward()
       }
       self.roots.push_front(min_node);
    }
}
