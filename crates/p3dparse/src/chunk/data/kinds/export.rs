use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers, parse_trait::Parse},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct P3DExportInfoNamedString {
    value: String,
}

impl Parse for P3DExportInfoNamedString {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(P3DExportInfoNamedString {
            value: helpers::pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct P3DExportInfoNamedInt {
    value: u32,
}

impl Parse for P3DExportInfoNamedInt {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(P3DExportInfoNamedInt {
            value: bytes.safe_get_u32_le()?,
        })
    }
}
