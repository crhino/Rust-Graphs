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

pub struct AdjList<V,E> {
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

    fn add_edge(&mut self, u: &V, v: &V) {
        let edge = Edge::new(u, v);
        let edge2 = Edge::new(u, v);
        self.vertices.get_mut(u).push(edge);
        self.vertices.get_mut(v).push(edge2);
    }

    fn remove_edge(&mut self, x: &V, y: &V) {
        self.remove_edge_from_hash(x, y);
        self.remove_edge_from_hash(y, x);
    }

    fn vertices<'a>(&'a self) -> Vec<&'a V> {
        self.vertices.keys().collect()
    }

    fn edges<'a>(&'a self) -> Vec<&'a E> {
        let edges: Vec<&'a E> = Vec::new();
        self.vertices.values().fold(edges, |mut init, vec| {
            init.extend(vec.iter());
            init
        })
    }

    fn new() -> AdjList<V,E> {
        AdjList { vertices: HashMap::new() }
    }
}

impl<V: Hash + Eq, E: Edge<V>> AdjList<V,E> {
    fn remove_edge_from_hash(&mut self, x: &V, y: &V) {
        self.vertices.get_mut(x).retain(|e| {
            match e.endpoints() {
                (u, v) => {
                    if *u != *y && *v != *y {
                        true
                    } else {
                        false
                    }
                }
            }
        });
    }
}

#[cfg(test)]
#[allow(warnings)]
mod test {
    use test;
    use super::AdjList;
    use super::super::{Graph, Edge};

    #[deriving(PartialEq, Show)]
    struct TestEdge<V> {
        source: V,
        target: V
    }

    impl<V: Clone> Edge<V> for TestEdge<V> {
        fn new(x: &V, y: &V) -> TestEdge<V> {
            TestEdge { source: x.clone(), target: y.clone() }
        }

        fn endpoints<'a>(&'a self) -> (&'a V, &'a V) {
            (&self.source, &self.target)
        }
    }

    #[test]
    fn test_adjacent() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&0i, &3i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        assert!(graph.adjacent(&0i, &1i));
        assert!(graph.adjacent(&0i, &3i));
        assert!(graph.adjacent(&2i, &3i));
        assert!(graph.adjacent(&1i, &2i));
    }

    #[test]
    fn test_neighbors() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_node(4i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&0i, &3i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        graph.add_edge(&1i, &4i);
        let mut neighbors = graph.neighbors(&1i);
        assert_eq!(neighbors.len(), 3);
        println!("{}", neighbors);
        neighbors.sort();
        println!("{}", neighbors);
        let expected = vec!(0i, 2i, 4i);
        let mut exp_iter = expected.iter();
        for i in neighbors.iter() {
            assert_eq!(*i, exp_iter.next().unwrap());
        }
    }

    #[test]
    fn test_vertex_edges() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_node(4i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&0i, &3i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        graph.add_edge(&1i, &4i);
        let edge0 = Edge::new(&0i, &1i);
        let edge2 = Edge::new(&1i, &2i);
        let edge4 = Edge::new(&1i, &4i);
        let mut edges = graph.vertex_edges(&1i);
        assert_eq!(edges.len(), 3);
        println!("{}", edges);
        let expected = vec!(edge0, edge2, edge4);
        let mut exp_iter = expected.iter();
        for i in edges.iter() {
            assert_eq!(*i, exp_iter.next().unwrap());
        }
    }

    #[test]
    fn test_remove_edge() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_node(4i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&0i, &3i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        graph.add_edge(&1i, &4i);
        graph.remove_edge(&1i, &2i);
        let mut neighbors = graph.neighbors(&1i);
        neighbors.sort();
        assert_eq!(neighbors.len(), 2);
        println!("{}", neighbors);
        println!("{}", neighbors);
        let expected = vec!(0i, 4i);
        let mut exp_iter = expected.iter();
        for i in neighbors.iter() {
            assert_eq!(*i, exp_iter.next().unwrap());
        }
    }

    #[test]
    fn test_vertices() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_node(4i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&0i, &3i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        graph.add_edge(&1i, &4i);
        let mut vertices = graph.vertices();
        assert_eq!(vertices.len(), 5);
        vertices.sort();
        println!("{}", vertices);
        let expected = vec!(0i, 1i, 2i, 3i, 4i);
        let mut exp_iter = expected.iter();
        for i in vertices.iter() {
            assert_eq!(*i, exp_iter.next().unwrap());
        }
    }

    #[test]
    fn test_edges() {
        let mut graph: AdjList<int, TestEdge<int>> = Graph::new();
        graph.add_node(0i);
        graph.add_node(1i);
        graph.add_node(2i);
        graph.add_node(3i);
        graph.add_node(4i);
        graph.add_edge(&0i, &1i);
        graph.add_edge(&2i, &3i);
        graph.add_edge(&1i, &2i);
        graph.add_edge(&1i, &4i);
        let edge0 = Edge::new(&0i, &1i);
        let edge2 = Edge::new(&1i, &2i);
        let edge3 = Edge::new(&2i, &3i);
        let edge4 = Edge::new(&1i, &4i);
        let mut edges = graph.edges();
        println!("{}", edges);
        assert_eq!(edges.len(), 4);
        let expected = vec!(edge0, edge3, edge2, edge4);
        let mut exp_iter = expected.iter();
        for i in edges.iter() {
            assert_eq!(*i, exp_iter.next().unwrap());
        }
    }
}
