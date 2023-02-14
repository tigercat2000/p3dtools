use crate::{
    chunk::{
        data::{helpers::pure3d_read_string, parse_trait::Parse, types::shared::Matrix},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphBranch {
    pub num_children: u32,
}

impl Parse for ScenegraphBranch {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphBranch {
            num_children: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct ScenegraphTransform {
    pub num_children: u32,
    pub transform: Matrix,
}

impl Parse for ScenegraphTransform {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(ScenegraphTransform {
            num_children: bytes.get_u32_le(),
            transform: Matrix::parse(bytes, typ)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphVisibility {
    pub num_children: u32,
    pub is_visible: u32,
}

impl Parse for ScenegraphVisibility {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphVisibility {
            num_children: bytes.get_u32_le(),
            is_visible: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphAttachment {
    pub drawable_pose_name: String,
    pub num_points: u32,
}

impl Parse for ScenegraphAttachment {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphAttachment {
            drawable_pose_name: pure3d_read_string(bytes)?,
            num_points: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphAttachmentPoint {
    pub joint: u32,
}

impl Parse for ScenegraphAttachmentPoint {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphAttachmentPoint {
            joint: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphDrawable {
    pub drawable_name: String,
    pub is_translucent: u32,
}

impl Parse for ScenegraphDrawable {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphDrawable {
            drawable_name: pure3d_read_string(bytes)?,
            is_translucent: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphCamera {
    pub camera_name: String,
}

impl Parse for ScenegraphCamera {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphCamera {
            camera_name: pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_snake_case)]
pub struct ScenegraphLightGroup {
    pub light_group_name: String,
}

impl Parse for ScenegraphLightGroup {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphLightGroup {
            light_group_name: pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(non_snake_case)]
pub struct ScenegraphSortOrder {
    pub sort_order: f32,
}

impl Parse for ScenegraphSortOrder {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ScenegraphSortOrder {
            sort_order: bytes.get_f32_le(),
        })
    }
}
