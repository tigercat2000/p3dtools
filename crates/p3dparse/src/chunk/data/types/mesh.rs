use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{pure3d_read_string, read_colour, read_vec2, read_vec3},
            parse_trait::Parse,
            types::shared::{Colour, Vector2, Vector3},
        },
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Mesh {
    pub num_prim_groups: u32,
}

impl Parse for Mesh {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Mesh {
            num_prim_groups: bytes.safe_get_u32_le()?,
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
#[allow(clippy::upper_case_acronyms)]
#[repr(u32)]
pub enum PrimitiveType {
    TriangleList = 0x0,
    TriangleStrip = 0x1,
    LineList = 0x2,
    LineStrip = 0x3,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldPrimGroup {
    pub shader_name: String,
    pub primitive_type: PrimitiveType,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub num_matrices: u32,
    /// Bitfield of [`VertexType`]
    pub vertex_types: u32,
}

impl Parse for OldPrimGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldPrimGroup {
            shader_name: pure3d_read_string(bytes)?,
            primitive_type: bytes.safe_get_u32_le()?.try_into()?,
            num_vertices: bytes.safe_get_u32_le()?,
            num_indices: bytes.safe_get_u32_le()?,
            num_matrices: bytes.safe_get_u32_le()?,
            vertex_types: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PositionList {
    pub positions: Vec<Vector3>,
}

impl Parse for PositionList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut positions = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            positions.push(read_vec3(bytes)?);
        }

        Ok(PositionList { positions })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UVList {
    pub channel: u32,
    pub UVs: Vec<Vector2>,
}

impl Parse for UVList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;
        let channel = bytes.safe_get_u32_le()?;

        #[allow(non_snake_case)]
        let mut UVs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            UVs.push(read_vec2(bytes)?);
        }

        Ok(UVList { channel, UVs })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct NormalList {
    pub normals: Vec<Vector3>,
}

impl Parse for NormalList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut normals = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            normals.push(read_vec3(bytes)?);
        }

        Ok(NormalList { normals })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TangentList {
    pub tangents: Vec<Vector3>,
}

impl Parse for TangentList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut tangents = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            tangents.push(read_vec3(bytes)?);
        }

        Ok(TangentList { tangents })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BinormalList {
    pub binormals: Vec<Vector3>,
}

impl Parse for BinormalList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut binormals = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            binormals.push(read_vec3(bytes)?);
        }

        Ok(BinormalList { binormals })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PackedNormalList {
    pub normals: Vec<u8>,
}

impl Parse for PackedNormalList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut normals = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            normals.push(bytes.safe_get_u8()?);
        }

        Ok(PackedNormalList { normals })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ColourList {
    pub colours: Vec<Colour>,
}

impl Parse for ColourList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut colours = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            colours.push(read_colour(bytes)?);
        }

        Ok(ColourList { colours })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IndexList {
    pub indices: Vec<u32>,
}

impl Parse for IndexList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.safe_get_u32_le()? as usize;

        let mut indices = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            indices.push(bytes.safe_get_u32_le()?);
        }

        Ok(IndexList { indices })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RenderStatus {
    pub cast_shadow: u32,
}

impl Parse for RenderStatus {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(RenderStatus {
            cast_shadow: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawable {
    pub skeleton_name: String,
}

impl Parse for CompositeDrawable {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawable {
            skeleton_name: pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawableSkinList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawableSkinList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSkinList {
            num_elements: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawableSkin {
    pub is_translucent: u32,
}

impl Parse for CompositeDrawableSkin {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSkin {
            is_translucent: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawablePropList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawablePropList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawablePropList {
            num_elements: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawableProp {
    pub is_translucent: u32,
    pub skeleton_joint_id: u32,
}

impl Parse for CompositeDrawableProp {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableProp {
            is_translucent: bytes.safe_get_u32_le()?,
            skeleton_joint_id: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawableEffectList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawableEffectList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableEffectList {
            num_elements: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CompositeDrawableEffect {
    pub is_translucent: u32,
    pub skeleton_joint_id: u32,
}

impl Parse for CompositeDrawableEffect {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableEffect {
            is_translucent: bytes.safe_get_u32_le()?,
            skeleton_joint_id: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CompositeDrawableSortOrder {
    pub sort_order: f32,
}

impl Parse for CompositeDrawableSortOrder {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSortOrder {
            sort_order: bytes.safe_get_f32_le()?,
        })
    }
}
