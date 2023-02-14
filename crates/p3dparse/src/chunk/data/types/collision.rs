use crate::{
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
use bytes::{Buf, Bytes};

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
            num_sub_object: bytes.get_u32_le(),
            num_owner: bytes.get_u32_le(),
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
            object_reference_index: bytes.get_u32_le(),
            owner_index: bytes.get_i32_le(),
            num_volume: bytes.get_u32_le(),
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
            num_names: bytes.get_u32_le(),
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
            nothing: bytes.get_u32_le(),
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
            half_extent_x: bytes.get_f32_le(),
            half_extent_y: bytes.get_f32_le(),
            half_extent_z: bytes.get_f32_le(),
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
            static_attribute: bytes.get_u16_le(),
            default_area: bytes.get_u32_le(),
            can_roll: bytes.get_u16_le(),
            can_slide: bytes.get_u16_le(),
            can_spin: bytes.get_u16_le(),
            can_bounce: bytes.get_u16_le(),
            extra_attribute_1: bytes.get_u32_le(),
            extra_attribute_2: bytes.get_u32_le(),
            extra_attribute_3: bytes.get_u32_le(),
        })
    }
}
