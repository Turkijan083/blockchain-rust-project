use std::sync::{Arc, Mutex};

use crate::blockchain::block::{Block, BlockHash};

pub type BlockVec = Vec<Block>;

// We don't need to export this because concurrency is encapsulated in this file
type SyncedBlockVec = Arc<Mutex<BlockVec>>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: SyncedBlockVec,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        // add the genesis block to the synced vec of blocks
        let mut blocks = BlockVec::default();
        blocks.push(genesis_block);
        let synced_blocks =  Arc::new(Mutex::new(blocks));

        let blockchain = Blockchain {
            blocks: synced_blocks,
        };

        return blockchain;
    }

    pub fn get_last_block(&self) -> Block {
        let blocks = self.blocks.lock().unwrap();
        let last_block = blocks[blocks.len() - 1].clone();

        return last_block;
    }

    pub fn get_all_blocks(&self) -> BlockVec {
        let blocks = self.blocks.lock().unwrap();
        return blocks.clone();
    }

    pub fn add_block(&self, block: Block) {
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[blocks.len() - 1];
 
        // check that the index is valid
        if block.index != last.index + 1 {
            panic!("Invalid index for new block {}.", block.index);
        }

        // check that the previous_hash is valid
        if block.previous_hash != last.hash {
            panic!("Invalid previous_hash for new block {}.", block.previous_hash);
        }

        // check that the hash matches the data
        if block.hash != block.calculate_hash() {
            panic!("Invalid hash for new block {}.", block.hash);
        }

        // append the block to the end
        blocks.push(block.clone());
    }

    fn create_genesis_block() -> Block {
        let index = 0;
        let nonce = 0;
        let previous_hash = BlockHash::default();
        let transactions = Vec::new();

        Block::new(index, nonce, previous_hash, transactions)
    }
}

impl Default for Blockchain {
    fn default() -> Self { Blockchain::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_valid_genesis_block() {
        let blockchain = Blockchain::new();

        // check that a new blockchain has one and only one block
        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 1);    

        // check that the last block is in the blockchain
        let block = blockchain.get_last_block();
        assert_eq!(block.hash, blocks[0].hash);

        // check that the genesis block has valid values
        assert_eq!(block.index, 0);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.previous_hash, BlockHash::default());
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn should_let_adding_valid_blocks() {
        let blockchain = Blockchain::new();

        // create a valid block
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(1, 0, previous_hash, Vec::new());

        // add it to the blockchain and check it was really added
        blockchain.add_block(block.clone());

        let blocks = blockchain.get_all_blocks();
        assert_eq!(blocks.len(), 2);

        let last_block = blockchain.get_last_block();
        assert_eq!(last_block.hash, block.hash);
    }

    #[test]
    #[should_panic]
    fn should_not_let_adding_block_with_invalid_index() {
        let blockchain = Blockchain::new();

        // create a block with invalid index
        let invalid_index = 2;
        let previous_hash = blockchain.get_last_block().hash;
        let block = Block::new(invalid_index, 0, previous_hash, Vec::new());

        // try adding the invalid block, it should panic
        blockchain.add_block(block.clone());
    }

    #[test]
    #[should_panic]
    fn should_not_let_adding_block_with_invalid_previous_hash() {
        let blockchain = Blockchain::new();

        // create a block with invalid previous hash
        let invalid_previous_hash = BlockHash::default();
        let block = Block::new(1, 0, invalid_previous_hash, Vec::new());

        // try adding the invalid block, it should panic
        blockchain.add_block(block.clone());
    }

    #[test]
    #[should_panic]
    fn should_not_let_adding_block_with_invalid_hash() {
        let blockchain = Blockchain::new();

        // create a block with invalid hash
        let previous_hash = blockchain.get_last_block().hash;
        let mut block = Block::new(1, 0, previous_hash, Vec::new());
        block.hash = BlockHash::default();

        // try adding the invalid block, it should panic
        blockchain.add_block(block.clone());
    }
}