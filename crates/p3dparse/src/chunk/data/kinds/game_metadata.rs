use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers, kinds::shared::Vector3, parse_trait::Parse},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Locator {
    position: Vector3,
}

impl Parse for Locator {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Locator {
            position: helpers::read_vec3(bytes)?,
        })
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct FollowCameraData {
    id: u32,

    rotation: f32,
    elevation: f32,
    magnitude: f32,

    target_offset: Vector3,
}

impl Parse for FollowCameraData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(FollowCameraData {
            id: bytes.safe_get_u32_le()?,
            rotation: bytes.safe_get_f32_le()?,
            elevation: bytes.safe_get_f32_le()?,
            magnitude: bytes.safe_get_f32_le()?,
            target_offset: helpers::read_vec3(bytes)?,
        })
    }
}
