/* An Undirected Graph implemenation.
 *
 * Chris Piraino
 *
 */
pub trait Graph<V,E> {
    fn adjacent(&self, x: &V, y: &V) -> bool;
    fn neighbors(&'a self, x: &V) -> Vec<&'a V>;
    fn vertex_edges(&'a self, x: &V) -> Vec<&'a E>;
    fn add_node(&self, x: &V);
    fn add_edge(&self, x: &V, y: &V);
    fn remove_edge(&self, x: &V, y: &V);
    fn vertices(&'a self) -> Vec<&'a V>;
    fn edges(&'a self) -> Vec<&'a E>;
}

mod test {
    #[test]
    fn it_works() {
    }
}
