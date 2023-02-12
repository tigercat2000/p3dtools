use crate::{
    chunk::{
        data::{helpers, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AnimationData {
    pub animation_type: String,
    pub num_frames: f32,
    pub frame_rate: f32,
    pub cyclic: u32,
}

impl Parse for AnimationData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationData {
            animation_type: helpers::pure3d_read_fourcc(bytes)?,
            num_frames: bytes.get_f32_le(),
            frame_rate: bytes.get_f32_le(),
            cyclic: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms, non_snake_case)]
pub struct AnimationSizeData {
    pub PC: u32,
    pub PS2: u32,
    pub XBOX: u32,
    pub GC: u32,
}

impl Parse for AnimationSizeData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationSizeData {
            PC: bytes.get_u32_le(),
            PS2: bytes.get_u32_le(),
            XBOX: bytes.get_u32_le(),
            GC: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroupData {
    pub group_id: u32,
    pub num_channels: u32,
}

impl Parse for AnimationGroupData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroupData {
            group_id: bytes.get_u32_le(),
            num_channels: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct AnimationGroupListData {
    pub num_groups: u32,
}

impl Parse for AnimationGroupListData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimationGroupListData {
            num_groups: bytes.get_u32_le(),
        })
    }
}
