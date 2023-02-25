use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers::read_vec3, kinds::shared::Vector3, parse_trait::Parse},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct P3DCamera {
    pub FOV: f32,
    pub aspect_ratio: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub position: Vector3,
    pub look: Vector3,
    pub up: Vector3,
}

impl Parse for P3DCamera {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(P3DCamera {
            FOV: bytes.safe_get_f32_le()?,
            aspect_ratio: bytes.safe_get_f32_le()?,
            near_clip: bytes.safe_get_f32_le()?,
            far_clip: bytes.safe_get_f32_le()?,
            position: read_vec3(bytes)?,
            look: read_vec3(bytes)?,
            up: read_vec3(bytes)?,
        })
    }
}
