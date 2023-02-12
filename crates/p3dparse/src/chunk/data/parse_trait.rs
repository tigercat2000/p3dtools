use crate::{chunk::types::ChunkType, Result};
use bytes::Bytes;

pub trait Parse {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self>
    where
        Self: Sized;
}
