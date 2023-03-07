use crate::{
    chunk::{
        data::{
            data_enum::ChunkData,
            kinds::{
                animation::{Animation, AnimationGroup, AnimationGroupList, AnimationSize},
                channel::{Channel, ChannelInterpolation},
                collision::{
                    CollisionBoundingBox, CollisionCylinder, CollisionObject,
                    CollisionObjectAttribute, CollisionOblongBox, CollisionSphere, CollisionVector,
                    CollisionVolume, CollisionVolumeOwner, IntersectDSG, TerrainTypeList,
                },
                explosion::BreakableObject,
                export::{P3DExportInfoNamedInt, P3DExportInfoNamedString},
                gameattr::{GameAttr, GameAttrParam},
                image::{Image, ImageRaw},
                locator::{WBLocator, WBMatrix, WBRail, WBSpline, WBTriggerVolume},
                mesh::{
                    BinormalList, ColourList, CompositeDrawable, CompositeDrawableEffect,
                    CompositeDrawableEffectList, CompositeDrawableProp, CompositeDrawablePropList,
                    CompositeDrawableSkin, CompositeDrawableSkinList, CompositeDrawableSortOrder,
                    IndexList, MatrixList, MatrixPalette, Mesh, NormalList, OldPrimGroup,
                    PackedNormalList, PositionList, RenderStatus, Skin, TangentList, UVList,
                    WeightList,
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
                pure3d_other::P3DCamera,
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
            parse_trait::Parse,
        },
        type_identifiers::ChunkType,
    },
    Result,
};
use bytes::Bytes;

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
            | ChunkType::CompressedQuaternionChannel
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
            ChunkType::Skin => Ok(ChunkData::Skin(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                Skin::parse(bytes, typ)?,
            )),
            ChunkType::OldPrimGroup => Ok(ChunkData::OldPrimGroup(
                Version::parse(bytes, typ)?,
                OldPrimGroup::parse(bytes, typ)?,
            )),
            ChunkType::PositionList => {
                Ok(ChunkData::PositionList(PositionList::parse(bytes, typ)?))
            }
            ChunkType::NormalList => Ok(ChunkData::NormalList(NormalList::parse(bytes, typ)?)),
            ChunkType::TangentList => Ok(ChunkData::TangentList(TangentList::parse(bytes, typ)?)),
            ChunkType::BinormalList => {
                Ok(ChunkData::BinormalList(BinormalList::parse(bytes, typ)?))
            }
            ChunkType::PackedNormalList => Ok(ChunkData::PackedNormalList(
                PackedNormalList::parse(bytes, typ)?,
            )),
            ChunkType::UVList => Ok(ChunkData::UVList(UVList::parse(bytes, typ)?)),
            ChunkType::ColourList => Ok(ChunkData::ColourList(ColourList::parse(bytes, typ)?)),
            ChunkType::IndexList => Ok(ChunkData::IndexList(IndexList::parse(bytes, typ)?)),
            ChunkType::MatrixList => Ok(ChunkData::MatrixList(MatrixList::parse(bytes, typ)?)),
            ChunkType::MatrixPalette => {
                Ok(ChunkData::MatrixPalette(MatrixPalette::parse(bytes, typ)?))
            }
            ChunkType::WeightList => Ok(ChunkData::WeightList(WeightList::parse(bytes, typ)?)),
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
            ChunkType::EntityDSG
            | ChunkType::InstanceableAnimatedDynamicPhysicsDSG
            | ChunkType::DynamicPhysicsDSG
            | ChunkType::InstanceableStaticPhysicsDSG => Ok(ChunkData::ObjectDSG(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                ObjectDSG::parse(bytes, typ)?,
            )),
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
            ChunkType::CollisionCylinder => Ok(ChunkData::CollisionCylinder(
                CollisionCylinder::parse(bytes, typ)?,
            )),
            ChunkType::CollisionSphere => Ok(ChunkData::CollisionSphere(CollisionSphere::parse(
                bytes, typ,
            )?)),
            ChunkType::CollisionVector => Ok(ChunkData::CollisionVector(CollisionVector::parse(
                bytes, typ,
            )?)),
            ChunkType::CollisionObjectAttribute => Ok(ChunkData::CollisionObjectAttribute(
                CollisionObjectAttribute::parse(bytes, typ)?,
            )),
            ChunkType::IntersectDSG => {
                Ok(ChunkData::IntersectDSG(IntersectDSG::parse(bytes, typ)?))
            }
            ChunkType::TerrainTypeList => Ok(ChunkData::TerrainTypeList(
                Version::parse(bytes, typ)?,
                TerrainTypeList::parse(bytes, typ)?,
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
            ChunkType::StaticPhysicsDSG => Ok(ChunkData::StaticPhysicsDSG(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
            )),
            // -- Export Info -- //
            ChunkType::P3DExportInfo => Ok(ChunkData::P3DExportInfo(Name::parse(bytes, typ)?)),
            ChunkType::P3DExportInfoNamedString => Ok(ChunkData::P3DExportInfoNamedString(
                Name::parse(bytes, typ)?,
                P3DExportInfoNamedString::parse(bytes, typ)?,
            )),
            ChunkType::P3DExportInfoNamedInt => Ok(ChunkData::P3DExportInfoNamedInt(
                Name::parse(bytes, typ)?,
                P3DExportInfoNamedInt::parse(bytes, typ)?,
            )),
            // -- P3D Other -- //
            ChunkType::P3DCamera => Ok(ChunkData::P3DCamera(
                Name::parse(bytes, typ)?,
                Version::parse(bytes, typ)?,
                P3DCamera::parse(bytes, typ)?,
            )),
            // -- Other produces error (eventually will produce Unknown) -- //
            typ => {
                eprintln!("Error: ChunkData parsing is not implemented for {:?}", typ);
                Ok(ChunkData::Unknown)
                // Err(eyre!("ChunkData parsing is not implemented for {:?}", typ))
            }
        }
    }

    pub fn get_name(&self) -> Option<Name> {
        match self {
            ChunkData::Texture(name, _, _) => Some(name.clone()),
            ChunkData::Image(name, _, _) => Some(name.clone()),
            ChunkData::Shader(name, _, _) => Some(name.clone()),
            ChunkData::Mesh(name, _, _) => Some(name.clone()),
            ChunkData::OldBaseEmitter(_, name, _) => Some(name.clone()),
            ChunkData::OldSpriteEmitter(_, name, _) => Some(name.clone()),
            ChunkData::OldParticleSystemFactory(_, name, _) => Some(name.clone()),
            _ => None,
        }
    }
}
