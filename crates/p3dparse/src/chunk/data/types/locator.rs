use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::read_vec3,
            parse_trait::Parse,
            types::shared::{Matrix, Vector3},
        },
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WBLocator {
    pub typ: WBLocatorType,
    pub num_data_elements: u32,
    pub data: Vec<u32>,
    pub position: Vector3,
    pub num_triggers: u32,
}

impl Parse for WBLocator {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let typ = WBLocatorType::try_from(bytes.safe_get_u32_le()?)?;
        let num_data_elements = bytes.safe_get_u32_le()?;
        let mut data = Vec::with_capacity(num_data_elements as usize);
        for _ in 0..num_data_elements {
            data.push(bytes.safe_get_u32_le()?);
        }

        Ok(WBLocator {
            typ,
            num_data_elements,
            data,
            position: read_vec3(bytes)?,
            num_triggers: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[allow(non_snake_case)]
#[repr(u32)]
pub enum WBLocatorType {
    Event = 0,
    Script = 1,
    Generic = 2,
    CarStart = 3,
    Spline = 4,
    DynamicZone = 5,
    Occlusion = 6,
    InteriorEntrance = 7,
    Directional = 8,
    Action = 9,
    Fov = 10,
    BreakableCamera = 11,
    StaticCamera = 12,
    PedGroup = 13,
    Coin = 14,
    SpawnPoint = 15,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WBTriggerVolume {
    pub typ: u32,
    pub scale: Vector3,
    pub matrix: Matrix,
}

impl Parse for WBTriggerVolume {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(WBTriggerVolume {
            typ: bytes.safe_get_u32_le()?,
            scale: read_vec3(bytes)?,
            matrix: Matrix::parse(bytes, typ)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WBMatrix {
    pub matrix: Matrix,
}

impl Parse for WBMatrix {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(WBMatrix {
            matrix: Matrix::parse(bytes, typ)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WBSpline {
    pub num_CVs: u32,
    pub CVs: Vec<Vector3>,
}

impl Parse for WBSpline {
    #[allow(non_snake_case)]
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let num_CVs = bytes.safe_get_u32_le()?;
        let mut CVs = Vec::with_capacity(num_CVs as usize);
        for _ in 0..num_CVs {
            CVs.push(read_vec3(bytes)?);
        }
        Ok(WBSpline { num_CVs, CVs })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WBRail {
    pub behavior: u32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub track_rail: u32,
    pub track_dist: f32,
    pub reverse_sense: u32,
    pub fov: f32,
    pub target_offset: Vector3,
    pub axis_play: Vector3,
    pub position_lag: f32,
    pub target_lag: f32,
}

impl Parse for WBRail {
    #[allow(non_snake_case)]
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(WBRail {
            behavior: bytes.safe_get_u32_le()?,
            min_radius: bytes.safe_get_f32_le()?,
            max_radius: bytes.safe_get_f32_le()?,
            track_rail: bytes.safe_get_u32_le()?,
            track_dist: bytes.safe_get_f32_le()?,
            reverse_sense: bytes.safe_get_u32_le()?,
            fov: bytes.safe_get_f32_le()?,
            target_offset: read_vec3(bytes)?,
            axis_play: read_vec3(bytes)?,
            position_lag: bytes.safe_get_f32_le()?,
            target_lag: bytes.safe_get_f32_le()?,
        })
    }
}
