struct BPNode {
    data: Vec<u8>, // can be dumped to the disk
}
const BP_NODE_NODE: i32 = 1; // internal nodes without values
const BP_NODE_LEAF: i32 = 2; // leaf nodes with values

struct BPTree {
    /// pointer (a nonzero page number)
    root: u64,
}

/// callbacks for managing on-disk pages
impl BPTree {
    /// allocate a new page
    fn new() -> Self {
        BPTree { root: 0 }
    }
    fn insert(&mut self, key: &[u8], value: &[u8]) {
        // TODO
    }
    /// dereference a pointer
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // TODO
        None
    }
    /// deallocate a page
    fn del(&mut self, key: &[u8]) {
        // TODO
    }
}

fn main() {
    println!("Hello, world!");
}
