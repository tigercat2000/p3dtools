use super::super::parse_trait::Parse;
use crate::{
    chunk::{data::helpers, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shader {
    pub pddi_shader_name: String,
    pub has_translucency: u32,
    pub vertex_needs: u32,
    pub vertex_mask: u32,
    pub num_params: u32,
}

impl Parse for Shader {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Shader {
            pddi_shader_name: helpers::pure3d_read_string(bytes)?,
            has_translucency: bytes.get_u32_le(),
            vertex_needs: bytes.get_u32_le(),
            vertex_mask: bytes.get_u32_le(),
            num_params: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
