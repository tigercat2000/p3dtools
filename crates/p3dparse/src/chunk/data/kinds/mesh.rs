use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{pure3d_read_string, read_colour, read_vec2, read_vec3},
            kinds::shared::{Colour, Vector2, Vector3},
            parse_trait::Parse,
        },
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use modular_bitfield::prelude::*;
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

#[bitfield(bits = 32)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub struct VertexType {
    pub uv_count: B4,
    // next 4
    pub has_normal: bool,
    pub has_colour: bool,
    pub has_specular: bool,
    pub has_indices: bool,
    // next 4
    pub has_weight: bool,
    pub has_size: bool,
    pub has_w: bool,
    pub has_binormal: bool,
    // next 3
    pub has_tangent: bool,
    pub has_position: bool,
    pub has_colour2: bool,
    // colour count
    pub colour_couint: B3,
    // rest
    #[skip]
    unused: B14,
}

impl Serialize for VertexType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(u32::from(self.clone()))
    }
}

impl<'de> Deserialize<'de> for VertexType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VertexTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for VertexTypeVisitor {
            type Value = VertexType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 2^32")
            }

            fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(VertexType::from(v))
            }
        }

        deserializer.deserialize_u32(VertexTypeVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldPrimGroup {
    pub shader_name: String,
    pub primitive_type: PrimitiveType,
    /// Bitfield of [`VertexType`]
    pub vertex_types: VertexType,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub num_matrices: u32,
}

impl Parse for OldPrimGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldPrimGroup {
            shader_name: pure3d_read_string(bytes)?,
            primitive_type: bytes.safe_get_u32_le()?.try_into()?,
            vertex_types: bytes.safe_get_u32_le()?.into(),
            num_vertices: bytes.safe_get_u32_le()?,
            num_indices: bytes.safe_get_u32_le()?,
            num_matrices: bytes.safe_get_u32_le()?,
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
