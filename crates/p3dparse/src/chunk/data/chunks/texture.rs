use super::super::parse_trait::Parse;
use crate::{chunk::types::ChunkType, Result};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub alpha_depth: u32,
    pub num_mip_maps: u32,
    pub texture_type: u32,
    pub usage: u32,
    pub priority: u32,
}

impl Parse for TextureData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(TextureData {
            width: bytes.get_u32_le(),
            height: bytes.get_u32_le(),
            bpp: bytes.get_u32_le(),
            alpha_depth: bytes.get_u32_le(),
            num_mip_maps: bytes.get_u32_le(),
            texture_type: bytes.get_u32_le(),
            usage: bytes.get_u32_le(),
            priority: bytes.get_u32_le(),
        })
    }
}
