use crate::{
    chunk::{
        data::{common_types::Matrix, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkeletonData {
    pub num_joints: u32,
}

impl Parse for SkeletonData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(SkeletonData {
            num_joints: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SkeletonJointData {
    pub parent: u32,
    pub dof: i32,
    pub free_axis: i32,
    pub primary_axis: i32,
    pub secondary_axis: i32,
    pub twist_axis: i32,
    pub rest_pose: Matrix,
}

impl Parse for SkeletonJointData {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(SkeletonJointData {
            parent: bytes.get_u32_le(),
            dof: bytes.get_i32_le(),
            free_axis: bytes.get_i32_le(),
            primary_axis: bytes.get_i32_le(),
            secondary_axis: bytes.get_i32_le(),
            twist_axis: bytes.get_i32_le(),
            rest_pose: Matrix::parse(bytes, typ)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SkeletonJointMirrorMapData {
    pub mapped_joint_index: u32,
    pub x_axis_map: f32,
    pub y_axis_map: f32,
    pub z_axis_map: f32,
}

impl Parse for SkeletonJointMirrorMapData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(SkeletonJointMirrorMapData {
            mapped_joint_index: bytes.get_u32_le(),
            x_axis_map: bytes.get_f32_le(),
            y_axis_map: bytes.get_f32_le(),
            z_axis_map: bytes.get_f32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkeletonJointBonePreserveData {
    pub preserve_bone_lengths: u32,
}

impl Parse for SkeletonJointBonePreserveData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(SkeletonJointBonePreserveData {
            preserve_bone_lengths: bytes.get_u32_le(),
        })
    }
}
