pub mod data;
pub mod type_identifiers;

use std::fmt::Display;

use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{kinds::name::Name, ChunkData},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
/// Display format is Absolute Index:Relative Index
pub struct Span {
    pub absolute_index: usize,
    pub relative_index: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.absolute_index, self.relative_index)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Chunk {
    pub typ: ChunkType,
    pub data: ChunkData,
    pub span: Span,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Chunk {
    pub fn parse_root(bytes: &mut Bytes) -> Result<Vec<Chunk>> {
        let mut vec = Vec::new();

        Chunk::parse(bytes, &mut vec, None, 0)?;

        Ok(vec)
    }

    pub fn parse(
        bytes: &mut Bytes,
        vec: &mut Vec<Chunk>,
        parent: Option<usize>,
        relative_index: usize,
    ) -> Result<usize> {
        // Note: C# BinaryReader is always LE unless otherwise stated
        // First 4 bytes indicate the chunk type
        let typ: ChunkType = bytes.safe_get_u32_le()?.try_into()?;
        // Second 4 bytes indicate the data size including the 12 bytes for these fields
        let data_size = bytes.safe_get_u32_le()?;
        // Third 4 bytes indicate entire size including children
        let total_size = bytes.safe_get_u32_le()?;

        // If the data size is larger than the total size, this is corrupted.
        if data_size > total_size {
            return Err(eyre!("Data size is greater than total size"));
        }

        // We expect to read data_size - 12 bytes in data.
        let expected_parse_size = (data_size - 12) as usize;
        // So we'll only give that many bytes to the ChunkData parser.
        let mut data_slice = bytes.slice(0..expected_parse_size);
        let original_data_slice = data_slice.clone();
        // Let the data get parsed...
        let data = match ChunkData::from_chunk_type(typ, &mut data_slice) {
            Ok(data) => data,
            Err(e) => {
                let lineage = format!(
                    "Error: Could not parse data. Lineage Info: {}",
                    if let Some(parent) = parent {
                        vec.get(parent).unwrap().get_lineage(vec)
                    } else {
                        "Unknown".to_owned()
                    }
                );
                return Err(e.wrap_err(lineage));
            }
        };

        let index = vec.len();
        vec.push(Chunk {
            typ,
            children: Vec::new(),
            span: Span {
                absolute_index: index,
                relative_index,
            },
            data,
            parent,
        });

        let mut children = Vec::new();

        // It's okay if we fail to parse a chunk because we always know the size and can keep our framing intact.
        if !data_slice.is_empty() {
            // eprintln!(
            //     "Warning: Chunk {} was expected to parse {:?} bytes but only parsed {:?}, data result: {:?}",
            //     &chunk.get_lineage(), expected_parse_size, expected_parse_size - data_slice.len(), chunk.data
            // );
            let actually_consumed = expected_parse_size - data_slice.len();
            // Our original bytes slice has already consumed the 12 byte header, so we have to subtract it here
            // Potential Children Size can never be 0 because we know we have leftover data in the data slice.
            let potential_children_size = total_size as usize - actually_consumed - 12;
            // However, that does not mean we want to index further than what we have read
            let mut potential_children_slice =
                bytes.slice(actually_consumed..actually_consumed + potential_children_size);
            let original_potential_children_slice = potential_children_slice.clone();
            eprintln!(
                "Recovery: We expected to parse {} bytes but actually parsed {} bytes",
                expected_parse_size, actually_consumed
            );
            eprintln!(
                "Recovery: We will try to find children in the remaining {} bytes",
                potential_children_size
            );

            let mut child_count = 0;
            let mut parsed_so_far = 0;
            while parsed_so_far < potential_children_size {
                let before_parse = potential_children_slice.len();
                match Chunk::parse(&mut potential_children_slice, vec, Some(index), child_count) {
                    Ok(child) => {
                        eprintln!(
                            "Recovery: We sucessfully parsed a misaligned child at index {}",
                            child // vec.get(child).unwrap().get_lineage()
                        );
                        children.push(child);
                    }
                    Err(e) => {
                        // eprintln!(
                        //     "Recovery: We failed to parse a misaligned child for {}",
                        //     &chunk.get_lineage()
                        // );
                        eprintln!("Recovery: This was caused by: {:?}", e);
                        eprintln!("-- Full Data Hexdump --");
                        hexdump::hexdump(&original_data_slice);
                        eprintln!("-- Misaligned Data Hexdump --");
                        hexdump::hexdump(&original_potential_children_slice);
                        // If any child parsing fails then we just have to ignore the rest of these...
                        break;
                    }
                }
                let after_parse = potential_children_slice.len();
                parsed_so_far += before_parse - after_parse;
                child_count += 1;
            }
        }

        // We parsed something, maybe an Unknown chunk, we need to move past it to keep framing intact.
        bytes.safe_advance(expected_parse_size)?;

        // Only parse children if we didn't do so already in fallback
        if children.is_empty() {
            // Indicates we have some children to parse.
            if total_size > data_size {
                let total_children_size = (total_size - data_size) as usize;
                let mut parsed_so_far = 0;
                let mut child_count = 0;
                while parsed_so_far < total_children_size {
                    let before_parse = bytes.len();
                    let child = Chunk::parse(bytes, vec, Some(index), child_count)?;
                    children.push(child);
                    let after_parse = bytes.len();
                    parsed_so_far += before_parse - after_parse;
                    child_count += 1;
                }
            }
        }

        vec.get_mut(index).unwrap().children = children;

        Ok(index)
    }

    /// See [`Chunk::get_name`] for information about the span format.
    pub fn get_lineage(&self, vec: &[Chunk]) -> String {
        let mut string = self.get_name();
        let mut target: &Chunk = self;
        while let Some(parent) = target.parent {
            let parent = vec
                .get(parent)
                .expect("Invariant violated: Child thought it had a parent at an invalid index!");
            string.push_str(&format!(" -> {}", parent.get_name()));
            target = parent;
        }
        string
    }

    /// Display format is Name:Type:Absolute Index:Relative Index
    pub fn get_name(&self) -> String {
        format!(
            "{}:{:?}:{}",
            self.data.get_name().unwrap_or(Name("<no name>".into())).0,
            self.typ,
            self.span,
        )
    }

    pub fn children_len(&self) -> usize {
        self.children.len()
    }

    pub fn get_child<'a>(&self, vec: &'a [Chunk], index: usize) -> Result<&'a Chunk> {
        let child_index = self
            .children
            .get(index)
            .ok_or_else(|| eyre!("Invalid child index"))?;

        vec.iter()
            .enumerate()
            .find_map(|(index, chunk)| {
                if index == *child_index {
                    Some(chunk)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                panic!("Invariant violated: Chunk thought it had a child at an invalid index")
            })
    }

    pub fn get_children<'a>(&self, vec: &'a [Chunk]) -> impl Iterator<Item = &'a Chunk> + 'a {
        let children = self.children.clone();
        vec.iter().enumerate().filter_map(move |(index, chunk)| {
            if children.contains(&index) {
                Some(chunk)
            } else {
                None
            }
        })
    }

    pub fn get_children_of_type<'a>(
        &self,
        vec: &'a [Chunk],
        typ: ChunkType,
    ) -> impl Iterator<Item = &'a Chunk> + 'a {
        self.get_children(vec).filter(move |c| c.typ == typ)
    }
}

pub trait VecChunkExtension {
    fn get_root(&self) -> Result<&Chunk>;
}

impl VecChunkExtension for Vec<Chunk> {
    fn get_root(&self) -> Result<&Chunk> {
        self.get(0)
            .ok_or_else(|| eyre!("Vec does not contain root chunk"))
    }
}
