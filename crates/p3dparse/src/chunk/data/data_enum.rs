use crate::chunk::data::kinds::{
    animation::{Animation, AnimationGroup, AnimationGroupList, AnimationSize},
    channel::{Channel, ChannelInterpolation},
    collision::{
        CollisionBoundingBox, CollisionCylinder, CollisionObject, CollisionObjectAttribute,
        CollisionOblongBox, CollisionSphere, CollisionVector, CollisionVolume,
        CollisionVolumeOwner, IntersectDSG, TerrainTypeList,
    },
    explosion::BreakableObject,
    file_metadata::{ExportInfoNamedInt, ExportInfoNamedString, History},
    game_metadata::{FollowCameraData, Locator},
    gameattr::{GameAttr, GameAttrParam},
    image::{Image, ImageRaw},
    locator::{WBLocator, WBMatrix, WBRail, WBSpline, WBTriggerVolume},
    mesh::{
        BinormalList, ColourList, CompositeDrawable, CompositeDrawableEffect,
        CompositeDrawableEffectList, CompositeDrawableProp, CompositeDrawablePropList,
        CompositeDrawableSkin, CompositeDrawableSkinList, CompositeDrawableSortOrder, IndexList,
        MatrixList, MatrixPalette, Mesh, NormalList, OldPrimGroup, PackedNormalList, PositionList,
        RenderStatus, Skin, TangentList, UVList, WeightList,
    },
    name::Name,
    object::{
        AnimatedObject, AnimatedObjectAnimation, AnimatedObjectDSGWrapper, AnimatedObjectFactory,
        MultiController, MultiControllerTracks, ObjectDSG, OldFrameController,
    },
    old_billboard::{
        OldBillboardDisplayInfo, OldBillboardPerspectiveInfo, OldBillboardQuad,
        OldBillboardQuadGroup,
    },
    old_particle_system::{
        InstanceableParticleSystem, OldBaseEmitter, OldParticleSystem, OldParticleSystemFactory,
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
    pure3d_other::Camera,
    scenegraph::{
        ScenegraphAttachment, ScenegraphAttachmentPoint, ScenegraphBranch, ScenegraphCamera,
        ScenegraphDrawable, ScenegraphLightGroup, ScenegraphSortOrder, ScenegraphTransform,
        ScenegraphVisibility,
    },
    shader::{Shader, VertexShader},
    shader_param::ShaderParam,
    skeleton::{Skeleton, SkeletonJoint, SkeletonJointBonePreserve, SkeletonJointMirrorMap},
    texture::Texture,
    version::Version,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum ChunkData {
    // Others
    None,
    Unknown,
    // Rendering
    Shader(Name, Version, Shader),
    ShaderParam(ShaderParam),
    Texture(Name, Version, Texture),
    Image(Name, Version, Image),
    ImageRaw(ImageRaw),
    VertexShader(VertexShader),
    // Old Particle System
    OldParticleSystem(Version, Name, OldParticleSystem),
    OldParticleSystemFactory(Version, Name, OldParticleSystemFactory),
    OldParticleInstancingInfo(Version, OldParticleSystemInstancingInfo),
    OldParticleAnimation(Version),
    OldEmitterAnimation(Version),
    OldGeneratorAnimation(Version),
    OldBaseEmitter(Version, Name, OldBaseEmitter),
    OldSpriteEmitter(Version, Name, OldSpriteEmitter),
    InstanceableParticleSystem(InstanceableParticleSystem),
    // Animations
    Animation(Version, Name, Animation),
    AnimationSize(Version, AnimationSize),
    AnimationGroup(Version, Name, AnimationGroup),
    AnimationGroupList(Version, AnimationGroupList),
    Channel(Version, Channel),
    ChannelInterpolation(Version, ChannelInterpolation),
    OldFrameController(Version, Name, OldFrameController),
    MultiController(Name, Version, MultiController),
    MultiControllerTracks(MultiControllerTracks),
    // Old Billboards
    OldBillboardQuad(Version, Name, OldBillboardQuad),
    OldBillboardQuadGroup(Version, Name, OldBillboardQuadGroup),
    OldBillboardDisplayInfo(Version, OldBillboardDisplayInfo),
    OldBillboardPerspectiveInfo(Version, OldBillboardPerspectiveInfo),
    // Breakable Objects
    BreakableObject(BreakableObject),
    // Skinning
    Skeleton(Name, Version, Skeleton),
    SkeletonJoint(Name, SkeletonJoint),
    SkeletonJointMirrorMap(SkeletonJointMirrorMap),
    SkeletonJointBonePreserve(SkeletonJointBonePreserve),
    Skin(Name, Version, Skin),
    MatrixList(MatrixList),
    MatrixPalette(MatrixPalette),
    WeightList(WeightList),
    // Meshes
    Mesh(Name, Version, Mesh),
    PrimGroup(Version, OldPrimGroup),
    PositionList(PositionList),
    NormalList(NormalList),
    PackedNormalList(PackedNormalList),
    TangentList(TangentList),
    BinormalList(BinormalList),
    UVList(UVList),
    ColourList(ColourList),
    IndexList(IndexList),
    // Composite Drawables (multiple meshes/skins in one)
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
    ObjectDSG(Name, Version, ObjectDSG),
    AnimatedObjectDSGWrapper(Name, AnimatedObjectDSGWrapper),
    // Physics
    BoundingBox(BoundingBox),
    BoundingSphere(BoundingSphere),
    PhysicsObject(Name, Version, PhysicsObject),
    PhysicsJoint(PhysicsJoint),
    PhysicsVector(PhysicsVector),
    PhysicsInertiaMatrix(PhysicsInertiaMatrix),
    // Collision
    CollisionObject(Name, Version, CollisionObject),
    CollisionVolume(CollisionVolume),
    CollisionVolumeOwner(CollisionVolumeOwner),
    CollisionVolumeOwnerName(Name),
    CollisionBoundingBox(CollisionBoundingBox),
    CollisionOblongBox(CollisionOblongBox),
    CollisionCylinder(CollisionCylinder),
    CollisionSphere(CollisionSphere),
    CollisionVector(CollisionVector),
    CollisionObjectAttribute(CollisionObjectAttribute),
    IntersectDSG(IntersectDSG),
    TerrainTypeList(Version, TerrainTypeList),
    StaticPhysicsDSG(Name, Version),
    // Prop Data
    StatePropDataV1(Version, Name, StatePropDataV1),
    StatePropStateDataV1(Name, StatePropStateDataV1),
    StatePropVisibilitiesData(Name, StatePropVisibilitiesData),
    StatePropFrameControllerData(Name, StatePropFrameControllerData),
    StatePropEventData(Name, StatePropEventData),
    StatePropCallbackData(Name, StatePropCallbackData),
    PropInstanceList(Name),
    ObjectAttributes(ObjectAttributes),
    // Scenegraph
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
    // Game attributes
    GameAttr(Name, Version, GameAttr),
    GameAttrParam(GameAttrParam),
    // Game Metadata
    Locator(Name, Version, Locator),
    FollowCameraData(FollowCameraData),
    // SHAR specific locators (no idea what WB stands for)
    WBLocator(Name, WBLocator),
    WBTriggerVolume(Name, WBTriggerVolume),
    WBMatrix(WBMatrix),
    WBSpline(Name, WBSpline),
    WBRail(Name, WBRail),
    // File Metadata
    ExportInfo(Name),
    ExportInfoNamedString(Name, ExportInfoNamedString),
    ExportInfoNamedInt(Name, ExportInfoNamedInt),
    History(History),
    // Other P3D chunks
    Camera(Name, Version, Camera),
}