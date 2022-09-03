trait UnionFind {
    fn union(&self, p: u32, q: u32) -> ();
    fn is_connected(&self, p: u32, q: u32) -> bool;
    fn find(&self, p: u32) -> u32;
    fn count(&self) -> u32;
}
