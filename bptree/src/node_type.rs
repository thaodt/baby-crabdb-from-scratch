use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(pub String);

#[derive(Debug, Clone, Eq)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

impl KVPair {
    pub fn new(key: String, value: String) -> Self {
        KVPair { key, value }
    }
}

impl PartialEq for KVPair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Ord for KVPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for KVPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents the type of a node in the B+ tree.
pub enum NodeType {
    /// A node that can have children, contain a vector of pointers to their children and a vector of keys.
    Internal(Vec<String>, Vec<Key>),
    /// this node type contains a vector of key-value pairs.
    Leaf(Vec<KVPair>),
    Unknown,
}
