mod quick_find;
mod quick_union;
mod weighted_quick_union;
mod weighted_quick_union_with_path_compression;

/// A data structure modelling a collection of sets of non-negative consecutive integer values 0..k-1, where set can be
/// easily merged together and values can be easily checked for membership to the same set.
/// ### Remarks
/// Disjoint sets are effective when dealing with equivalence relationships, i.e. relationships which are reflexive,
/// symmetric and transitive.
/// It's not directly usable when relationships have direction (e.g. if A points to B, B doesn't necessarily points to
/// A).
pub trait UnionFind {
    /// Establishes a "union" relashionship between the integer values `p` and `q`
    fn union(&mut self, p: u32, q: u32) -> ();
    /// Whether the two provided integer values belong to the same set, or two disjoint sets.
    fn is_connected(&self, p: u32, q: u32) -> bool;
    /// Returns the set identifier of the provided value
    fn find(&self, p: u32) -> u32;
    /// The number of integer values in the data structure.
    fn count(&self) -> usize;
}
