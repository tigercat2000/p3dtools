use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers, kinds::mesh::VertexType, parse_trait::Parse},
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Shader {
    pub pddi_shader_name: String,
    pub has_translucency: u32,
    pub vertex_needs: VertexType,
    pub vertex_mask: VertexType,
    pub num_params: u32,
}

impl Parse for Shader {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Shader {
            pddi_shader_name: helpers::pure3d_read_string(bytes)?,
            has_translucency: bytes.safe_get_u32_le()?,
            vertex_needs: bytes.safe_get_u32_le()?.try_into()?,
            vertex_mask: bytes.safe_get_u32_le()?.try_into()?,
            num_params: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VertexShader {
    pub vertex_shader_name: String,
}

impl Parse for VertexShader {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(VertexShader {
            vertex_shader_name: helpers::pure3d_read_string(bytes)?,
        })
    }
}
