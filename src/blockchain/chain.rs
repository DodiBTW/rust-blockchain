use crate::blockchain::block::Block;
#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, 0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String, timestamp: u64) {
        let last_block = self.blocks.last().expect("Chain should have at least one block");
        let new_block = Block::new(
            last_block.index + 1,
            timestamp,
            data,
            last_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let prev = &self.blocks[i - 1];

            if !current.is_valid() || current.prev_hash != prev.hash {
                return false;
            }
        }
        true
    }
}