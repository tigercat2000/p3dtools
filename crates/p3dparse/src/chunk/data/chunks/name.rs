use super::super::parse_trait::Parse;
use crate::{
    chunk::{data::helpers::pure3d_read_string, types::ChunkType},
    Result,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameData {
    pub name: String,
}

impl Parse for NameData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let name = pure3d_read_string(bytes)?;
        Ok(NameData { name })
    }
}
