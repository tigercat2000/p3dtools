use crate::{
    chunk::{
        data::{common_types::Vector3, helpers, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshData {
    pub num_prim_groups: u32,
}

impl Parse for MeshData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(MeshData {
            num_prim_groups: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u32)]
pub enum PrimitiveType {
    TriangleList = 0x0,
    TriangleStrip = 0x1,
    LineList = 0x2,
    LineStrip = 0x3,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u32)]
#[allow(dead_code)]
pub enum VertexType {
    UVs = 0x0001,
    UVs2 = 0x0002,
    UVs3 = 0x0004,
    UVs4 = 0x0008,
    Normals = 0x0010,
    Colours = 0x0020,
    Matrices = 0x0080,
    Weights = 0x0100,
    Unknown = 0x2000,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OldPrimGroupData {
    pub shader_name: String,
    pub primitive_type: PrimitiveType,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub num_matrices: u32,
    /// Bitfield of [`VertexType`]
    pub vertex_types: u32,
}

impl Parse for OldPrimGroupData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldPrimGroupData {
            shader_name: helpers::pure3d_read_string(bytes)?,
            primitive_type: bytes.get_u32_le().try_into()?,
            num_vertices: bytes.get_u32_le(),
            num_indices: bytes.get_u32_le(),
            num_matrices: bytes.get_u32_le(),
            vertex_types: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositionListData {
    pub positions: Vec<Vector3>,
}

impl Parse for PositionListData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.get_u32_le() as usize;

        let mut positions = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            positions.push(helpers::read_vec3(bytes)?);
        }

        Ok(PositionListData { positions })
    }
}
