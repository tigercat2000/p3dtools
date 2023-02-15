use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{pure3d_read_string, read_vec3},
            parse_trait::Parse,
            types::shared::Vector3,
        },
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BoundingBox {
    pub low: Vector3,
    pub high: Vector3,
}

impl Parse for BoundingBox {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(BoundingBox {
            low: read_vec3(bytes)?,
            high: read_vec3(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BoundingSphere {
    pub centre: Vector3,
    pub radius: f32,
}

impl Parse for BoundingSphere {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(BoundingSphere {
            centre: read_vec3(bytes)?,
            radius: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PhysicsObject {
    pub material_name: String,
    pub num_joints: u32,
    pub volume: f32,
    pub resting_sensitivity: f32,
}

impl Parse for PhysicsObject {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(PhysicsObject {
            material_name: pure3d_read_string(bytes)?,
            num_joints: bytes.safe_get_u32_le()?,
            volume: bytes.safe_get_f32_le()?,
            resting_sensitivity: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PhysicsJoint {
    pub index: u32,
    pub volume: f32,
    pub stiffness: f32,
    pub max_angle: f32,
    pub min_angle: f32,
    pub dof: u32,
}

impl Parse for PhysicsJoint {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(PhysicsJoint {
            index: bytes.safe_get_u32_le()?,
            volume: bytes.safe_get_f32_le()?,
            stiffness: bytes.safe_get_f32_le()?,
            max_angle: bytes.safe_get_f32_le()?,
            min_angle: bytes.safe_get_f32_le()?,
            dof: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PhysicsVector {
    pub vector: Vector3,
}

impl Parse for PhysicsVector {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(PhysicsVector {
            vector: read_vec3(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PhysicsInertiaMatrix {
    pub X: Vector3,
    pub YY: f32,
    pub YZ: f32,
    pub ZZ: f32,
}

impl Parse for PhysicsInertiaMatrix {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(PhysicsInertiaMatrix {
            X: read_vec3(bytes)?,
            YY: bytes.safe_get_f32_le()?,
            YZ: bytes.safe_get_f32_le()?,
            ZZ: bytes.safe_get_f32_le()?,
        })
    }
}
