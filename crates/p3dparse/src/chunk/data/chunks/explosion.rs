use crate::{
    chunk::{data::parse_trait::Parse, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExplosionEffectData {
    pub typ: u32,
    pub count: u32,
}

impl Parse for ExplosionEffectData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ExplosionEffectData {
            typ: bytes.get_u32_le(),
            count: bytes.get_u32_le(),
        })
    }
}
