use std::vec;

use crate::union_find::UnionFind;

pub struct WeightedQuickUnion {
    pub array: Vec<u32>,
    pub ranks: Vec<u32>,
}

impl UnionFind for WeightedQuickUnion {
    fn union(&mut self, p: u32, q: u32) -> () {
        let p_root = self.root(p);
        let q_root = self.root(q);
        let p_rank = self.ranks[p_root as usize];
        let q_rank = self.ranks[q_root as usize];
        if p_rank > q_rank {
            self.array[q_root as usize] = p_root;
            self.ranks[p_root as usize] = std::cmp::max(p_rank, q_rank + 1);
        } else {
            self.array[p_root as usize] = q_root;
            self.ranks[q_root as usize] = std::cmp::max(q_rank, p_rank + 1);
        }
    }

    fn is_connected(&self, p: u32, q: u32) -> bool {
        let p_index = p as usize;
        let q_index = q as usize;
        let len = self.array.len();
        if self.array.is_empty() {
            panic!("The disjoint set is empty")
        }
        if p_index >= len {
            panic!("p should be less than {:?}", len);
        }
        if q_index >= len {
            panic!("q should be less than {:?}", len);
        }
        return self.root(p) == self.root(q);
    }

    fn find(&self, p: u32) -> u32 {
        return self.root(p);
    }

    fn count(&self) -> usize {
        return self.array.len();
    }
}

impl WeightedQuickUnion {
    pub fn new(n: u32) -> WeightedQuickUnion {
        let mut i = 0;
        let mut array = vec![];
        let mut ranks = vec![];
        while i < n {
            array.push(i);
            ranks.push(0);
            i += 1;
        }
        return WeightedQuickUnion {
            array: array,
            ranks,
        };
    }

    fn root(&self, p: u32) -> u32 {
        let mut node = p;
        let mut parent = self.array[p as usize];
        while node != parent {
            node = parent;
            parent = self.array[node as usize];
        }
        return node;
    }
}

mod tests {
    use crate::{union_find::UnionFind, weighted_quick_union::WeightedQuickUnion};

    #[test]
    #[should_panic(expected = "The disjoint set is empty")]
    fn is_connected_empty_set() {
        let result = WeightedQuickUnion::new(0);
        result.is_connected(42, 0);
    }

    #[test]
    fn is_connected_symmetric() {
        let mut set = WeightedQuickUnion::new(5);
        set.union(3, 4);
        assert_eq!(set.is_connected(3, 4), true);
        assert_eq!(set.is_connected(4, 3), true);
        assert_eq!(set.is_connected(0, 3), false);
        assert_eq!(set.is_connected(3, 0), false);
    }

    #[test]
    fn is_connected_transitive() {
        let mut set = WeightedQuickUnion::new(10);
        assert_eq!(set.is_connected(0, 1), false);
        assert_eq!(set.is_connected(0, 9), false);
        set.union(0, 9);
        assert_eq!(set.is_connected(0, 9), true);
        assert_eq!(set.is_connected(0, 8), false);
        set.union(8, 9);
        assert_eq!(set.is_connected(0, 8), true);
        assert_eq!(set.is_connected(8, 9), true);
    }

    #[test]
    fn is_connected() {
        let set = WeightedQuickUnion::new(7);
        assert_eq!(set.is_connected(1, 0), false);
        assert_eq!(set.is_connected(0, 0), true);
    }

    #[test]
    fn union() {
        let mut set = WeightedQuickUnion::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.is_connected(0, 2), false);
    }

    #[test]
    fn find() {
        let mut set = WeightedQuickUnion::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.root(0), set.root(1));
        assert_eq!(set.is_connected(0, 2), false);
        assert_ne!(set.root(0), set.root(2));
    }

    #[test]
    fn count() {
        let set = WeightedQuickUnion::new(7);
        assert_eq!(set.count(), 7);
    }

    #[test]
    fn avoid_loops() {
        let mut set = WeightedQuickUnion::new(7);
        set.union(2, 3);
        set.union(3, 2);
        assert_eq!(set.is_connected(2, 3), true);
        assert_eq!(set.is_connected(3, 2), true);
    }

    #[test]
    fn ranks() {
        let mut set = WeightedQuickUnion::new(7);
        set.union(2, 3);
        assert_eq!(set.ranks[set.root(2) as usize], 1);
        assert_eq!(set.ranks[set.root(3) as usize], 1);
        set.union(3, 6);
        assert_eq!(set.ranks[set.root(3) as usize], 1);
        set.union(1, 5);
        set.union(1, 3);
        assert_eq!(set.ranks[set.root(1) as usize], 2);
    }
}
