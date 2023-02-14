#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::Bytes;
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut v = Vec::with_capacity(self.data.len());
        v.append(&mut self.data.clone());
        let num_entries = self.offsets.len() as u16;
        let encode_num_entries = num_entries.to_be_bytes();
        for &offset in self.offsets.iter() {
            let encode_offset = offset.to_be_bytes();
            v.append(&mut encode_offset.to_vec());
        }
        v.append(&mut encode_num_entries.to_vec());

        Bytes::from(v)
    }

    pub fn decode(data: &[u8]) -> Self {
        let data_len = data.len() as u16;
        let encode_num_entries = &data[((data_len - 2) as usize) ..];
        let num_entries = u16::from_be_bytes(encode_num_entries.try_into().unwrap());
        println!("num entries: {}", num_entries);

        let mut offsets = Vec::with_capacity(num_entries as usize);
        for i in (1..=num_entries).rev() {
            let start = (data_len - 2 * (i + 1)) as usize;
            let end = (data_len - 2 * i) as usize;
            let encode_offset = &data[start..end];
            let offset = u16::from_be_bytes(encode_offset.try_into().unwrap());
            println!("offset: {}", offset);
            offsets.push(offset);
        }

        let data_end = (data_len - 2 * (num_entries + 1)) as usize;
        let kv_data = data[..data_end].to_vec();

        Block { 
            data: kv_data, 
            offsets 
        }
    }
}

#[cfg(test)]
mod tests;
