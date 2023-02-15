use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Animation {
    pub animation_type: String,
    pub num_frames: f32,
    pub frame_rate: f32,
    pub cyclic: u32,
}

impl Parse for Animation {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Animation {
            animation_type: helpers::pure3d_read_fourcc(bytes)?,
            num_frames: bytes.safe_get_f32_le()?,
            frame_rate: bytes.safe_get_f32_le()?,
            cyclic: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms, non_snake_case)]
pub struct AnimationSize {
    pub PC: u32,
    pub PS2: u32,
    pub XBOX: u32,
    pub GC: u32,
}

impl Parse for AnimationSize {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationSize {
            PC: bytes.safe_get_u32_le()?,
            PS2: bytes.safe_get_u32_le()?,
            XBOX: bytes.safe_get_u32_le()?,
            GC: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroup {
    pub group_id: u32,
    pub num_channels: u32,
}

impl Parse for AnimationGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroup {
            group_id: bytes.safe_get_u32_le()?,
            num_channels: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroupList {
    pub num_groups: u32,
}

impl Parse for AnimationGroupList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroupList {
            num_groups: bytes.safe_get_u32_le()?,
        })
    }
}
