use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{
            helpers::{pure3d_read_string, read_vec3},
            parse_trait::Parse,
            types::shared::Vector3,
        },
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct CollisionObject {
    pub material_name: String,
    pub num_sub_object: u32,
    pub num_owner: u32,
}

impl Parse for CollisionObject {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionObject {
            material_name: pure3d_read_string(bytes)?,
            num_sub_object: bytes.safe_get_u32_le()?,
            num_owner: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct CollisionVolume {
    pub object_reference_index: u32,
    pub owner_index: i32,
    pub num_volume: u32,
}

impl Parse for CollisionVolume {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionVolume {
            object_reference_index: bytes.safe_get_u32_le()?,
            owner_index: bytes.safe_get_i32_le()?,
            num_volume: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct CollisionVolumeOwner {
    pub num_names: u32,
}

impl Parse for CollisionVolumeOwner {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionVolumeOwner {
            num_names: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct CollisionBoundingBox {
    /// Literally the same in SRR2 source code, "dummy" or "Nothing"
    pub nothing: u32,
}

impl Parse for CollisionBoundingBox {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionBoundingBox {
            nothing: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct CollisionOblongBox {
    pub half_extent_x: f32,
    pub half_extent_y: f32,
    pub half_extent_z: f32,
}

impl Parse for CollisionOblongBox {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionOblongBox {
            half_extent_x: bytes.safe_get_f32_le()?,
            half_extent_y: bytes.safe_get_f32_le()?,
            half_extent_z: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct CollisionCylinder {
    pub cylinder_radius: f32,
    pub length: f32,
    pub flat_end: u16,
}

impl Parse for CollisionCylinder {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionCylinder {
            cylinder_radius: bytes.safe_get_f32_le()?,
            length: bytes.safe_get_f32_le()?,
            flat_end: bytes.safe_get_u16_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct CollisionSphere {
    pub radius: f32,
}

impl Parse for CollisionSphere {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionSphere {
            radius: bytes.safe_get_f32_le()?,
        })
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct CollisionVector {
    pub vector: Vector3,
}

impl Parse for CollisionVector {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionVector {
            vector: read_vec3(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct CollisionObjectAttribute {
    pub static_attribute: u16,
    pub default_area: u32,
    pub can_roll: u16,
    pub can_slide: u16,
    pub can_spin: u16,
    pub can_bounce: u16,
    pub extra_attribute_1: u32,
    pub extra_attribute_2: u32,
    pub extra_attribute_3: u32,
}

impl Parse for CollisionObjectAttribute {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(CollisionObjectAttribute {
            static_attribute: bytes.safe_get_u16_le()?,
            default_area: bytes.safe_get_u32_le()?,
            can_roll: bytes.safe_get_u16_le()?,
            can_slide: bytes.safe_get_u16_le()?,
            can_spin: bytes.safe_get_u16_le()?,
            can_bounce: bytes.safe_get_u16_le()?,
            extra_attribute_1: bytes.safe_get_u32_le()?,
            extra_attribute_2: bytes.safe_get_u32_le()?,
            extra_attribute_3: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct IntersectDSG {
    pub indices: Vec<u32>,
    pub positions: Vec<Vector3>,
    pub normals: Vec<Vector3>,
}

impl Parse for IntersectDSG {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let indices_len = bytes.safe_get_u32_le()?;
        let mut indices = Vec::with_capacity(indices_len as usize);
        for _ in 0..indices_len {
            indices.push(bytes.safe_get_u32_le()?)
        }
        let positions_len = bytes.safe_get_u32_le()?;
        let mut positions = Vec::with_capacity(positions_len as usize);
        for _ in 0..positions_len {
            positions.push(read_vec3(bytes)?)
        }
        let normals_len = bytes.safe_get_u32_le()?;
        let mut normals = Vec::with_capacity(normals_len as usize);
        for _ in 0..normals_len {
            normals.push(read_vec3(bytes)?)
        }
        Ok(IntersectDSG {
            indices,
            positions,
            normals,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct TerrainTypeList {
    pub types: Vec<u8>,
}

impl Parse for TerrainTypeList {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let types_len = bytes.safe_get_u32_le()?;
        let mut types = Vec::with_capacity(types_len as usize);
        for _ in 0..types_len {
            types.push(bytes.safe_get_u8()?)
        }
        Ok(TerrainTypeList { types })
    }
}
