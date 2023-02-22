use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, type_identifiers::ChunkType},
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub alpha_depth: u32,
    pub num_mip_maps: u32,
    pub texture_type: u32,
    pub usage: u32,
    pub priority: u32,
}

impl Parse for Texture {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Texture {
            width: bytes.safe_get_u32_le()?,
            height: bytes.safe_get_u32_le()?,
            bpp: bytes.safe_get_u32_le()?,
            alpha_depth: bytes.safe_get_u32_le()?,
            num_mip_maps: bytes.safe_get_u32_le()?,
            texture_type: bytes.safe_get_u32_le()?,
            usage: bytes.safe_get_u32_le()?,
            priority: bytes.safe_get_u32_le()?,
        })
    }
}
