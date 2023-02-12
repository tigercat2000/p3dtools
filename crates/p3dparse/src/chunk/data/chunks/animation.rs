use crate::{
    chunk::{
        data::{helpers, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
            num_frames: bytes.get_f32_le(),
            frame_rate: bytes.get_f32_le(),
            cyclic: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            PC: bytes.get_u32_le(),
            PS2: bytes.get_u32_le(),
            XBOX: bytes.get_u32_le(),
            GC: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroup {
    pub group_id: u32,
    pub num_channels: u32,
}

impl Parse for AnimationGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroup {
            group_id: bytes.get_u32_le(),
            num_channels: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroupList {
    pub num_groups: u32,
}

impl Parse for AnimationGroupList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroupList {
            num_groups: bytes.get_u32_le(),
        })
    }
}
