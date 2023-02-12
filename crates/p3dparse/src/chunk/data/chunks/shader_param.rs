use super::super::parse_trait::Parse;
use crate::{
    chunk::{data::helpers, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ShaderParam {
    pub param: String,
    pub value: ShaderParamValue,
}

impl Parse for ShaderParam {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(ShaderParam {
            param: helpers::pure3d_read_fourcc(bytes)?,
            value: match typ {
                ChunkType::ShaderTextureParam => {
                    ShaderParamValue::Texture(helpers::pure3d_read_string(bytes)?)
                }
                ChunkType::ShaderIntParam => ShaderParamValue::Int(bytes.get_u32_le()),
                ChunkType::ShaderFloatParam => ShaderParamValue::Float(bytes.get_f32_le()),
                ChunkType::ShaderColourParam => ShaderParamValue::Colour((
                    bytes.get_u8(),
                    bytes.get_u8(),
                    bytes.get_u8(),
                    bytes.get_u8(),
                )),
                _ => ShaderParamValue::None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ShaderParamValue {
    Texture(String),
    Int(u32),
    Float(f32),
    Colour((u8, u8, u8, u8)),
    None,
}
