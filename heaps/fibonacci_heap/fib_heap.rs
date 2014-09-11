/*
 * Chris Piraino
 *
 * Fibonacci Heap
 */
#![crate_name = "fib_heap"]
#![crate_type = "lib"]

extern crate collections;
extern crate core;
use core::fmt;
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
    fn remove_child(&mut self, child: Self);
}

impl<K,V: PartialEq> FibHeapEntry for FibEntry<K,V> {
    fn rank(&self) -> uint {
        self.children.borrow().len()
    }
    fn remove_child(&mut self, child: FibEntry<K,V>) {
        let mut children = self.children.borrow_mut();
        let deref = children.deref_mut();
        for _ in range(0, deref.len()) {
            if *deref.front().unwrap() == child {
                deref.pop_front();
                return
            }
            deref.rotate_backward();
        }
        fail!("Child was not found in parent.")
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

impl<K: fmt::Show, V: fmt::Show> fmt::Show for FibNode<K,V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "key: {}, value: {}\n", *self.key.borrow().deref(), self.value));
        try!(write!(f, "***Children***\n"));
        for n in self.children.borrow().deref().iter() {
            try!(write!(f, "{}", n));
        }
        write!(f, "***Children End***\n")
    }
}

impl<K: PartialOrd,V: PartialEq> PartialOrd for FibNode<K,V> {
    fn partial_cmp(&self, other: &FibNode<K,V>) -> Option<Ordering> {
        self.key.borrow().partial_cmp(other.key.borrow().deref())
    }
}

impl<K,V: PartialEq> PartialEq for FibNode<K,V> {
    fn eq(&self, other: &FibNode<K,V>) -> bool {
        self.value.eq(&other.value)
    }
}

#[deriving(Show)]
pub struct FibHeap<K,V> {
    // The minimum element is always contained at the top of the first root.
    roots: DList<FibEntry<K,V>>,
    total: int
}

impl<K: PartialOrd + Clone + Sub<K,K>, V: PartialEq + Clone> FibHeap<K,V> {
    pub fn new() -> FibHeap<K,V> {
        FibHeap { roots: DList::new(), total: 0 }
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
        self.total = self.total + 1;
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

                self.add_children_to_roots(min_node);
                // Linking Step
                self.consolidate();

                // Find the new minimum root
                self.sort_roots();
                self.total = self.total - 1;
                return_value
            }
        }
    }
    fn add_children_to_roots(&mut self, node: FibEntry<K,V>) {
        // Add children of min node to root list.
        for n in node.children.borrow().iter() {
            let mut parent = n.parent.borrow_mut();
            *parent.deref_mut() = None
        }

        self.roots.append(node.children.borrow().clone());
    }
    fn consolidate(&mut self) {
        // The maximum rank of a FibHeap is O(log n).
        let log_n = (self.total as f64).log2() as uint + 1;
        let mut rank_vec = Vec::from_fn(log_n, |_| -> Option<FibEntry<K,V>> { None });
        loop {
            match self.roots.pop_front() {
                Some(node) => {
                    insert_by_rank(&mut rank_vec, node);
                }
                None => break
            }
        }
        for n in rank_vec.move_iter() {
            if n.is_some() {
                self.roots.push(n.unwrap());
            }
        }
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
    pub fn decrease_key(&mut self, node: FibEntry<K,V>, delta: K) {
        {
            let mut key = node.key.borrow_mut();
            *key.deref_mut() =*key.deref() - delta;
        }
        match node.parent.borrow().deref() {
            &Some(ref weak_parent) => {
                match weak_parent.upgrade() {
                    Some(parent) => {
                        self.cascading_cut(parent, node.clone());
                    }
                    None => fail!("parent node has been dropped already.")
                }
            }
            &None => {
                self.remove_root(node.clone())
            }
        }
        {
            let mut parent = node.parent.borrow_mut();
            *parent.deref_mut() = None;
        }
        self.add_root(node);
    }
    pub fn delete(&mut self, node: FibEntry<K,V>) {
        if node == *self.roots.front().unwrap() {
            self.delete_min();
        } else {
            self.add_children_to_roots(node.clone());
            match node.parent.borrow().deref() {
                &Some(ref weak_parent) => {
                    match weak_parent.upgrade() {
                        Some(parent) => {
                            self.cascading_cut(parent, node.clone());
                        }
                        None => fail!("parent node has been dropped already.")
                    }
                }
                &None => {
                    self.remove_root(node.clone())
                }
            }
        }
    }
    fn remove_root(&mut self, node: FibEntry<K,V>) {
        for _ in range(0, self.roots.len()) {
            if node == *self.roots.front().unwrap() {
                self.roots.pop_front();
                break
            }
            self.roots.rotate_backward();
        }
    }
    fn cascading_cut(&mut self, mut parent: FibEntry<K,V>, child: FibEntry<K,V>) {
        parent.remove_child(child.clone());

        match parent.parent.borrow().deref() {
            &Some(ref weak_grandpa) => {
                match weak_grandpa.upgrade() {
                    Some(grandparent) => {
                        if parent.marked.get() {
                            self.cascading_cut(grandparent, parent.clone());
                            self.add_root(parent.clone());
                        } else {
                            parent.marked.set(true);
                        }
                    }
                    None => fail!("parent node has been dropped already.")
                }
            }
            // parent is a root.
            &None => return
        }
    }
    fn add_root(&mut self, node: FibEntry<K,V>) {
        if *self.roots.front().unwrap() <= node {
            self.roots.push(node);
        } else {
            self.roots.push_front(node);
        }
    }
}

fn link_and_insert<K: PartialOrd,V: PartialEq>(rank_vec: &mut Vec<Option<FibEntry<K,V>>>,
                   root: FibEntry<K,V>, child: FibEntry<K,V>) {
    {
        let mut child_parent = child.parent.borrow_mut();
        *child_parent.deref_mut() = Some(root.clone().downgrade());
        child.marked.set(false);

    }
    root.children.borrow_mut().push_front(child);
    insert_by_rank(rank_vec, root);
}

fn insert_by_rank<K: PartialOrd,V: PartialEq>(rank_vec: &mut Vec<Option<FibEntry<K,V>>>, node: FibEntry<K,V>) {
    let rank = node.rank();
    if (*rank_vec)[rank].is_none() {
        *rank_vec.get_mut(rank) = Some(node);
        return
    }
    rank_vec.push(None);
    let other = rank_vec.swap_remove(rank).unwrap().unwrap();
    if node < other {
        link_and_insert(rank_vec, node, other);
    } else {
        link_and_insert(rank_vec, other, node);
    }
}

#[cfg(test)]
#[allow(warnings)]
mod test {
    use super::{FibHeap};
    use test;

    #[test]
    fn fheap_insert() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        let one = fheap.insert(1, 1);
        let two = fheap.insert(2, 2);
        assert_eq!(*one.key.borrow().deref(), 1);
        assert_eq!(*two.key.borrow().deref(), 2);
        assert_eq!(fheap.find_min(), (1, 1));
        let zero = fheap.insert(0, 0);
        assert_eq!(*zero.key.borrow().deref(), 0);
        assert_eq!(fheap.find_min(), (0, 0));
    }
    #[test]
    fn fheap_meld() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        let zero = fheap.insert(1, 1);
        let one = fheap.insert(4, 4);
        let two = fheap.insert(2, 2);
        let mut fheap1: FibHeap<int, int> = FibHeap::new();
        fheap1.insert(1, 1);
        fheap1.insert(0, 0);
        fheap1.insert(3, 3);
        let fheap_deref = fheap.meld(fheap1);
        assert_eq!(fheap_deref.find_min(), (0, 0));
    }
    #[test]
    fn fheap_delete_min() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        let one = fheap.insert(1, 1);
        let two = fheap.insert(2, 2);
        let three = fheap.insert(3, 3);
        let four = fheap.insert(4, 4);
        fheap.insert(5, 5);
        assert_eq!(fheap.find_min(), (1, 1));
        let zero = fheap.insert(0, 0);
        assert_eq!(fheap.find_min(), (0, 0));
        assert_eq!(fheap.delete_min(), (0, 0));
        assert_eq!(fheap.delete_min(), (1, 1));
        assert_eq!(fheap.delete_min(), (2, 2));
    }
    #[test]
    fn test_fheap_decrease_key() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        fheap.insert(1, 1);
        let four = fheap.insert(4, 4);
        fheap.insert(0, 0);
        let five = fheap.insert(5, 5);
        fheap.delete_min();
        assert_eq!(fheap.roots.len(), 2);
        fheap.decrease_key(four.clone(), 2);
        assert_eq!(*four.key.borrow().deref(), 2);
        assert!(four.parent.borrow().deref().is_none());
        assert_eq!(fheap.roots.len(), 3);
        fheap.decrease_key(five.clone(), 5);
        assert_eq!(fheap.roots.len(), 3);
        assert_eq!(fheap.find_min(), (0, 5))
    }
    #[test]
    fn test_fheap_delete() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        let one = fheap.insert(1, 1);
        fheap.insert(4, 4);
        fheap.insert(0, 0);
        let five = fheap.insert(5, 5);
        fheap.delete_min();
        fheap.delete(five);
        assert_eq!(fheap.roots.len(), 1);
        fheap.delete(one);
        assert_eq!(fheap.roots.len(), 1);
        assert_eq!(fheap.find_min(), (4, 4))
    }
    #[test]
    fn test_fheap_cascading_cut() {
        let mut fheap: FibHeap<int, int> = FibHeap::new();
        fheap.insert(0, 0);
        fheap.insert(1, 1);
        fheap.insert(4, 4);
        fheap.insert(5, 5);
        fheap.insert(2, 2);
        fheap.insert(3, 3);
        let h6 = fheap.insert(6, 6);
        let h7 = fheap.insert(7, 7);
        fheap.insert(18, 18);
        fheap.insert(9, 9);
        fheap.insert(11, 11);
        fheap.insert(15, 15);
        fheap.delete_min();
        assert_eq!(fheap.find_min(), (1, 1));
        assert_eq!(fheap.roots.len(), 3);
        fheap.decrease_key(h6, 2);
        assert_eq!(fheap.roots.len(), 4);
        fheap.decrease_key(h7, 3);
        assert_eq!(fheap.roots.len(), 6);
    }
}
