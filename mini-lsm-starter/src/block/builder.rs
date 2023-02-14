#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use super::Block;

/// Builds a block.
pub struct BlockBuilder {
    data: Vec<u8>,
    block_size: usize,
    offsets: Vec<u16>,
    next_offset: u16,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        BlockBuilder { 
            data: Vec::with_capacity(block_size),
            block_size,
            offsets: Vec::new(),
            next_offset: 0,
        }
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        let key_len = key.len() as u16;
        let key_len_bytes = key_len.to_be_bytes();
        let value_len = value.len() as u16;
        let value_len_bytes = value_len.to_be_bytes();
        
        if self.next_offset + key_len + value_len + 4 >= self.block_size as u16 {
            return false;
        }

        self.data.append(&mut key_len_bytes.to_vec());
        self.data.append(&mut key.to_vec());
        self.data.append(&mut value_len_bytes.to_vec());
        self.data.append(&mut value.to_vec());

        self.offsets.push(self.next_offset);
        self.next_offset = self.data.len() as u16;

        true
    }

    /// Check if there is no key-value pair in the block.
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        Block {
            data: self.data,
            offsets: self.offsets,
        }
    }
}
