use core::panic;

trait UnionFind {
    fn union(&mut self, p: u32, q: u32) -> ();
    fn is_connected(&self, p: u32, q: u32) -> bool;
    fn find(&self, p: u32) -> u32;
    fn count(&self) -> u32;
}

struct DisjointSet {
    array: Vec<u32>,
}

impl UnionFind for DisjointSet {
    fn union(&mut self, p: u32, q: u32) -> () {
        let p_value = self.array[p as usize];
        let q_value = self.array[q as usize];
        let mut index = 0;
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
        if p_index > len {
            panic!("p should be less than {:?}", len);
        }
        if q_index > len {
            panic!("q should be less than {:?}", len);
        }
        return self.array[p_index] == self.array[q_index];
    }

    fn find(&self, p: u32) -> u32 {
        todo!()
    }

    fn count(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::union_find::{DisjointSet, UnionFind};

    #[test]
    #[should_panic(expected = "The disjoint set is empty")]
    fn is_connected_empty_set() {
        let result = DisjointSet { array: vec![] };
        result.is_connected(42, 0);
    }

    #[test]
    fn is_connected() {
        let set = DisjointSet {
            array: vec![0, 1, 2, 3, 4, 5, 6],
        };
        assert_eq!(set.is_connected(1, 0), false);
        assert_eq!(set.is_connected(0, 0), true);
    }

    #[test]
    fn union() {
        let mut set = DisjointSet {
            array: vec![0, 1, 2, 3, 4, 5, 6],
        };
        set.union(0, 1);
        assert_eq!(set.is_connected(0, 1), true);
        assert_eq!(set.is_connected(0, 2), false);
    }
}
