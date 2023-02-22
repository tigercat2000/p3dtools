use crate::{
    chunk::{
        data::{helpers::pure3d_read_string, parse_trait::Parse},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Name(pub String);

impl Parse for Name {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let name = pure3d_read_string(bytes)?;
        Ok(Name(name))
    }
}
