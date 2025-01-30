use super::pager::Pager;
use serde::{Deserialize, Serialize};
use std::mem;

const ORDER: usize = 4; // B-tree order

#[derive(Debug, Serialize, Deserialize)]
pub struct BTreeNode {
    pub keys: Vec<i32>,
    pub children: Vec<u64>, // Page IDs of child nodes
    pub is_leaf: bool,
}

impl BTreeNode {
    pub fn new(is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::with_capacity(ORDER - 1),
            children: Vec::with_capacity(ORDER),
            is_leaf,
        }
    }

    pub fn is_full(&self) -> bool {
        self.keys.len() >= ORDER - 1
    }
}

pub struct BTree {
    root_page_id: u64,
    pager: Pager,
}

impl BTree {
    pub fn new(pager: Pager) -> Self {
        let root = BTreeNode::new(true);
        let root_page_id = 0;
        pager.serialize_page(root_page_id, &root).unwrap();
        BTree { root_page_id, pager }
    }

    pub fn insert(&mut self, key: i32) -> Result<(), DbError> {
        let root = self.pager.get_page(self.root_page_id)?;
        if root.is_full() {
            // Split root and grow the tree
            let new_root = BTreeNode::new(false);
            let new_root_page_id = self.allocate_page(new_root)?;
            // ... (split logic)
        }
        self.insert_non_leaf(self.root_page_id, key)
    }

    fn insert_non_leaf(&mut self, page_id: u64, key: i32) -> Result<(), DbError> {
        // ... (recursive insertion)
        Ok(())
    }

    fn allocate_page(&mut self, node: BTreeNode) -> Result<u64, DbError> {
        let page_id = self.pager.num_pages;
        self.pager.serialize_page(page_id, &node)?;
        Ok(page_id)
    }
}
