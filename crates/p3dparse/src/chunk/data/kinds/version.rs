use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, type_identifiers::ChunkType},
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Version {
    pub version: u32,
}

impl Parse for Version {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Version {
            version: bytes.safe_get_u32_le()?,
        })
    }
}
