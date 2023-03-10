use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers,
            kinds::shared::{Colour, Quaternion, Vector2, Vector3},
            parse_trait::Parse,
        },
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldBillboardQuad {
    pub billboard_mode: String,
    pub translation: Vector3,
    pub color: Colour,
    pub uv0: Vector2,
    pub uv1: Vector2,
    pub uv2: Vector2,
    pub uv3: Vector2,
    pub width: f32,
    pub height: f32,
    pub distance: f32,
    pub uv_offset: Vector2,
}

impl Parse for OldBillboardQuad {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBillboardQuad {
            billboard_mode: helpers::pure3d_read_fourcc(bytes)?,
            translation: helpers::read_vec3(bytes)?,
            color: helpers::read_colour(bytes)?,
            uv0: helpers::read_vec2(bytes)?,
            uv1: helpers::read_vec2(bytes)?,
            uv2: helpers::read_vec2(bytes)?,
            uv3: helpers::read_vec2(bytes)?,
            width: bytes.safe_get_f32_le()?,
            height: bytes.safe_get_f32_le()?,
            distance: bytes.safe_get_f32_le()?,
            uv_offset: helpers::read_vec2(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldBillboardQuadGroup {
    pub shader: String,
    pub ztest: u32,
    pub zwrite: u32,
    pub fog: u32,
    pub num_quads: u32,
}

impl Parse for OldBillboardQuadGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBillboardQuadGroup {
            shader: helpers::pure3d_read_string(bytes)?,
            ztest: bytes.safe_get_u32_le()?,
            zwrite: bytes.safe_get_u32_le()?,
            fog: bytes.safe_get_u32_le()?,
            num_quads: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldBillboardDisplayInfo {
    pub rotation: Quaternion,
    pub cut_off_mode: String,
    pub uv_offset_range: Vector2,
    pub source_range: f32,
    pub edge_range: f32,
}

impl Parse for OldBillboardDisplayInfo {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBillboardDisplayInfo {
            rotation: helpers::read_quaternion(bytes)?,
            cut_off_mode: helpers::pure3d_read_fourcc(bytes)?,
            uv_offset_range: helpers::read_vec2(bytes)?,
            source_range: bytes.safe_get_f32_le()?,
            edge_range: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldBillboardPerspectiveInfo {
    pub perspective: u32,
}

impl Parse for OldBillboardPerspectiveInfo {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBillboardPerspectiveInfo {
            perspective: bytes.safe_get_u32_le()?,
        })
    }
}
