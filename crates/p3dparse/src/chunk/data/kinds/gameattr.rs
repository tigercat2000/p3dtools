use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{self, read_vec3},
            kinds::shared::{Colour, Matrix, Vector3},
            parse_trait::Parse,
        },
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GameAttr {
    pub num_params: u32,
}

impl Parse for GameAttr {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(GameAttr {
            num_params: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GameAttrParam {
    pub param: String,
    pub value: GameAttrParamValue,
}

impl Parse for GameAttrParam {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(GameAttrParam {
            param: helpers::pure3d_read_string(bytes)?,
            value: match typ {
                ChunkType::GameAttrIntParam => GameAttrParamValue::Int(bytes.safe_get_u32_le()?),
                ChunkType::GameAttrFloatParam => {
                    GameAttrParamValue::Float(bytes.safe_get_f32_le()?)
                }
                ChunkType::GameAttrColourParam => GameAttrParamValue::Colour((
                    bytes.safe_get_u8()?,
                    bytes.safe_get_u8()?,
                    bytes.safe_get_u8()?,
                    bytes.safe_get_u8()?,
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum GameAttrParamValue {
    Int(u32),
    Float(f32),
    Colour(Colour),
    Vector(Vector3),
    Matrix(Matrix),
    None,
}
