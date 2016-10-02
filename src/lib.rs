/// A type representing a mutable binary tree
pub enum BTree<K, V> {
    Empty,
    Node {
        key: K,
        val: V,
        left: Box<BTree<K, V>>,
        right: Box<BTree<K, V>>,
    },
}

#[cfg(test)]
mod tests {
}
