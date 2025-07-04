use crate::blockchain::block::Block;
#[derive(Clone,Debug)]
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

    pub fn create_block(&mut self, data: String, timestamp: u64) {
        let last_block = self.blocks.last().expect("Chain should have at least one block");
        let new_block = Block::new(
            last_block.index + 1,
            timestamp,
            data,
            last_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    pub fn add_block(&mut self, block : &Block) -> bool{
        let previous_block : Block = match self.blocks.last(){
            Some(block) => block.clone(),
            None => return false,
        };
        if block.prev_hash != previous_block.hash {
            if block.timestamp < previous_block.timestamp {
                // We need to replace our chain because we're outdated
                self.blocks.pop();
                self.blocks.push(block.clone());
                self.blocks.push(previous_block.clone());
                return true;
            }
            else{
                return false;
            }
        }
        self.blocks.push(block.clone());
        true
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
    pub fn contains(&self, block: &Block) -> bool {
        self.blocks.iter().any(|b| b.hash == block.hash)
    }
}