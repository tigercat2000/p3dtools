use crate::{
    chunk::{data::parse_trait::Parse, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplosionEffect {
    pub typ: u32,
    pub count: u32,
}

impl Parse for ExplosionEffect {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ExplosionEffect {
            typ: bytes.get_u32_le(),
            count: bytes.get_u32_le(),
        })
    }
}
