use crate::{chunk::type_identifiers::ChunkType, Result};
use bytes::Bytes;

pub trait Parse {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self>
    where
        Self: Sized;
}
