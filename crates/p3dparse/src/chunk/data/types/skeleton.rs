use crate::{
    chunk::{
        data::{parse_trait::Parse, types::shared::Matrix},
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Skeleton {
    pub num_joints: u32,
}

impl Parse for Skeleton {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Skeleton {
            num_joints: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SkeletonJoint {
    pub parent: u32,
    pub dof: i32,
    pub free_axis: i32,
    pub primary_axis: i32,
    pub secondary_axis: i32,
    pub twist_axis: i32,
    pub rest_pose: Matrix,
}

impl Parse for SkeletonJoint {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        Ok(SkeletonJoint {
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
pub struct SkeletonJointMirrorMap {
    pub mapped_joint_index: u32,
    pub x_axis_map: f32,
    pub y_axis_map: f32,
    pub z_axis_map: f32,
}

impl Parse for SkeletonJointMirrorMap {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(SkeletonJointMirrorMap {
            mapped_joint_index: bytes.get_u32_le(),
            x_axis_map: bytes.get_f32_le(),
            y_axis_map: bytes.get_f32_le(),
            z_axis_map: bytes.get_f32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkeletonJointBonePreserve {
    pub preserve_bone_lengths: u32,
}

impl Parse for SkeletonJointBonePreserve {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(SkeletonJointBonePreserve {
            preserve_bone_lengths: bytes.get_u32_le(),
        })
    }
}
