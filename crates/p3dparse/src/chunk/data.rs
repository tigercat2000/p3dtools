use self::{
    chunks::{
        animation::{AnimationData, AnimationGroupData, AnimationGroupListData, AnimationSizeData},
        channel::{ChannelData, ChannelInterpolationData},
        explosion::ExplosionEffectData,
        image::{ImageData, ImageRawData},
        mesh::{MeshData, OldPrimGroupData, PositionListData},
        name::NameData,
        old_billboard::{
            OldBillboardDisplayInfoData, OldBillboardPerspectiveInfoData, OldBillboardQuadData,
            OldBillboardQuadGroupData,
        },
        old_particle_system::{
            OldBaseEmitterData, OldParticleSystemData, OldSpriteEmitterData, WorldEffectData,
        },
        shader::{ShaderData, VertexShaderData},
        shader_param::ShaderParamData,
        skeleton::{
            SkeletonData, SkeletonJointBonePreserveData, SkeletonJointData,
            SkeletonJointMirrorMapData,
        },
        texture::TextureData,
        version::VersionData,
    },
    parse_trait::Parse,
};

use super::types::ChunkType;
use crate::Result;
use bytes::Bytes;
use eyre::eyre;

mod chunks;
mod common_types;
mod helpers;
mod parse_trait;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ChunkData {
    None,
    // -- Rendering -- //
    Texture(NameData, VersionData, TextureData),
    Image(NameData, VersionData, ImageData),
    ImageRaw(ImageRawData),
    Shader(NameData, VersionData, ShaderData),
    ShaderParam(ShaderParamData),
    VertexShader(VertexShaderData),
    // -- Old Particle System -- //
    OldParticleSystem(VersionData, NameData, OldParticleSystemData),
    OldSpriteEmitter(VersionData, NameData, OldSpriteEmitterData),
    OldBaseEmitter(VersionData, NameData, OldBaseEmitterData),
    WorldEffect(VersionData, NameData, WorldEffectData),
    // -- Animations -- //
    Animation(VersionData, NameData, AnimationData),
    AnimationSize(VersionData, AnimationSizeData),
    AnimationGroup(VersionData, NameData, AnimationGroupData),
    AnimationGroupList(VersionData, AnimationGroupListData),
    Channel(VersionData, ChannelData),
    ChannelInterpolation(VersionData, ChannelInterpolationData),
    // -- Old Billboards -- //
    OldBillboardQuad(VersionData, NameData, OldBillboardQuadData),
    OldBillboardQuadGroup(VersionData, NameData, OldBillboardQuadGroupData),
    OldBillboardDisplayInfo(VersionData, OldBillboardDisplayInfoData),
    OldBillboardPerspectiveInfo(VersionData, OldBillboardPerspectiveInfoData),
    // -- Explosion FX -- //
    ExplosionEffectType(ExplosionEffectData),
    // -- Skeleton -- //
    Skeleton(NameData, VersionData, SkeletonData),
    SkeletonJoint(NameData, SkeletonJointData),
    SkeletonJointMirrorMap(SkeletonJointMirrorMapData),
    SkeletonJointBonePreserve(SkeletonJointBonePreserveData),
    // -- Mesh -- //
    Mesh(NameData, VersionData, MeshData),
    OldPrimGroup(VersionData, OldPrimGroupData),
    PositionList(PositionListData),
    Unknown,
}

impl ChunkData {
    pub fn from_chunk_type(typ: ChunkType, bytes: &mut Bytes) -> Result<ChunkData> {
        match typ {
            ChunkType::Root => Ok(ChunkData::None),
            // -- Rendering -- //
            ChunkType::Texture => Ok(ChunkData::Texture(
                NameData::parse(bytes, typ)?,
                VersionData::parse(bytes, typ)?,
                TextureData::parse(bytes, typ)?,
            )),
            ChunkType::Image => Ok(ChunkData::Image(
                NameData::parse(bytes, typ)?,
                VersionData::parse(bytes, typ)?,
                ImageData::parse(bytes, typ)?,
            )),
            ChunkType::ImageData => Ok(ChunkData::ImageRaw(ImageRawData::parse(bytes, typ)?)),
            ChunkType::Shader => Ok(ChunkData::Shader(
                NameData::parse(bytes, typ)?,
                VersionData::parse(bytes, typ)?,
                ShaderData::parse(bytes, typ)?,
            )),
            ChunkType::ShaderTextureParam
            | ChunkType::ShaderIntParam
            | ChunkType::ShaderFloatParam
            | ChunkType::ShaderColourParam => {
                Ok(ChunkData::ShaderParam(ShaderParamData::parse(bytes, typ)?))
            }
            ChunkType::VertexShader => Ok(ChunkData::VertexShader(VertexShaderData::parse(
                bytes, typ,
            )?)),
            // -- Old Particle System -- //
            ChunkType::OldParticleSystem => Ok(ChunkData::OldParticleSystem(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                OldParticleSystemData::parse(bytes, typ)?,
            )),
            // Ignore these chunks
            ChunkType::OldParticleInstancingInfo | ChunkType::OldParticleAnimation => {
                Ok(ChunkData::Unknown)
            }
            ChunkType::OldSpriteEmitter => Ok(ChunkData::OldSpriteEmitter(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                OldSpriteEmitterData::parse(bytes, typ)?,
            )),
            ChunkType::OldBaseEmitter => Ok(ChunkData::OldBaseEmitter(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                OldBaseEmitterData::parse(bytes, typ)?,
            )),
            ChunkType::WorldEffect => Ok(ChunkData::WorldEffect(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                WorldEffectData::parse(bytes, typ)?,
            )),
            // -- Animations -- //
            ChunkType::Animation => Ok(ChunkData::Animation(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                AnimationData::parse(bytes, typ)?,
            )),
            ChunkType::AnimationSize => Ok(ChunkData::AnimationSize(
                VersionData::parse(bytes, typ)?,
                AnimationSizeData::parse(bytes, typ)?,
            )),
            ChunkType::AnimationGroup => Ok(ChunkData::AnimationGroup(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                AnimationGroupData::parse(bytes, typ)?,
            )),
            ChunkType::AnimationGroupList => Ok(ChunkData::AnimationGroupList(
                VersionData::parse(bytes, typ)?,
                AnimationGroupListData::parse(bytes, typ)?,
            )),
            ChunkType::Float1Channel
            | ChunkType::Float2Channel
            | ChunkType::IntChannel
            | ChunkType::Vector1DOFChannel
            | ChunkType::Vector2DOFChannel
            | ChunkType::Vector3DOFChannel
            | ChunkType::QuaternionChannel
            | ChunkType::ColourChannel => Ok(ChunkData::Channel(
                VersionData::parse(bytes, typ)?,
                ChannelData::parse(bytes, typ)?,
            )),
            ChunkType::ChannelInterpolationMode => Ok(ChunkData::ChannelInterpolation(
                VersionData::parse(bytes, typ)?,
                ChannelInterpolationData::parse(bytes, typ)?,
            )),
            ChunkType::OldEmitterAnimation | ChunkType::OldGeneratorAnimation => {
                Ok(ChunkData::None)
            }
            // -- Old Billboards -- //
            ChunkType::OldBillboardQuad => Ok(ChunkData::OldBillboardQuad(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                OldBillboardQuadData::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardQuadGroup => Ok(ChunkData::OldBillboardQuadGroup(
                VersionData::parse(bytes, typ)?,
                NameData::parse(bytes, typ)?,
                OldBillboardQuadGroupData::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardDisplayInfo => Ok(ChunkData::OldBillboardDisplayInfo(
                VersionData::parse(bytes, typ)?,
                OldBillboardDisplayInfoData::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardPerspectiveInfo => Ok(ChunkData::OldBillboardPerspectiveInfo(
                VersionData::parse(bytes, typ)?,
                OldBillboardPerspectiveInfoData::parse(bytes, typ)?,
            )),
            // -- Explosion FX -- //
            ChunkType::ExplosionEffectType => Ok(ChunkData::ExplosionEffectType(
                ExplosionEffectData::parse(bytes, typ)?,
            )),
            // -- Skeleton -- //
            ChunkType::Skeleton => Ok(ChunkData::Skeleton(
                NameData::parse(bytes, typ)?,
                VersionData::parse(bytes, typ)?,
                SkeletonData::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJoint => Ok(ChunkData::SkeletonJoint(
                NameData::parse(bytes, typ)?,
                SkeletonJointData::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJointMirrorMap => Ok(ChunkData::SkeletonJointMirrorMap(
                SkeletonJointMirrorMapData::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJointBonePreserve => Ok(ChunkData::SkeletonJointBonePreserve(
                SkeletonJointBonePreserveData::parse(bytes, typ)?,
            )),
            // -- Mesh -- //
            ChunkType::Mesh => Ok(ChunkData::Mesh(
                NameData::parse(bytes, typ)?,
                VersionData::parse(bytes, typ)?,
                MeshData::parse(bytes, typ)?,
            )),
            ChunkType::OldPrimGroup => Ok(ChunkData::OldPrimGroup(
                VersionData::parse(bytes, typ)?,
                OldPrimGroupData::parse(bytes, typ)?,
            )),
            ChunkType::PositionList => Ok(ChunkData::PositionList(PositionListData::parse(
                bytes, typ,
            )?)),
            // -- Other produces error -- //
            typ => Err(eyre!("ChunkData parsing is not implemented for {:?}", typ)),
        }
    }
}
