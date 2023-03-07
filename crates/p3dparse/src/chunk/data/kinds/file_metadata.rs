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
pub struct ExportInfoNamedString {
    value: String,
}

impl Parse for ExportInfoNamedString {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ExportInfoNamedString {
            value: helpers::pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExportInfoNamedInt {
    value: u32,
}

impl Parse for ExportInfoNamedInt {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ExportInfoNamedInt {
            value: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct History {
    history: Vec<String>,
}

impl Parse for History {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let mut history = History { history: vec![] };
        let num_lines = bytes.safe_get_u16_le()?;
        for _ in 0..num_lines {
            history.history.push(helpers::pure3d_read_string(bytes)?)
        }
        Ok(history)
    }
}
