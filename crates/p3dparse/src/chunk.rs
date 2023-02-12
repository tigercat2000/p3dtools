pub mod data;
pub mod types;

use self::{data::ChunkData, types::ChunkType};
use crate::Result;
use bytes::{Buf, Bytes};
use eyre::eyre;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Chunk {
    pub typ: ChunkType,
    pub children: Vec<Chunk>,
    pub data: ChunkData,
}

impl Chunk {
    pub fn parse(bytes: &mut Bytes) -> Result<Chunk> {
        // Note: C# BinaryReader is always LE unless otherwise stated
        // First 4 bytes indicate the chunk type
        let typ: ChunkType = bytes.get_u32_le().try_into()?;
        // Second 4 bytes indicate the data size including the 12 bytes for these fields
        let data_size = bytes.get_u32_le();
        // Third 4 bytes indicate entire size including children
        let total_size = bytes.get_u32_le();

        // If the data size is larger than the total size, this is corrupted.
        if data_size > total_size {
            return Err(eyre!("Data size is greater than total size"));
        }

        eprintln!(
            "Parsing chunk of type {:?}, data size: {}, children size: {}",
            typ,
            data_size,
            total_size - data_size
        );

        // We expect to read data_size - 12 bytes in data.
        let expected_parse_size = (data_size - 12) as usize;
        // So we'll only give that many bytes to the ChunkData parser.
        let mut data_slice = bytes.slice(0..expected_parse_size);
        // Let the data get parsed...
        let data = ChunkData::from_chunk_type(typ, &mut data_slice)?;
        // It's okay if we fail to parse a chunk because we always know the size and can keep our framing intact.
        if !data_slice.is_empty() {
            eprintln!(
                "Warning: Chunk {:?} was expected to parse {:?} bytes but only parsed {:?}, skipping this chunk's data",
                typ, expected_parse_size, expected_parse_size - data_slice.len()
            );
        }
        // We parsed something, maybe an Unknown chunk, we need to move past it to keep framing intact.
        bytes.advance(expected_parse_size);

        let mut children = Vec::new();
        // Indicates we have some children to parse.
        if total_size > data_size {
            let total_children_size = (total_size - data_size) as usize;
            let mut parsed_so_far = 0;
            while parsed_so_far < total_children_size {
                let before_parse = bytes.len();
                children.push(Chunk::parse(bytes)?);
                let after_parse = bytes.len();
                parsed_so_far += before_parse - after_parse;
                // println!(
                //     "Parsed {}/{} children bytes",
                //     parsed_so_far, total_children_size
                // );
            }
        }

        Ok(Chunk {
            typ,
            children,
            data,
        })
    }
}
