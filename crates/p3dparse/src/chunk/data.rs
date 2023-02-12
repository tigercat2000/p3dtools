use self::{
    parse_trait::Parse,
    types::{
        animation::{Animation, AnimationGroup, AnimationGroupList, AnimationSize},
        channel::{Channel, ChannelInterpolation},
        explosion::ExplosionEffect,
        image::{Image, ImageRaw},
        mesh::{Mesh, OldPrimGroup, PositionList},
        name::Name,
        old_billboard::{
            OldBillboardDisplayInfo, OldBillboardPerspectiveInfo, OldBillboardQuad,
            OldBillboardQuadGroup,
        },
        old_particle_system::{OldBaseEmitter, OldParticleSystem, OldSpriteEmitter, WorldEffect},
        shader::{Shader, VertexShader},
        shader_param::ShaderParam,
        skeleton::{Skeleton, SkeletonJoint, SkeletonJointBonePreserve, SkeletonJointMirrorMap},
        texture::Texture,
        version::Version,
    },
};

use crate::{
    chunk::{data::types::mesh::UVList, types::ChunkType},
    Result,
};
use bytes::Bytes;
use eyre::eyre;

mod helpers;
mod parse_trait;
mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ChunkData {
    None,
    // -- Rendering -- //
    Texture(Name, Version, Texture),
    Image(Name, Version, Image),
    ImageRaw(ImageRaw),
    Shader(Name, Version, Shader),
    ShaderParam(ShaderParam),
    VertexShader(VertexShader),
    // -- Old Particle System -- //
    OldParticleSystem(Version, Name, OldParticleSystem),
    OldSpriteEmitter(Version, Name, OldSpriteEmitter),
    OldBaseEmitter(Version, Name, OldBaseEmitter),
    WorldEffect(Version, Name, WorldEffect),
    // -- Animations -- //
    Animation(Version, Name, Animation),
    AnimationSize(Version, AnimationSize),
    AnimationGroup(Version, Name, AnimationGroup),
    AnimationGroupList(Version, AnimationGroupList),
    Channel(Version, Channel),
    ChannelInterpolation(Version, ChannelInterpolation),
    // -- Old Billboards -- //
    OldBillboardQuad(Version, Name, OldBillboardQuad),
    OldBillboardQuadGroup(Version, Name, OldBillboardQuadGroup),
    OldBillboardDisplayInfo(Version, OldBillboardDisplayInfo),
    OldBillboardPerspectiveInfo(Version, OldBillboardPerspectiveInfo),
    // -- Explosion FX -- //
    ExplosionEffectType(ExplosionEffect),
    // -- Skeleton -- //
    Skeleton(Name, Version, Skeleton),
    SkeletonJoint(Name, SkeletonJoint),
    SkeletonJointMirrorMap(SkeletonJointMirrorMap),
    SkeletonJointBonePreserve(SkeletonJointBonePreserve),
    // -- Mesh -- //
    Mesh(Name, Version, Mesh),
    OldPrimGroup(Version, OldPrimGroup),
    PositionList(PositionList),
    UVList(UVList),
    Unknown,
}

impl ChunkData {
    pub fn from_chunk_type(typ: ChunkType, bytes: &mut Bytes) -> Result<ChunkData> {
        match typ {
            ChunkType::Root => Ok(ChunkData::None),
            // -- Rendering -- //
            ChunkType::Texture => Ok(ChunkData::Texture(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Texture::parse(bytes, typ)?,
            )),
            ChunkType::Image => Ok(ChunkData::Image(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Image::parse(bytes, typ)?,
            )),
            ChunkType::ImageData => Ok(ChunkData::ImageRaw(ImageRaw::parse(bytes, typ)?)),
            ChunkType::Shader => Ok(ChunkData::Shader(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Shader::parse(bytes, typ)?,
            )),
            ChunkType::ShaderTextureParam
            | ChunkType::ShaderIntParam
            | ChunkType::ShaderFloatParam
            | ChunkType::ShaderColourParam => {
                Ok(ChunkData::ShaderParam(ShaderParam::parse(bytes, typ)?))
            }
            ChunkType::VertexShader => {
                Ok(ChunkData::VertexShader(VertexShader::parse(bytes, typ)?))
            }
            // -- Old Particle System -- //
            ChunkType::OldParticleSystem => Ok(ChunkData::OldParticleSystem(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldParticleSystem::parse(bytes, typ)?,
            )),
            // Ignore these chunks
            ChunkType::OldParticleInstancingInfo | ChunkType::OldParticleAnimation => {
                Ok(ChunkData::Unknown)
            }
            ChunkType::OldSpriteEmitter => Ok(ChunkData::OldSpriteEmitter(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldSpriteEmitter::parse(bytes, typ)?,
            )),
            ChunkType::OldBaseEmitter => Ok(ChunkData::OldBaseEmitter(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldBaseEmitter::parse(bytes, typ)?,
            )),
            ChunkType::WorldEffect => Ok(ChunkData::WorldEffect(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                WorldEffect::parse(bytes, typ)?,
            )),
            // -- Animations -- //
            ChunkType::Animation => Ok(ChunkData::Animation(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                Animation::parse(bytes, typ)?,
            )),
            ChunkType::AnimationSize => Ok(ChunkData::AnimationSize(
                Version::parse(bytes, typ)?,
                AnimationSize::parse(bytes, typ)?,
            )),
            ChunkType::AnimationGroup => Ok(ChunkData::AnimationGroup(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                AnimationGroup::parse(bytes, typ)?,
            )),
            ChunkType::AnimationGroupList => Ok(ChunkData::AnimationGroupList(
                Version::parse(bytes, typ)?,
                AnimationGroupList::parse(bytes, typ)?,
            )),
            ChunkType::Float1Channel
            | ChunkType::Float2Channel
            | ChunkType::IntChannel
            | ChunkType::Vector1DOFChannel
            | ChunkType::Vector2DOFChannel
            | ChunkType::Vector3DOFChannel
            | ChunkType::QuaternionChannel
            | ChunkType::ColourChannel => Ok(ChunkData::Channel(
                Version::parse(bytes, typ)?,
                Channel::parse(bytes, typ)?,
            )),
            ChunkType::ChannelInterpolationMode => Ok(ChunkData::ChannelInterpolation(
                Version::parse(bytes, typ)?,
                ChannelInterpolation::parse(bytes, typ)?,
            )),
            ChunkType::OldEmitterAnimation | ChunkType::OldGeneratorAnimation => {
                Ok(ChunkData::None)
            }
            // -- Old Billboards -- //
            ChunkType::OldBillboardQuad => Ok(ChunkData::OldBillboardQuad(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldBillboardQuad::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardQuadGroup => Ok(ChunkData::OldBillboardQuadGroup(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldBillboardQuadGroup::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardDisplayInfo => Ok(ChunkData::OldBillboardDisplayInfo(
                Version::parse(bytes, typ)?,
                OldBillboardDisplayInfo::parse(bytes, typ)?,
            )),
            ChunkType::OldBillboardPerspectiveInfo => Ok(ChunkData::OldBillboardPerspectiveInfo(
                Version::parse(bytes, typ)?,
                OldBillboardPerspectiveInfo::parse(bytes, typ)?,
            )),
            // -- Explosion FX -- //
            ChunkType::ExplosionEffectType => Ok(ChunkData::ExplosionEffectType(
                ExplosionEffect::parse(bytes, typ)?,
            )),
            // -- Skeleton -- //
            ChunkType::Skeleton => Ok(ChunkData::Skeleton(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Skeleton::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJoint => Ok(ChunkData::SkeletonJoint(
                Name::parse(bytes, typ)?,
                SkeletonJoint::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJointMirrorMap => Ok(ChunkData::SkeletonJointMirrorMap(
                SkeletonJointMirrorMap::parse(bytes, typ)?,
            )),
            ChunkType::SkeletonJointBonePreserve => Ok(ChunkData::SkeletonJointBonePreserve(
                SkeletonJointBonePreserve::parse(bytes, typ)?,
            )),
            // -- Mesh -- //
            ChunkType::Mesh => Ok(ChunkData::Mesh(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Mesh::parse(bytes, typ)?,
            )),
            ChunkType::OldPrimGroup => Ok(ChunkData::OldPrimGroup(
                Version::parse(bytes, typ)?,
                OldPrimGroup::parse(bytes, typ)?,
            )),
            ChunkType::PositionList => {
                Ok(ChunkData::PositionList(PositionList::parse(bytes, typ)?))
            }
            ChunkType::UVList => Ok(ChunkData::UVList(UVList::parse(bytes, typ)?)),
            // -- Other produces error -- //
            typ => Err(eyre!("ChunkData parsing is not implemented for {:?}", typ)),
        }
    }
}
