use crate::quick_find::UnionFind;

/// Implements union in `O(1)` and find in O(n)
struct QuickUnion {
    array: Vec<u32>,
}

impl UnionFind for QuickUnion {
    fn union(&mut self, p: u32, q: u32) -> () {
        let p_root = self.root(p);
        let q_root = self.root(q);
        self.array[p_root as usize] = q_root;
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

impl QuickUnion {
    fn new(n: u32) -> QuickUnion {
        let mut i = 0;
        let mut array = vec![];
        while i < n {
            array.push(i);
            i += 1;
        }
        return QuickUnion { array: array };
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
    use crate::{quick_find::UnionFind, quick_union::QuickUnion};

    #[test]
    #[should_panic(expected = "The disjoint set is empty")]
    fn is_connected_empty_set() {
        let result = QuickUnion::new(0);
        result.is_connected(42, 0);
    }

    #[test]
    fn is_connected_symmetric() {
        let mut set = QuickUnion::new(5);
        set.union(3, 4);
        assert_eq!(set.is_connected(3, 4), true);
        assert_eq!(set.is_connected(4, 3), true);
        assert_eq!(set.is_connected(0, 3), false);
        assert_eq!(set.is_connected(3, 0), false);
    }

    #[test]
    fn is_connected_transitive() {
        let mut set = QuickUnion::new(10);
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
        let set = QuickUnion::new(7);
        assert_eq!(set.is_connected(1, 0), false);
        assert_eq!(set.is_connected(0, 0), true);
    }

    #[test]
    fn union() {
        let mut set = QuickUnion::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.is_connected(0, 2), false);
    }

    #[test]
    fn find() {
        let mut set = QuickUnion::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.root(0), set.root(1));
        assert_eq!(set.is_connected(0, 2), false);
        assert_ne!(set.root(0), set.root(2));
    }

    #[test]
    fn count() {
        let set = QuickUnion::new(7);
        assert_eq!(set.count(), 7);
    }

    #[test]
    fn avoid_loops() {
        let mut set = QuickUnion::new(7);
        set.union(2, 3);
        set.union(3, 2);
        assert_eq!(set.is_connected(2, 3), true);
        assert_eq!(set.is_connected(3, 2), true);
    }
}
