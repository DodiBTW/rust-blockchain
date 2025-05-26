use sha2::{Sha256, Digest};
use crate::network::chain::ProtoBlock;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, data: String, prev_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}
impl From<ProtoBlock> for Block {
    fn from(proto: ProtoBlock) -> Self {
        Block {
            index: proto.index,
            timestamp: proto.timestamp,
            data: proto.data,
            prev_hash: proto.prev_hash,
            hash: proto.hash,
        }
    }
}

impl From<&Block> for ProtoBlock {
    fn from(block: &Block) -> Self {
        ProtoBlock {
            index: block.index,
            timestamp: block.timestamp,
            data: block.data.clone(),
            prev_hash: block.prev_hash.clone(),
            hash: block.hash.clone(),
        }
    }
}

