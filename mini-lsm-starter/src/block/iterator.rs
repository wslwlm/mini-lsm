#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

// use core::slice::SlicePattern;
use std::{sync::Arc, ops::Deref};

use super::Block;

const KEY_LEN_BYTES: usize = 2;
const VALUE_LEN_BYTES: usize = 2;

/// Iterates on a block.
pub struct BlockIterator {
    block: Arc<Block>,
    key: Vec<u8>,
    value: Vec<u8>,
    idx: usize,
}

impl BlockIterator {
    fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            key: Vec::new(),
            value: Vec::new(),
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first entry.
    pub fn create_and_seek_to_first(block: Arc<Block>) -> Self {
        let mut block = BlockIterator::new(block);
        block.seek_to_first();
        block
    }

    /// Creates a block iterator and seek to the first key that >= `key`.
    pub fn create_and_seek_to_key(block: Arc<Block>, key: &[u8]) -> Self {
        let mut block_iter = BlockIterator::create_and_seek_to_first(block);
        block_iter.seek_to_key(key);
        block_iter
    }

    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {
        self.key.as_slice()
    }

    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {
        self.value.as_slice()
    }

    /// Returns true if the iterator is valid.
    pub fn is_valid(&self) -> bool {
        !self.key().is_empty()
    }

    fn extract_key_value(&self, index: usize) -> (Vec<u8>, Vec<u8>) {
        let offset = self.block.offsets[index];
        let key_len_start = offset as usize;
        let key_len_end = key_len_start + KEY_LEN_BYTES;
        let encode_key_len = &self.block.data[key_len_start .. key_len_end];
        let key_len = u16::from_be_bytes(encode_key_len.try_into().unwrap());

        let key_start = key_len_start + KEY_LEN_BYTES;
        let key_end = key_start + key_len as usize;

        let encode_value_len = &self.block.data[key_end..(key_end + VALUE_LEN_BYTES)];
        let value_len = u16::from_be_bytes(encode_value_len.try_into().unwrap());
        let value_start = key_end + VALUE_LEN_BYTES;
        let value_end = value_start + value_len as usize;

        (self.block.data[key_start .. key_end].to_vec(), self.block.data[value_start .. value_end].to_vec())
    }

    /// Seeks to the first key in the block.
    pub fn seek_to_first(&mut self) {
        if let Some(first) = self.block.offsets.first() {
            self.idx = 0;
            let (key, value) = self.extract_key_value(self.idx);
            self.key = key;
            self.value = value;
        }
    }

    /// Move to the next key in the block.
    pub fn next(&mut self) {
        self.idx += 1;
        assert_ne!(self.block.offsets.last(), None);
        let num_entries = self.block.offsets.len();
        // At the end of the Block
        if self.idx == num_entries {
            self.key = Vec::new();
            self.value = Vec::new();
        } else {
            let (key, value) = self.extract_key_value(self.idx);
            self.key = key.to_vec();
            self.value = value.to_vec();
        }
    }

    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        self.seek_to_first();
        while self.is_valid() && self.key.deref() < key {
            self.next();
        }
    }
}
