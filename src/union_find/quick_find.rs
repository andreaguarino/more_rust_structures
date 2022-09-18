use core::panic;

use crate::union_find::UnionFind;

struct QuickFind {
    array: Vec<u32>,
}

impl UnionFind for QuickFind {
    fn union(&mut self, p: u32, q: u32) -> () {
        let p_value = self.array[p as usize];
        let q_value = self.array[q as usize];
        let mut index = 0;
        if p_value == q_value {
            return;
        }
        while index < self.array.len() {
            let elem = self.array[index];
            if elem == p_value {
                self.array[index] = q_value
            }
            index += 1;
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
        return self.array[p_index] == self.array[q_index];
    }

    fn find(&self, p: u32) -> u32 {
        return self.array[p as usize];
    }

    fn count(&self) -> usize {
        return self.array.len();
    }
}

impl QuickFind {
    fn new(n: u32) -> QuickFind {
        let mut i = 0;
        let mut array = vec![];
        while i < n {
            array.push(i);
            i += 1;
        }
        return QuickFind { array: array };
    }
}

#[cfg(test)]
mod tests {
    use crate::union_find::quick_find::{QuickFind, UnionFind};

    #[test]
    #[should_panic(expected = "The disjoint set is empty")]
    fn is_connected_empty_set() {
        let result = QuickFind::new(0);
        result.is_connected(42, 0);
    }

    #[test]
    fn is_connected_symmetric() {
        let mut set = QuickFind::new(5);
        set.union(3, 4);
        assert_eq!(set.is_connected(3, 4), true);
        assert_eq!(set.is_connected(4, 3), true);
        assert_eq!(set.is_connected(0, 3), false);
        assert_eq!(set.is_connected(3, 0), false);
    }

    #[test]
    fn is_connected_transitive() {
        let mut set = QuickFind::new(10);
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
        let set = QuickFind::new(7);
        assert_eq!(set.is_connected(1, 0), false);
        assert_eq!(set.is_connected(0, 0), true);
    }

    #[test]
    fn union() {
        let mut set = QuickFind::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.is_connected(0, 2), false);
    }

    #[test]
    fn find() {
        let mut set = QuickFind::new(7);
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.find(0), set.find(1));
        assert_eq!(set.is_connected(0, 2), false);
        assert_ne!(set.find(0), set.find(2));
    }

    #[test]
    fn count() {
        let set = QuickFind::new(7);
        assert_eq!(set.count(), 7);
    }
}
