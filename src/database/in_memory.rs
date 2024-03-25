use std::collections::HashSet;

use crate::block::Block;

use super::database::Database;

pub struct InMemoryDatabase {
    blocks: Vec<Block>,
    unspent_outputs: HashSet<(String, usize)>,
}

impl InMemoryDatabase {
    fn new() -> Self {
        InMemoryDatabase {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }
}

impl Database for InMemoryDatabase {
    fn insert_block(&mut self, block: Block) {
        println!("Inserting block into in-memory database.");
    }

    fn get_blocks(&self) -> &[Block] {
        &self.blocks
    }

    fn add_utxo(&mut self, tx_hash: String, output_index: usize) {
        self.unspent_outputs.insert((tx_hash, output_index));
    }

    fn remove_utxo(&mut self, tx_hash: &str, output_index: usize) {
        self.unspent_outputs
            .remove(&(tx_hash.to_string(), output_index));
    }

    fn is_utxo(&self, tx_hash: &str, output_index: usize) -> bool {
        self.unspent_outputs
            .contains(&(tx_hash.to_string(), output_index))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        block::Block,
        database::{database::Database, InMemoryDatabase},
    };

    #[test]
    fn test_insert_block() {
        let mut in_memory_db = InMemoryDatabase::new();
        in_memory_db.insert_block(Block::default());
        assert_eq!(in_memory_db.blocks.len(), 1)
    }
}
