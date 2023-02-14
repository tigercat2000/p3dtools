use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers::pure3d_read_string, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct StatePropDataV1 {
    pub object_factory_name: String,
    pub num_states: u32,
}

impl Parse for StatePropDataV1 {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropDataV1 {
            object_factory_name: pure3d_read_string(bytes)?,
            num_states: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct StatePropStateDataV1 {
    pub auto_transition: u32,
    pub out_state: u32,
    pub num_drawable: u32,
    pub num_frame_controllers: u32,
    pub num_events: u32,
    pub num_callbacks: u32,
    pub out_frames: f32,
}

impl Parse for StatePropStateDataV1 {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropStateDataV1 {
            auto_transition: bytes.safe_get_u32_le()?,
            out_state: bytes.safe_get_u32_le()?,
            num_drawable: bytes.safe_get_u32_le()?,
            num_frame_controllers: bytes.safe_get_u32_le()?,
            num_events: bytes.safe_get_u32_le()?,
            num_callbacks: bytes.safe_get_u32_le()?,
            out_frames: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct StatePropVisibilitiesData {
    pub visible: u32,
}

impl Parse for StatePropVisibilitiesData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropVisibilitiesData {
            visible: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct StatePropFrameControllerData {
    pub cyclic: u32,
    pub num_cycles: u32,
    pub hold_frame: u32,
    pub min_frame: f32,
    pub max_frame: f32,
    pub relative_speed: f32,
}

impl Parse for StatePropFrameControllerData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropFrameControllerData {
            cyclic: bytes.safe_get_u32_le()?,
            num_cycles: bytes.safe_get_u32_le()?,
            hold_frame: bytes.safe_get_u32_le()?,
            min_frame: bytes.safe_get_f32_le()?,
            max_frame: bytes.safe_get_f32_le()?,
            relative_speed: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct StatePropEventData {
    pub state: u32,
    pub event_enum: i32,
}

impl Parse for StatePropEventData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropEventData {
            state: bytes.safe_get_u32_le()?,
            event_enum: bytes.safe_get_i32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct StatePropCallbackData {
    pub event_enum: i32,
    pub on_frame: f32,
}

impl Parse for StatePropCallbackData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(StatePropCallbackData {
            event_enum: bytes.safe_get_i32_le()?,
            on_frame: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ObjectAttributes {
    pub class_type: u32,
    pub phy_prop_id: u32,
    pub sound: String,
}

impl Parse for ObjectAttributes {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ObjectAttributes {
            class_type: bytes.safe_get_u32_le()?,
            phy_prop_id: bytes.safe_get_u32_le()?,
            sound: pure3d_read_string(bytes)?,
        })
    }
}
