use crate::{
    chunk::{
        data::{
            helpers::{self, read_vec3},
            parse_trait::Parse,
            types::shared::{Colour, Matrix, Vector3},
        },
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameAttr {
    pub num_params: u32,
}

impl Parse for GameAttr {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(GameAttr {
            num_params: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GameAttrParam {
    pub param: String,
    pub value: GameAttrParamValue,
}

impl Parse for GameAttrParam {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(GameAttrParam {
            param: helpers::pure3d_read_string(bytes)?,
            value: match typ {
                ChunkType::GameAttrIntParam => GameAttrParamValue::Int(bytes.get_u32_le()),
                ChunkType::GameAttrFloatParam => GameAttrParamValue::Float(bytes.get_f32_le()),
                ChunkType::GameAttrColourParam => GameAttrParamValue::Colour((
                    bytes.get_u8(),
                    bytes.get_u8(),
                    bytes.get_u8(),
                    bytes.get_u8(),
                )),
                ChunkType::GameAttrVectorParam => GameAttrParamValue::Vector(read_vec3(bytes)?),
                ChunkType::GameAttrMatrixParam => {
                    GameAttrParamValue::Matrix(Matrix::parse(bytes, typ)?)
                }
                _ => GameAttrParamValue::None,
            },
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum GameAttrParamValue {
    Int(u32),
    Float(f32),
    Colour(Colour),
    Vector(Vector3),
    Matrix(Matrix),
    None,
}
