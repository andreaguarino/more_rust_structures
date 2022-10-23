use std::path::Iter;

pub trait Dictionary<K, V> {
    fn get(&self, key: K) -> Option<&V>;
    fn put(&mut self, key: K, value: V) -> ();
    fn contains_key(&self, key: K) -> bool;
    fn delete(&self, key: K) -> Option<&V>;
    fn keys(&self) -> Keys;
    fn values(&self) -> Keys;
}

struct BST<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> BST<K, V> {
    fn new() -> Self {
        BST { root: None }
    }

    fn insert(root: &mut Box<Node<K, V>>, key: K, value: V) -> () {
        if key <= root.key {
            match &mut root.left {
                None => root.left = Some(Box::new(Node::new(key, value))),
                Some(node) => {
                    Self::insert(node, key, value);
                }
            }
        } else {
            match &mut root.right {
                None => root.right = Some(Box::new(Node::new(key, value))),
                Some(node) => {
                    Self::insert(node, key, value);
                }
            }
        }
    }

    fn _get(node: &Option<Box<Node<K, V>>>, key: K) -> Option<&V> {
        match node {
            None => None,
            Some(node) => match node {
                node if node.key == key => Some(&node.value),
                node if node.key < key => Self::_get(&node.left, key),
                node if node.key > key => Self::_get(&node.right, key),
                _ => panic!(""),
            },
        }
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

impl<K: Ord, V> Dictionary<K, V> for BST<K, V> {
    fn put(&mut self, key: K, value: V) -> () {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node::new(key, value)));
            }
            Some(root) => {
                BST::insert(root, key, value);
            }
        }
    }

    fn contains_key(&self, key: K) -> bool {
        todo!()
    }

    fn delete(&self, key: K) -> Option<&V> {
        todo!()
    }

    fn keys(&self) -> Keys {
        todo!()
    }

    fn values(&self) -> Keys {
        todo!()
    }

    fn get(&self, key: K) -> Option<&V> {
        Self::_get(&self.root, key)
    }
}

pub struct Keys<K, V> {
    root: &Box<Node<K, V>>,
}

impl<K, V> Iterator for Keys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::Dictionary;

    use super::BST;

    #[test]
    fn basic_bst() {
        let mut bst = BST::new();
        bst.put("foo", 42);
        assert_eq!(*bst.get("foo").unwrap(), 42);
    }
}
