use crate::{union_find::weighted_quick_union::WeightedQuickUnion, union_find::UnionFind};

struct WeightedQuickUnionWithPathCompression<WQU> {
    wqu: WQU,
}

impl UnionFind for WeightedQuickUnionWithPathCompression<WeightedQuickUnion> {
    fn union(&mut self, p: u32, q: u32) -> () {
        let p_root = self.root(p);
        let q_root = self.root(q);
        let p_rank = self.wqu.ranks[p_root as usize];
        let q_rank = self.wqu.ranks[q_root as usize];
        if p_rank > q_rank {
            self.wqu.array[q_root as usize] = p_root;
            self.wqu.ranks[p_root as usize] = std::cmp::max(p_rank, q_rank + 1);
        } else {
            self.wqu.array[p_root as usize] = q_root;
            self.wqu.ranks[q_root as usize] = std::cmp::max(q_rank, p_rank + 1);
        }
    }

    fn is_connected(&self, p: u32, q: u32) -> bool {
        return self.wqu.is_connected(p, q);
    }

    fn find(&self, p: u32) -> u32 {
        return self.wqu.find(p);
    }

    fn count(&self) -> usize {
        return self.wqu.count();
    }
}

impl WeightedQuickUnionWithPathCompression<WeightedQuickUnion> {
    fn new(n: u32) -> WeightedQuickUnionWithPathCompression<WeightedQuickUnion> {
        let wqu = WeightedQuickUnion::new(n);
        return WeightedQuickUnionWithPathCompression { wqu };
    }

    fn root(&mut self, p: u32) -> u32 {
        let mut node = p as usize;
        let mut parent = self.wqu.array[p as usize] as usize;
        while node != parent {
            self.wqu.array[node] = self.wqu.array[parent];
            node = parent;
            parent = self.wqu.array[node] as usize;
        }
        return node as u32;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        union_find::weighted_quick_union_with_path_compression::WeightedQuickUnionWithPathCompression,
        union_find::UnionFind,
    };

    #[test]
    #[should_panic(expected = "The disjoint set is empty")]
    fn is_connected_empty_set() {
        let result = WeightedQuickUnionWithPathCompression::new(0);
        result.is_connected(42, 0);
    }

    #[test]
    fn is_connected_symmetric() {
        let mut set = WeightedQuickUnionWithPathCompression::new(5);
        set.union(3, 4);
        assert_eq!(set.is_connected(3, 4), true);
        assert_eq!(set.is_connected(4, 3), true);
        assert_eq!(set.is_connected(0, 3), false);
        assert_eq!(set.is_connected(3, 0), false);
    }

    #[test]
    fn is_connected_transitive() {
        let mut set = WeightedQuickUnionWithPathCompression::new(10);
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
        let set = WeightedQuickUnionWithPathCompression::new(7);
        assert_eq!(set.is_connected(1, 0), false);
        assert_eq!(set.is_connected(0, 0), true);
    }

    #[test]
    fn union() {
        let mut set = WeightedQuickUnionWithPathCompression::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.is_connected(0, 2), false);
    }

    #[test]
    fn find() {
        let mut set = WeightedQuickUnionWithPathCompression::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.root(0), set.root(1));
        assert_eq!(set.is_connected(0, 2), false);
        assert_ne!(set.root(0), set.root(2));
    }

    #[test]
    fn count() {
        let set = WeightedQuickUnionWithPathCompression::new(7);
        assert_eq!(set.count(), 7);
    }

    #[test]
    fn avoid_loops() {
        let mut set = WeightedQuickUnionWithPathCompression::new(7);
        set.union(2, 3);
        set.union(3, 2);
        assert_eq!(set.is_connected(2, 3), true);
        assert_eq!(set.is_connected(3, 2), true);
    }

    #[test]
    fn ranks() {
        let mut set = WeightedQuickUnionWithPathCompression::new(7);
        set.union(2, 3);
        let root2 = set.root(2);
        let root3 = set.root(3);
        assert_eq!(set.wqu.ranks[root2 as usize], 1);
        assert_eq!(set.wqu.ranks[root3 as usize], 1);
        set.union(3, 6);
        let root3 = set.root(3);
        assert_eq!(set.wqu.ranks[root3 as usize], 1);
        set.union(1, 5);
        set.union(1, 3);
        let root1 = set.root(3);
        assert_eq!(set.wqu.ranks[root1 as usize], 2);
    }
}
