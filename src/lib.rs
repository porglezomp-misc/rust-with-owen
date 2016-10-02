/// A type representing a mutable binary tree
#[derive(PartialEq, Debug)]
pub enum BTree<K, V> where K: Ord {
    Empty,
    Node {
        key: K,
        val: V,
        left: Box<BTree<K, V>>,
        right: Box<BTree<K, V>>,
    },
}

impl<K, V> BTree<K, V> where K: Ord {
    pub fn node(key: K, val: V) -> Self {
        BTree::Node {
            key: key,
            val: val,
            left: Box::new(BTree::Empty),
            right: Box::new(BTree::Empty),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        match *self {
            BTree::Empty => *self = BTree::node(key, val),
            BTree::Node { key: ref k0, val: ref mut v0, ref mut left, ref mut right } => {
                if key < *k0 {
                    left.insert(key, val);
                } else if key > *k0 {
                    right.insert(key, val);
                } else {
                    *v0 = val;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BTree;

    #[test]
    fn test_insert_into_empty() {
        let mut tree = BTree::Empty;
        tree.insert(1, 42);
        assert_eq!(tree, BTree::node(1, 42));
    }

    #[test]
    fn test_insert_into_singleton() {
        let mut tree = BTree::node(1, 42);
        tree.insert(0, 13);
        assert_eq!(tree, BTree::Node {
            key: 1,
            val: 42,
            left: Box::new(BTree::node(0, 13)),
            right: Box::new(BTree::Empty),
        });

        tree = BTree::node(1, 42);
        tree.insert(2, 13);
        assert_eq!(tree, BTree::Node {
            key: 1,
            val: 42,
            left: Box::new(BTree::Empty),
            right: Box::new(BTree::node(2, 13)),
        });
    }
}
