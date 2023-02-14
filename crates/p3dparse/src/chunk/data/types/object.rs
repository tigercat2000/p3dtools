use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{pure3d_read_fourcc, pure3d_read_string},
            parse_trait::Parse,
        },
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnimatedObjectFactory {
    pub factory_name: String,
    pub num_animations: u32,
}

impl Parse for AnimatedObjectFactory {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimatedObjectFactory {
            factory_name: pure3d_read_string(bytes)?,
            num_animations: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnimatedObject {
    pub factory_name: String,
    pub starting_animation: u32,
}

impl Parse for AnimatedObject {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimatedObject {
            factory_name: pure3d_read_string(bytes)?,
            starting_animation: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AnimatedObjectAnimation {
    pub frame_rate: f32,
    pub num_old_frame_controllers: u32,
}

impl Parse for AnimatedObjectAnimation {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimatedObjectAnimation {
            frame_rate: bytes.safe_get_f32_le()?,
            num_old_frame_controllers: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OldFrameController {
    pub type2: String,
    pub frame_offset: f32,
    pub hierarchy_name: String,
    pub animation_name: String,
}

impl Parse for OldFrameController {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldFrameController {
            type2: pure3d_read_fourcc(bytes)?,
            frame_offset: bytes.safe_get_f32_le()?,
            hierarchy_name: pure3d_read_string(bytes)?,
            animation_name: pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiController {
    pub length: f32,
    pub frame_rate: f32,
    pub num_tracks: u32,
}

impl Parse for MultiController {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(MultiController {
            length: bytes.safe_get_f32_le()?,
            frame_rate: bytes.safe_get_f32_le()?,
            num_tracks: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiControllerTracks {
    pub tracks: Vec<MultiControllerTrack>,
}

impl Parse for MultiControllerTracks {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut tracks = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            tracks.push(MultiControllerTrack::parse(bytes, typ)?);
        }

        Ok(MultiControllerTracks { tracks })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MultiControllerTrack {
    pub name: String,
    pub start_time: f32,
    pub end_time: f32,
    pub scale: f32,
}

impl Parse for MultiControllerTrack {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(MultiControllerTrack {
            name: pure3d_read_string(bytes)?,
            start_time: bytes.safe_get_f32_le()?,
            end_time: bytes.safe_get_f32_le()?,
            scale: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectDSG {
    pub render_order: u32,
}

impl Parse for ObjectDSG {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ObjectDSG {
            render_order: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnimatedObjectDSGWrapper {
    pub version: u8,
    pub has_alpha: u8,
}

impl Parse for AnimatedObjectDSGWrapper {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(AnimatedObjectDSGWrapper {
            version: bytes.safe_get_u8()?,
            has_alpha: bytes.safe_get_u8()?,
        })
    }
}
