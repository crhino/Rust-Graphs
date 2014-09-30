/* 
 * Graph Data Structure
 *
 * Adjacency List
 *
 *
 * Chris Piraino
 */
use super::{Graph, Edge};
use std::collections::HashMap;
use std::hash::Hash;

struct AdjList<V,E> {
    vertices: HashMap<V, Vec<E>>
}

impl<V: Hash + Eq, E: Edge<V>> Graph<V,E> for AdjList<V,E> {
    fn adjacent(&self, x: &V, y: &V) -> bool {
        let ref x_neighbors = self.vertices[*x];
        for e in x_neighbors.iter() {
            let (u, v) = e.endpoints();
            if *y == *u || *y == *v {
                return true
            }
        }
        false
    }
    
    fn neighbors<'a>(&'a self, x: &V) -> Vec<&'a V> {
        self.vertices[*x].iter().map(|v| {
            let (s, r) = v.endpoints();
            if *s == *x {
                r
            } else {
                s
            }
        }).collect()
    }

    fn vertex_edges<'a>(&'a self, x: &V) -> Vec<&'a E> {
        self.vertices[*x].iter().collect()
    }

    fn add_node(&mut self, x: V) {
        self.vertices.insert(x, Vec::new());
    }

    fn add_edge(&mut self, x: &V, y: &V) {

    }

    fn remove_edge(&mut self, x: &V, y: &V) {

    }

    fn vertices<'a>(&'a self) -> Vec<&'a V> {
        Vec::new()
    }

    fn edges<'a>(&'a self) -> Vec<&'a E> {
        Vec::new()
    }
}

