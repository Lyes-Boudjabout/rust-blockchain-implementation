use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: Vec::new() }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn get_latest_hash(&self) -> String {
        match self.chain.last() {
            Some(block) => block.hash.clone(),
            None => String::from("0"),
        }
    }
}