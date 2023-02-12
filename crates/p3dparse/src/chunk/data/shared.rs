use bytes::{Buf, Bytes};

use crate::{chunk::types::ChunkType, result::Result};

use super::parse_trait::Parse;

pub type Vector2 = (f32, f32);
pub type Vector3 = (f32, f32, f32);
pub type Colour = (u8, u8, u8, u8);
pub type Quaternion = (f32, f32, f32, f32);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[allow(non_snake_case)]
pub struct Matrix {
    pub M11: f32,
    pub M12: f32,
    pub M13: f32,
    pub M14: f32,
    pub M21: f32,
    pub M22: f32,
    pub M23: f32,
    pub M24: f32,
    pub M31: f32,
    pub M32: f32,
    pub M33: f32,
    pub M34: f32,
    pub M41: f32,
    pub M42: f32,
    pub M43: f32,
    pub M44: f32,
}

impl Parse for Matrix {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Matrix {
            M11: bytes.get_f32_le(),
            M12: bytes.get_f32_le(),
            M13: bytes.get_f32_le(),
            M14: bytes.get_f32_le(),
            M21: bytes.get_f32_le(),
            M22: bytes.get_f32_le(),
            M23: bytes.get_f32_le(),
            M24: bytes.get_f32_le(),
            M31: bytes.get_f32_le(),
            M32: bytes.get_f32_le(),
            M33: bytes.get_f32_le(),
            M34: bytes.get_f32_le(),
            M41: bytes.get_f32_le(),
            M42: bytes.get_f32_le(),
            M43: bytes.get_f32_le(),
            M44: bytes.get_f32_le(),
        })
    }
}
