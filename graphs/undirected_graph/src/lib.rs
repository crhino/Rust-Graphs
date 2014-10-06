/* An Undirected Graph implemenation.
 *
 * Chris Piraino
 *
 */
pub trait Graph<V,E> {
    fn adjacent<'a>(&self, x: &V, y: &V) -> bool;
    fn neighbors<'a>(&'a self, x: &V) -> Vec<&'a V>;
    fn vertex_edges<'a>(&'a self, x: &V) -> Vec<&'a E>;
    fn add_node<'a>(&mut self, x: V);
    fn add_edge(&mut self, u: &V, v: &V);
    fn remove_edge<'a>(&mut self, x: &V, y: &V);
    fn vertices<'a>(&'a self) -> Vec<&'a V>;
    fn edges<'a>(&'a self) -> Vec<&'a E>;
    fn new() -> Self;
}

pub trait Edge<V> {
    fn endpoints<'a>(&'a self) -> (&'a V, &'a V);
    fn new(x: &V, y: &V) -> Self;
}

pub mod adj_list;

mod test {
    #[test]
    fn it_works() {
    }
}
