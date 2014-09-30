/* 
 * Graph Data Structure
 *
 * Adjacency List
 *
 *
 * Chris Piraino
 */
use std::collections::HashMap;

struct AdjList<V,E> {
    vertices: HashMap<V, Vec<E>>
}

impl<V: Hash + Eq, E> Graph<V,E> for AdjList<V,E> {
    fn adjacent(&self, x: &V, y: &V) -> bool {

    }
    
    fn neighbors(&'a self, x: &V) -> Vec<&'a V> {

    }

    fn vertex_edges(&'a self, x: &V) -> Vec<&'a E> {

    }

    fn add_node(&self, x: &V) {

    }

    fn add_edge(&self, x: &V, y: &V) {

    }

    fn remove_edge(&self, x: &V, y: &V) {

    }

    fn vertices(&'a self) -> Vec<&'a V> {

    }

    fn edges(&'a self) -> Vec<&'a E> {

    }
}

