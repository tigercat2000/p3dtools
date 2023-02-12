use crate::{
    chunk::{
        data::{
            helpers,
            parse_trait::Parse,
            types::shared::{Colour, Vector2, Vector3},
        },
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mesh {
    pub num_prim_groups: u32,
}

impl Parse for Mesh {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Mesh {
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
pub struct PositionList {
    pub positions: Vec<Vector3>,
}

impl Parse for PositionList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.get_u32_le() as usize;

        let mut positions = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            positions.push(helpers::read_vec3(bytes)?);
        }

        Ok(PositionList { positions })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct UVList {
    pub channel: u32,
    pub UVs: Vec<Vector2>,
}

impl Parse for UVList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.get_u32_le() as usize;
        let channel = bytes.get_u32_le();

        #[allow(non_snake_case)]
        let mut UVs = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            UVs.push(helpers::read_vec2(bytes)?);
        }

        Ok(UVList { channel, UVs })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColourList {
    pub colours: Vec<Colour>,
}

impl Parse for ColourList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.get_u32_le() as usize;

        let mut colours = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            colours.push(helpers::read_colour(bytes)?);
        }

        Ok(ColourList { colours })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexList {
    pub indices: Vec<u32>,
}

impl Parse for IndexList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let capacity = bytes.get_u32_le() as usize;

        let mut indices = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            indices.push(bytes.get_u32_le());
        }

        Ok(IndexList { indices })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingBox {
    pub low: Vector3,
    pub high: Vector3,
}

impl Parse for BoundingBox {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(BoundingBox {
            low: helpers::read_vec3(bytes)?,
            high: helpers::read_vec3(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BoundingSphere {
    pub centre: Vector3,
    pub radius: f32,
}

impl Parse for BoundingSphere {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(BoundingSphere {
            centre: helpers::read_vec3(bytes)?,
            radius: bytes.get_f32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderStatus {
    pub cast_shadow: u32,
}

impl Parse for RenderStatus {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(RenderStatus {
            cast_shadow: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawable {
    pub skeleton_name: String,
}

impl Parse for CompositeDrawable {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawable {
            skeleton_name: helpers::pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawableSkinList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawableSkinList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSkinList {
            num_elements: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawableSkin {
    pub is_translucent: u32,
}

impl Parse for CompositeDrawableSkin {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSkin {
            is_translucent: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawablePropList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawablePropList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawablePropList {
            num_elements: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawableProp {
    pub is_translucent: u32,
    pub skeleton_joint_id: u32,
}

impl Parse for CompositeDrawableProp {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableProp {
            is_translucent: bytes.get_u32_le(),
            skeleton_joint_id: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawableEffectList {
    pub num_elements: u32,
}

impl Parse for CompositeDrawableEffectList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableEffectList {
            num_elements: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositeDrawableEffect {
    pub is_translucent: u32,
    pub skeleton_joint_id: u32,
}

impl Parse for CompositeDrawableEffect {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableEffect {
            is_translucent: bytes.get_u32_le(),
            skeleton_joint_id: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompositeDrawableSortOrder {
    pub sort_order: f32,
}

impl Parse for CompositeDrawableSortOrder {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CompositeDrawableSortOrder {
            sort_order: bytes.get_f32_le(),
        })
    }
}
