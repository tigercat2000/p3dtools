use crate::{
    chunk::{
        data::{
            parse_trait::Parse,
            types::{
                animation::{Animation, AnimationGroup, AnimationGroupList, AnimationSize},
                channel::{Channel, ChannelInterpolation},
                collision::{
                    CollisionBoundingBox, CollisionObject, CollisionObjectAttribute,
                    CollisionOblongBox, CollisionVector, CollisionVolume, CollisionVolumeOwner,
                },
                explosion::BreakableObject,
                gameattr::{GameAttr, GameAttrParam},
                image::{Image, ImageRaw},
                locator::{WBLocator, WBMatrix, WBRail, WBSpline, WBTriggerVolume},
                mesh::{
                    ColourList, CompositeDrawable, CompositeDrawableEffect,
                    CompositeDrawableEffectList, CompositeDrawableProp, CompositeDrawablePropList,
                    CompositeDrawableSkin, CompositeDrawableSkinList, CompositeDrawableSortOrder,
                    IndexList, Mesh, OldPrimGroup, PositionList, RenderStatus, UVList,
                },
                name::Name,
                object::{
                    AnimatedObject, AnimatedObjectAnimation, AnimatedObjectDSGWrapper,
                    AnimatedObjectFactory, MultiController, MultiControllerTracks, ObjectDSG,
                    OldFrameController,
                },
                old_billboard::{
                    OldBillboardDisplayInfo, OldBillboardPerspectiveInfo, OldBillboardQuad,
                    OldBillboardQuadGroup,
                },
                old_particle_system::{
                    OldBaseEmitter, OldParticleSystem, OldParticleSystemFactory,
                    OldParticleSystemInstancingInfo, OldSpriteEmitter,
                },
                physics::{
                    BoundingBox, BoundingSphere, PhysicsInertiaMatrix, PhysicsJoint, PhysicsObject,
                    PhysicsVector,
                },
                prop_state::{
                    ObjectAttributes, StatePropCallbackData, StatePropDataV1, StatePropEventData,
                    StatePropFrameControllerData, StatePropStateDataV1, StatePropVisibilitiesData,
                },
                scenegraph::{
                    ScenegraphAttachment, ScenegraphAttachmentPoint, ScenegraphBranch,
                    ScenegraphCamera, ScenegraphDrawable, ScenegraphLightGroup,
                    ScenegraphSortOrder, ScenegraphTransform, ScenegraphVisibility,
                },
                shader::{Shader, VertexShader},
                shader_param::ShaderParam,
                skeleton::{
                    Skeleton, SkeletonJoint, SkeletonJointBonePreserve, SkeletonJointMirrorMap,
                },
                texture::Texture,
                version::Version,
            },
        },
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use eyre::eyre;

mod helpers;
mod parse_trait;
mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
#[allow(dead_code)]
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
    OldParticleSystemFactory(Version, Name, OldParticleSystemFactory),
    OldParticleInstancingInfo(Version, OldParticleSystemInstancingInfo),
    OldParticleAnimation(Version),
    OldEmitterAnimation(Version),
    OldGeneratorAnimation(Version),
    OldSpriteEmitter(Version, Name, OldSpriteEmitter),
    OldBaseEmitter(Version, Name, OldBaseEmitter),
    OldParticleSystem(Version, Name, OldParticleSystem),
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
    // -- Breakable Objects -- //
    BreakableObject(BreakableObject),
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
    ColourList(ColourList),
    IndexList(IndexList),
    RenderStatus(RenderStatus),
    CompositeDrawable(Name, CompositeDrawable),
    CompositeDrawableEffect(Name, CompositeDrawableEffect),
    CompositeDrawableEffectList(CompositeDrawableEffectList),
    CompositeDrawableProp(Name, CompositeDrawableProp),
    CompositeDrawablePropList(CompositeDrawablePropList),
    CompositeDrawableSkin(Name, CompositeDrawableSkin),
    CompositeDrawableSkinList(CompositeDrawableSkinList),
    CompositeDrawableSortOrder(CompositeDrawableSortOrder),
    AnimatedObjectFactory(Version, Name, AnimatedObjectFactory),
    AnimatedObject(Version, Name, AnimatedObject),
    AnimatedObjectAnimation(Version, Name, AnimatedObjectAnimation),
    OldFrameController(Version, Name, OldFrameController),
    MultiController(Name, Version, MultiController),
    MultiControllerTracks(MultiControllerTracks),
    ObjectDSG(Name, Version, ObjectDSG),
    AnimatedObjectDSGWrapper(Name, AnimatedObjectDSGWrapper),
    // -- Physics -- //
    BoundingBox(BoundingBox),
    BoundingSphere(BoundingSphere),
    PhysicsObject(Name, Version, PhysicsObject),
    PhysicsJoint(PhysicsJoint),
    PhysicsVector(PhysicsVector),
    PhysicsInertiaMatrix(PhysicsInertiaMatrix),
    // -- Collision -- //
    CollisionObject(Name, Version, CollisionObject),
    CollisionVolume(CollisionVolume),
    CollisionVolumeOwner(CollisionVolumeOwner),
    CollisionVolumeOwnerName(Name),
    CollisionBoundingBox(CollisionBoundingBox),
    CollisionOblongBox(CollisionOblongBox),
    CollisionVector(CollisionVector),
    CollisionObjectAttribute(CollisionObjectAttribute),
    // -- Prop Data -- //
    StatePropDataV1(Version, Name, StatePropDataV1),
    StatePropStateDataV1(Name, StatePropStateDataV1),
    StatePropVisibilitiesData(Name, StatePropVisibilitiesData),
    StatePropFrameControllerData(Name, StatePropFrameControllerData),
    StatePropEventData(Name, StatePropEventData),
    StatePropCallbackData(Name, StatePropCallbackData),
    PropInstanceList(Name),
    ObjectAttributes(ObjectAttributes),
    // -- Scenegraph -- //
    Scenegraph(Name, Version),
    ScenegraphBranch(Name, ScenegraphBranch),
    ScenegraphTransform(Name, ScenegraphTransform),
    ScenegraphVisibility(Name, ScenegraphVisibility),
    ScenegraphAttachment(Name, ScenegraphAttachment),
    ScenegraphAttachmentPoint(ScenegraphAttachmentPoint),
    ScenegraphDrawable(Name, ScenegraphDrawable),
    ScenegraphCamera(Name, ScenegraphCamera),
    ScenegraphLightGroup(Name, ScenegraphLightGroup),
    ScenegraphSortOrder(ScenegraphSortOrder),
    // -- GameAttr -- //
    GameAttr(Name, Version, GameAttr),
    GameAttrParam(GameAttrParam),
    // -- Locator -- //
    WBLocator(Name, WBLocator),
    WBTriggerVolume(Name, WBTriggerVolume),
    WBMatrix(WBMatrix),
    WBSpline(Name, WBSpline),
    WBRail(Name, WBRail),
    Unknown,
}

impl ChunkData {
    pub fn from_chunk_type(typ: ChunkType, bytes: &mut Bytes) -> Result<ChunkData> {
        match typ {
            ChunkType::DataFile => Ok(ChunkData::None),
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
            ChunkType::OldParticleSystemFactory => Ok(ChunkData::OldParticleSystemFactory(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldParticleSystemFactory::parse(bytes, typ)?,
            )),
            // Ignore these chunks
            ChunkType::OldParticleInstancingInfo => Ok(ChunkData::OldParticleInstancingInfo(
                Version::parse(bytes, typ)?,
                OldParticleSystemInstancingInfo::parse(bytes, typ)?,
            )),
            ChunkType::OldParticleAnimation => {
                Ok(ChunkData::OldParticleAnimation(Version::parse(bytes, typ)?))
            }
            ChunkType::OldEmitterAnimation => {
                Ok(ChunkData::OldEmitterAnimation(Version::parse(bytes, typ)?))
            }
            ChunkType::OldGeneratorAnimation => Ok(ChunkData::OldGeneratorAnimation(
                Version::parse(bytes, typ)?,
            )),
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
            ChunkType::OldParticleSystem => Ok(ChunkData::OldParticleSystem(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldParticleSystem::parse(bytes, typ)?,
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
            | ChunkType::ColourChannel
            | ChunkType::BoolChannel => Ok(ChunkData::Channel(
                Version::parse(bytes, typ)?,
                Channel::parse(bytes, typ)?,
            )),
            ChunkType::ChannelInterpolationMode => Ok(ChunkData::ChannelInterpolation(
                Version::parse(bytes, typ)?,
                ChannelInterpolation::parse(bytes, typ)?,
            )),
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
            ChunkType::BreakableObject => Ok(ChunkData::BreakableObject(BreakableObject::parse(
                bytes, typ,
            )?)),
            // -- Skeleton -- //
            ChunkType::P3DSkeleton => Ok(ChunkData::Skeleton(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Skeleton::parse(bytes, typ)?,
            )),
            ChunkType::P3DSkeletonJoint => Ok(ChunkData::SkeletonJoint(
                Name::parse(bytes, typ)?,
                SkeletonJoint::parse(bytes, typ)?,
            )),
            ChunkType::P3DSkeletonJointMirrorMap => Ok(ChunkData::SkeletonJointMirrorMap(
                SkeletonJointMirrorMap::parse(bytes, typ)?,
            )),
            ChunkType::P3DSkeletonJointBonePreserve => Ok(ChunkData::SkeletonJointBonePreserve(
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
            ChunkType::ColourList => Ok(ChunkData::ColourList(ColourList::parse(bytes, typ)?)),
            ChunkType::IndexList => Ok(ChunkData::IndexList(IndexList::parse(bytes, typ)?)),
            ChunkType::BBox => Ok(ChunkData::BoundingBox(BoundingBox::parse(bytes, typ)?)),
            ChunkType::BSphere => Ok(ChunkData::BoundingSphere(BoundingSphere::parse(
                bytes, typ,
            )?)),
            ChunkType::RenderStatus => {
                Ok(ChunkData::RenderStatus(RenderStatus::parse(bytes, typ)?))
            }
            ChunkType::P3DCompositeDrawable => Ok(ChunkData::CompositeDrawable(
                Name::parse(bytes, typ)?,
                CompositeDrawable::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawableEffect => Ok(ChunkData::CompositeDrawableEffect(
                Name::parse(bytes, typ)?,
                CompositeDrawableEffect::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawableEffectList => {
                Ok(ChunkData::CompositeDrawableEffectList(
                    CompositeDrawableEffectList::parse(bytes, typ)?,
                ))
            }
            ChunkType::P3DCompositeDrawableProp => Ok(ChunkData::CompositeDrawableProp(
                Name::parse(bytes, typ)?,
                CompositeDrawableProp::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawablePropList => Ok(ChunkData::CompositeDrawablePropList(
                CompositeDrawablePropList::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawableSkin => Ok(ChunkData::CompositeDrawableSkin(
                Name::parse(bytes, typ)?,
                CompositeDrawableSkin::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawableSkinList => Ok(ChunkData::CompositeDrawableSkinList(
                CompositeDrawableSkinList::parse(bytes, typ)?,
            )),
            ChunkType::P3DCompositeDrawableSortOrder => Ok(ChunkData::CompositeDrawableSortOrder(
                CompositeDrawableSortOrder::parse(bytes, typ)?,
            )),
            ChunkType::AnimatedObjectFactory => Ok(ChunkData::AnimatedObjectFactory(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                AnimatedObjectFactory::parse(bytes, typ)?,
            )),
            ChunkType::AnimatedObject => Ok(ChunkData::AnimatedObject(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                AnimatedObject::parse(bytes, typ)?,
            )),
            ChunkType::AnimatedObjectAnimation => Ok(ChunkData::AnimatedObjectAnimation(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                AnimatedObjectAnimation::parse(bytes, typ)?,
            )),
            ChunkType::OldFrameController => Ok(ChunkData::OldFrameController(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                OldFrameController::parse(bytes, typ)?,
            )),
            ChunkType::P3DMultiController => Ok(ChunkData::MultiController(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                MultiController::parse(bytes, typ)?,
            )),
            ChunkType::P3DMultiControllerTracks => Ok(ChunkData::MultiControllerTracks(
                MultiControllerTracks::parse(bytes, typ)?,
            )),
            ChunkType::EntityDSG | ChunkType::InstanceableAnimatedDynamicPhysicsDSG => {
                Ok(ChunkData::ObjectDSG(
                    Name::parse(bytes, typ)?,
                    Version::parse(bytes, typ)?,
                    ObjectDSG::parse(bytes, typ)?,
                ))
            }
            ChunkType::AnimatedObjectDSGWrapper => Ok(ChunkData::AnimatedObjectDSGWrapper(
                Name::parse(bytes, typ)?,
                AnimatedObjectDSGWrapper::parse(bytes, typ)?,
            )),
            ChunkType::PhysicsObject => Ok(ChunkData::PhysicsObject(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                PhysicsObject::parse(bytes, typ)?,
            )),
            ChunkType::PhysicsJoint => {
                Ok(ChunkData::PhysicsJoint(PhysicsJoint::parse(bytes, typ)?))
            }
            ChunkType::PhysicsVector => {
                Ok(ChunkData::PhysicsVector(PhysicsVector::parse(bytes, typ)?))
            }
            ChunkType::PhysicsInertiaMatrix => Ok(ChunkData::PhysicsInertiaMatrix(
                PhysicsInertiaMatrix::parse(bytes, typ)?,
            )),
            ChunkType::CollisionObject => Ok(ChunkData::CollisionObject(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                CollisionObject::parse(bytes, typ)?,
            )),
            ChunkType::CollisionVolume => Ok(ChunkData::CollisionVolume(CollisionVolume::parse(
                bytes, typ,
            )?)),
            ChunkType::CollisionVolumeOwner => Ok(ChunkData::CollisionVolumeOwner(
                CollisionVolumeOwner::parse(bytes, typ)?,
            )),
            ChunkType::CollisionVolumeOwnerName => Ok(ChunkData::CollisionVolumeOwnerName(
                Name::parse(bytes, typ)?,
            )),
            ChunkType::CollisionBoundingBox => Ok(ChunkData::CollisionBoundingBox(
                CollisionBoundingBox::parse(bytes, typ)?,
            )),
            ChunkType::CollisionOblongBox => Ok(ChunkData::CollisionOblongBox(
                CollisionOblongBox::parse(bytes, typ)?,
            )),
            ChunkType::CollisionVector => Ok(ChunkData::CollisionVector(CollisionVector::parse(
                bytes, typ,
            )?)),
            ChunkType::CollisionObjectAttribute => Ok(ChunkData::CollisionObjectAttribute(
                CollisionObjectAttribute::parse(bytes, typ)?,
            )),
            ChunkType::StatePropDataV1 => Ok(ChunkData::StatePropDataV1(
                Version::parse(bytes, typ)?,
                Name::parse(bytes, typ)?,
                StatePropDataV1::parse(bytes, typ)?,
            )),
            ChunkType::StatePropStateDataV1 => Ok(ChunkData::StatePropStateDataV1(
                Name::parse(bytes, typ)?,
                StatePropStateDataV1::parse(bytes, typ)?,
            )),
            ChunkType::StatePropVisibilitiesData => Ok(ChunkData::StatePropVisibilitiesData(
                Name::parse(bytes, typ)?,
                StatePropVisibilitiesData::parse(bytes, typ)?,
            )),
            ChunkType::StatePropFrameControllerData => Ok(ChunkData::StatePropFrameControllerData(
                Name::parse(bytes, typ)?,
                StatePropFrameControllerData::parse(bytes, typ)?,
            )),
            ChunkType::StatePropEventData => Ok(ChunkData::StatePropEventData(
                Name::parse(bytes, typ)?,
                StatePropEventData::parse(bytes, typ)?,
            )),
            ChunkType::StatePropCallbackData => Ok(ChunkData::StatePropCallbackData(
                Name::parse(bytes, typ)?,
                StatePropCallbackData::parse(bytes, typ)?,
            )),
            ChunkType::ObjectAttributes => Ok(ChunkData::ObjectAttributes(
                ObjectAttributes::parse(bytes, typ)?,
            )),
            ChunkType::PropInstanceList => {
                Ok(ChunkData::PropInstanceList(Name::parse(bytes, typ)?))
            }
            // -- Scenegraph -- //
            ChunkType::Scenegraph => Ok(ChunkData::Scenegraph(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphRoot => Ok(ChunkData::None),
            ChunkType::OldScenegraphBranch => Ok(ChunkData::ScenegraphBranch(
                Name::parse(bytes, typ)?,
                ScenegraphBranch::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphTransform => Ok(ChunkData::ScenegraphTransform(
                Name::parse(bytes, typ)?,
                ScenegraphTransform::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphVisibility => Ok(ChunkData::ScenegraphVisibility(
                Name::parse(bytes, typ)?,
                ScenegraphVisibility::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphAttachment => Ok(ChunkData::ScenegraphAttachment(
                Name::parse(bytes, typ)?,
                ScenegraphAttachment::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphAttachmentPoint => Ok(ChunkData::ScenegraphAttachmentPoint(
                ScenegraphAttachmentPoint::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphDrawable => Ok(ChunkData::ScenegraphDrawable(
                Name::parse(bytes, typ)?,
                ScenegraphDrawable::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphCamera => Ok(ChunkData::ScenegraphCamera(
                Name::parse(bytes, typ)?,
                ScenegraphCamera::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphLightGroup => Ok(ChunkData::ScenegraphLightGroup(
                Name::parse(bytes, typ)?,
                ScenegraphLightGroup::parse(bytes, typ)?,
            )),
            ChunkType::OldScenegraphSortOrder => Ok(ChunkData::ScenegraphSortOrder(
                ScenegraphSortOrder::parse(bytes, typ)?,
            )),
            // -- GameAttr -- //
            ChunkType::GameAttr => Ok(ChunkData::GameAttr(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                GameAttr::parse(bytes, typ)?,
            )),
            ChunkType::GameAttrIntParam
            | ChunkType::GameAttrFloatParam
            | ChunkType::GameAttrColourParam
            | ChunkType::GameAttrVectorParam
            | ChunkType::GameAttrMatrixParam => {
                Ok(ChunkData::GameAttrParam(GameAttrParam::parse(bytes, typ)?))
            }
            ChunkType::WBLocator => Ok(ChunkData::WBLocator(
                Name::parse(bytes, typ)?,
                WBLocator::parse(bytes, typ)?,
            )),
            ChunkType::WBTriggerVolume => Ok(ChunkData::WBTriggerVolume(
                Name::parse(bytes, typ)?,
                WBTriggerVolume::parse(bytes, typ)?,
            )),
            ChunkType::WBMatrix => Ok(ChunkData::WBMatrix(WBMatrix::parse(bytes, typ)?)),
            ChunkType::WBSpline => Ok(ChunkData::WBSpline(
                Name::parse(bytes, typ)?,
                WBSpline::parse(bytes, typ)?,
            )),
            ChunkType::WBRail => Ok(ChunkData::WBRail(
                Name::parse(bytes, typ)?,
                WBRail::parse(bytes, typ)?,
            )),
            // -- Other produces error (eventually will produce Unknown) -- //
            typ => Err(eyre!("ChunkData parsing is not implemented for {:?}", typ)),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ChunkData::OldBaseEmitter(_, name, _) => name.name.clone(),
            ChunkData::OldSpriteEmitter(_, name, _) => name.name.clone(),
            ChunkData::OldParticleSystemFactory(_, name, _) => name.name.clone(),
            _ => "".into(),
        }
    }
}
