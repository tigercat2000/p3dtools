use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Directly stolen from LucasStuff.Radical
/// And the Simpsons Hit & Run Source code leak!
#[repr(u32)]
#[derive(IntoPrimitive, TryFromPrimitive, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum ChunkType {
    // Old Pure3D
    // 0x0000 - 0x0FFF     unused
    Grid = 0x00001000,
    GridCell = 0x00001001,
    Locator3 = 0x00001003,
    Trigger2 = 0x00001004,
    RoadNode2 = 0x00001005,
    GroundCollision2 = 0x00001008,
    GroundCollision3 = 0x00001009,
    Unknown1010 = 0x00001010,
    Unknown1011 = 0x00001011,
    Unknown1013 = 0x00001013,
    Unknown1014 = 0x00001014,
    Unknown1021 = 0x00001021,
    Unknown1022 = 0x00001022,
    LocatorCounts = 0x00001023,
    Unknown1024 = 0x00001024,
    BlackMagic = 0x00001025,
    // 0x2000 - 0x2FFF     Various data
    P3DMatrix = 0x00002000,
    P3DPosRot = 0x00002001,
    P3DColorRgb = 0x00002002,
    P3DColorRgba = 0x00002003,
    P3DFov = 0x00002004,
    P3DDirection = 0x00002005,
    P3DPosition = 0x00002006,
    P3DRotationAxis = 0x00002007,
    P3DBox = 0x00002008,
    P3DSphere = 0x00002009,
    P3DPlane = 0x0000200A,
    P3DParameters = 0x0000200B,
    P3DSphereList = 0x0000200C,
    P3DParticleSystem = 0x00002100,
    P3DPointEmitter = 0x00002101,
    P3DSpriteEmitter = 0x00002102,
    P3DParticleLifeChannel = 0x00002110,
    P3DParticleSpeedChannel = 0x00002111,
    P3DParticleWeightChannel = 0x00002112,
    P3DParticleLifeVarChannel = 0x00002113,
    P3DParticleSpeedVarChannel = 0x00002114,
    P3DParticleWeightVarChannel = 0x00002115,
    P3DParticleLifeOlChannel = 0x00002116,
    P3DParticleSpeedOlChannel = 0x00002117,
    P3DParticleWeightOlChannel = 0x00002118,
    P3DParticleNumParticlesChannel = 0x00002119,
    P3DParticleEmissionRateChannel = 0x0000211A,
    P3DParticleSizeChannel = 0x0000211B,
    P3DParticleSpinChannel = 0x0000211C,
    P3DParticleTransparencyChannel = 0x0000211D,
    P3DParticleColourChannel = 0x0000211E,
    P3DParticleSizeVarChannel = 0x0000211F,
    P3DParticleSpinVarChannel = 0x00002120,
    P3DParticleTransparencyVarChannel = 0x00002121,
    P3DParticleColourVarChannel = 0x00002122,
    P3DParticleSizeOlChannel = 0x00002123,
    P3DParticleSpinOlChannel = 0x00002124,
    P3DParticleTransparencyOlChannel = 0x00002125,
    P3DParticleColourOlChannel = 0x00002126,
    P3DParticleChannel = 0x00002127,
    P3DParticlePointGenerator = 0x00002128,
    P3DParticlePlaneGenerator = 0x00002129,
    P3DParticleSphereGenerator = 0x0000212A,
    P3DParticleGravityChannel = 0x0000212B,
    P3DParticleGeneratorHorizSpread = 0x0000212E,
    P3DParticleGeneratorVertSpread = 0x0000212F,
    P3DParticlePositionChannel = 0x00002130,
    P3DParticleRotationChannel = 0x00002131,
    P3DCamera = 0x00002200,
    P3DLightGroup = 0x00002380,
    P3DV12GeoMesh = 0x00003000,
    P3DV12GeoVertexList = 0x00003001,
    P3DV12GeoFaceListTex = 0x00003005,
    P3DV12GeoUvList = 0x00003006,
    P3DV12GeoNormalList = 0x00003007,
    P3DV12GeoMaterialGroup = 0x00003008,
    P3DV12GeoHit = 0x00003009,
    P3DV12GeoFlags = 0x0000300A,
    P3DV12GeoAnimVertexList = 0x0000300B,
    P3DV12GeoAnimNormalList = 0x0000300C,
    P3DV12GeoColourList = 0x0000300D,
    P3DV12GeoVertexColourList = 0x0000300E,
    P3DV12GeoProTexture = 0x00003010,
    P3DV12GeoProTexPal = 0x00003011,
    P3DV12GeoProTexPixels = 0x00003012,
    P3DV12GeoProAlphaPixels = 0x00003013,
    P3DV12GeoProMaterial = 0x00003020,
    P3DV12GeoProMatColour = 0x00003021,
    P3DV12GeoProMatTexture = 0x00003022,
    P3DV12GeoProMatTransp = 0x00003023,
    P3DV12GeoProMatBlendmode = 0x00003024,
    P3DFont = 0x00003062,
    P3DFontGlyphs = 0x00003063,
    P3DTextureFont = 0x00003064,
    P3DTextureGlyph = 0x00003065,
    P3DImageFont = 0x00003066,
    P3DImageGlyph = 0x00003067,
    P3DV12Mesh = 0x00003100,
    P3DV12VertexList = 0x00003101,
    P3DV12NormalList = 0x00003102,
    P3DV12UvList = 0x00003103,
    P3DV12ColourList = 0x00003104,
    P3DV12MaterialList = 0x00003105,
    P3DV12FaceList = 0x00003106,
    P3DV12PrimGroup = 0x00003107,
    P3DV12FaceNormalList = 0x00003108,
    P3DV12EdgeList = 0x000031a9,
    P3DV12Skin = 0x00003700,
    P3DV12BoneWeighting = 0x00003701,
    P3DV12Material = 0x00003120,
    P3DV12MaterialPass = 0x00003125,
    P3DV14Shader = 0x00003130,
    P3DV14ShaderDefinition = 0x00003131,
    P3DV14ShaderTextureParam = 0x00003132,
    P3DV14ShaderIntParam = 0x00003133,
    P3DV14ShaderFloatParam = 0x00003134,
    P3DV14ShaderColourParam = 0x00003135,
    P3DV14ShaderVectorParam = 0x00003136,
    P3DV14ShaderMatrixParam = 0x00003137,
    P3DTriStripMesh = 0x00003200,
    P3DTriStrip = 0x00003201,
    P3DBackground = 0x00003300,
    P3DBMPImageRef = 0x00003400,
    P3DTexture = 0x00003500,
    P3DImage = 0x00003510,
    P3DImageData = 0x00003511,
    P3DImageFilename = 0x00003512,
    P3DTextureAnimation = 0x00003520,
    P3DTextureAnimationChannel = 0x00003521,
    P3DHspline = 0x00003E00,
    P3DHSplineSbList = 0x00003E10,
    P3DHSplineStorageBlock = 0x00003E11,
    P3DHSplineGnList = 0x00003E30,
    P3DHSplineGraftingNode = 0x00003E31,
    P3DHSplineContribList = 0x00003E40,
    P3DHSplineContributor = 0x00003E41,
    P3DHSplineEdgeList = 0x00003E50,
    P3DHSplineEdge = 0x00003E51,
    P3DHSplineOffsetList = 0x00003E60,
    P3DHSplineOffset = 0x00003E61,
    P3DHSplineOffsetAdd = 0x00003E62,
    P3DHSplineOffsetTangent = 0x00003E63,
    P3DHSplineOffsetJoint = 0x00003E64,
    P3DHSplineOffsetPhantom = 0x00003E65,
    P3DHSplineOffsetFrame = 0x00003E66,
    P3DHSplineCopyList = 0x00003E70,
    P3DHSplineCopyCn = 0x00003E71,
    P3DHSplineControlNode = 0x00003E81,
    P3DHSplineCcpatchList = 0x00003E90,
    P3DHSplineCcpatch = 0x00003E91,
    P3DHSplineRefFrameList = 0x00003EA0,
    P3DHSplineRefCn = 0x00003EA1,
    P3DHSplineTree = 0x00003EF0,
    P3DHSplineTreeJoint = 0x00003EF1,
    P3DHSplineTreeMappedHstree = 0x00003EF2,
    P3DHSplineTreeMapping = 0x00003EF3,
    P3DHSplineTreeRestPose = 0x00003EF4,
    P3DHSplineTreeParentIndex = 0x00003EF5,
    P3DHSplineStitcher = 0x00003F00,
    P3DHSplineStitch = 0x00003F01,
    P3DHSplineStitchPatch = 0x00003F02,
    P3DHSplineStitchPatchlist = 0x00003F03,
    P3DHSplineStitchTargetlist = 0x00003F04,
    P3DHSplineStitchSkin = 0x00003F05,
    P3DHSplineTessellation = 0x00003F10,
    P3DHSplineIndexMapping = 0x00003F11,
    P3DHSplineSkin = 0x00003F20,
    P3DHSplineSkinOffsetGroup = 0x00003F21,
    P3DHSplineSkinConnect = 0x00003F22,
    P3DHSplineSkinVertConnect = 0x00003F23,
    P3DHSplinePolyskin = 0x00003F30,
    P3DHSplineOffsetAnim = 0x00003F40,
    P3DHSplineAnimChannel = 0x00003F41,
    P3DHSplineChannelOffsetDynamic = 0x00003F42,
    P3DHSplineChannelOffsetStatic = 0x00003F43,
    GeoAnimation = 0x00004001,
    GeoAnimationJoint = 0x00004002,
    GeoAnimationTranslList = 0x00004003,
    GeoAnimationRotateList = 0x00004004,
    GeoAnimationQuatRotateList = 0x00004010,
    GeoAnimationScaleList = 0x00004005,
    GeoAnimationClut = 0x00004006,
    P3DDeformPolyskin = 0x00004A88,
    P3DDeformPolyskinJoint = 0x00004A89,
    P3DDeformPolyskinState = 0x00004A8A,
    GeoCompositeAnimation = 0x00004007,
    GeoAnimationTex = 0x00004008,
    GeoAnimationRootTrans = 0x00004009,
    GeoAnimationVert = 0x0000400A,
    GeoAnimationVertSphere = 0x0000400B,
    GeoAnimationVertFrames = 0x0000400C,
    GeoAnimationCvert = 0x0000400D,
    GeoAnimationCvertSphere = 0x0000400E,
    GeoAnimationCvertFrames = 0x0000400F,
    GeoAnimationTreetype = 0x00004011,
    AnimationSeq = 0x00004012,
    P3DVizAnimation = 0x00004020,
    P3DVizAnimationData = 0x00004021,
    P3DUvAnimation = 0x00004030,
    P3DUvAnimationFrames = 0x00004031,
    P3DCbvAnimation = 0x00004040,
    P3DCbvAnimationFrames = 0x00004041,
    P3DCbvParamAnimation = 0x00004050,
    P3DCbvParamAnimationFrames = 0x00004051,
    P3DEventAnimation = 0x00004060,
    P3DEventAnimationEvent = 0x00004061,
    P3DEventAnimationData = 0x00004062,
    MtrMtree = 0x4100,
    MtrMtreeJoint = 0x4101,
    MtrBillboard = 0x4110,
    StrStree = 0x4120,
    StrStreeJoint = 0x4121,
    StrMappedStree = 0x4122,
    StrStreeMapping = 0x4123,
    StrStreeWeighting = 0x4124,
    StrStreeRestPose = 0x4125,
    StrStreeParentIndex = 0x4126,
    P3DTranAnim = 0x00004200,
    P3DJointList = 0x00004201,
    P3DJoint = 0x00004202,
    P3DTimeIndex = 0x00004203,
    P3DJointNames = 0x00004204,
    P3DJointInfo = 0x00004205,
    P3DKeylist1Dof = 0x00004210,
    P3DKeylist2Dof = 0x00004211,
    P3DKeylist3Dof = 0x00004212,
    P3DKeylist1DofAngle = 0x00004213,
    P3DKeylist2DofAngle = 0x00004214,
    P3DKeylist3DofAngle = 0x00004215,
    P3DKeyListColour = 0x00004216,
    P3DKeylistQuat = 0x00004217,
    P3DKeylistRot = 0x00004218,
    P3DKeylistScalematrix = 0x00004219,
    P3DStaticRotKeylist = 0x00004220,
    P3DStaticTransKeylis = 0x00004221,
    P3DStaticScaleKeylis = 0x00004222,
    P3DStaticQuatKeylist = 0x00004223,
    P3DStaticScalematrix = 0x00004224,
    P3DStaticRotation = 0x00004225,
    P3DStaticTranslation = 0x00004226,
    P3DKeylistHsOff3Dof = 0x00004230,
    P3DVisibilityAnim = 0x00004290,
    P3DVisibilityAnimChannel = 0x00004291,
    P3DEntityAnimChannel = 0x000042A0,
    P3DParamAnim = 0x4300,
    P3DParamAnimParam = 0x4301,
    P3DHsplineParamAnim = 0x4400,
    P3DSkeleton = 0x00004500,
    P3DSkeletonJoint = 0x00004501,
    P3DSkeletonJointMirrorMap = 0x00004503,
    P3DSkeletonJointBonePreserve = 0x00004504,
    P3DCompositeDrawable = 0x00004512,
    P3DCompositeDrawableSkinList = 0x00004513,
    P3DCompositeDrawablePropList = 0x00004514,
    P3DCompositeDrawableSkin = 0x00004515,
    P3DCompositeDrawableProp = 0x00004516,
    P3DCompositeDrawableEffectList = 0x00004517,
    P3DCompositeDrawableEffect = 0x00004518,
    P3DCompositeDrawableSortOrder = 0x00004519,
    P3DFrameController = 0x00004520,
    P3DV12PoseAnim = 0x00004700,
    P3DV12JointList = 0x00004701,
    P3DV12AnimChannel = 0x00004702,
    P3DV12PoseAnimMirrored = 0x00004703,
    P3DChannel1DOF = 0x00004800,
    P3DChannel3DOF = 0x00004801,
    P3DChannel1DOFAngle = 0x00004802,
    P3DChannel3DOFAngle = 0x00004803,
    P3DChannelStatic = 0x00004804,
    P3DChannelStaticAngle = 0x00004805,
    P3DChannelQuaternion = 0x00004806,
    P3DChannelStaticQuaternion = 0x00004807,
    P3DMultiController = 0x000048A0,
    P3DMultiControllerTracks = 0x000048A1,
    P3DMultiControllerTrack = 0x000048A2,
    P3DCameraAnim = 0x00004900,
    P3DCameraAnimChannel = 0x00004901,
    P3DCameraAnimPosChannel = 0x00004902,
    P3DCameraAnimLookChannel = 0x00004903,
    P3DCameraAnimUpChannel = 0x00004904,
    P3DCameraAnimFOVChannel = 0x00004905,
    P3DLightAnim = 0x00004980,
    P3DLightAnimChannel = 0x00004981,
    P3DLightAnimColourChannel = 0x00004982,
    P3DLightAnimParamChannel = 0x00004983,
    P3DLightAnimEnableChannel = 0x00004985,
    P3DVertexAnim = 0x00004A00,
    P3DVertexAnimChannel = 0x00004A01,
    P3DExpressionAnim = 0x00004A10,
    P3DExpressionAnimChannel = 0x00004A11,
    P3DExpressionMixer = 0x00004A20,
    P3DVertexOffset = 0x00004A80,
    P3DVertexOffsetAnim = 0x00004A81,
    P3DVertexOffsetExpression = 0x00004A82,
    P3DProgessiveMeshMesh = 0x00005000,
    P3DProgessiveMeshSkin = 0x00005001,
    P3DProgessiveMeshPrimGroup = 0x00005002,
    P3DProgessiveMeshHistory = 0x00005005,
    P3DProgessiveMeshHistoryElement = 0x00005006,
    P3DViewDependentProgessiveMeshGeo = 0x00005010,
    P3DViewDependentProgessiveMeshStree = 0x00005011,
    P3DViewDependentProgessiveMeshHistory = 0x00005012,
    P3DViewDependentProgessiveMeshJointHistory = 0x00005013,
    P3DViewDependentProgessiveMeshHistoryLevel = 0x00005014,
    PSXVersion = 0x00006000,
    PSXMaterials = 0x00006001,
    PSXGeometry = 0x00006002,
    PSXCollisionGeom = 0x00006003,
    PSXVertAnim = 0x00006004,
    PSXNormAnim = 0x00006005,
    PSXClutAnim = 0x00006006,
    PSXTexAnim = 0x00006007,
    PSXTexture = 0x00006008,
    PSXPrims = 0x00006009,
    PSXTexAnimFrames = 0x0000600A,
    PSXTexAnimOffsets = 0x0000600B,
    PSXClutAnimFrames = 0x0000600C,
    PSXClutAnimOffsets = 0x0000600D,
    PSXUvAnim = 0x0000600E,
    PSXUvAnimFrames = 0x0000600F,
    PSXUvAnimOffsets = 0x00006010,
    PSXCbvAnim = 0x00006011,
    PSXCbvAnimFrames = 0x00006012,
    PSXCbvAnimOffsets = 0x00006013,
    PSXCbvParamAnim = 0x00006021,
    PSXCbvParamAnimFrames = 0x00006022,
    PSXCbvParamAnimOffsets = 0x00006023,
    PSXSequenceAnim = 0x00006040,
    PSXMainRamTexAnim = 0x00006050,
    PSXMainRamTexAnimNames = 0x00006051,
    PSXMainRamTexAnimFrames = 0x00006052,
    PSXStree = 0x00006120,
    PSXStreeJoint = 0x00006121,
    PSXMappedStree = 0x00006122,
    PSXStreeWeighting = 0x00006124,
    PSXStreeRestPose = 0x00006125,
    PSXMtree = 0x00006130,
    PSXMtreeJoint = 0x00006131,
    PSXEtree = 0x00006140,
    PSXEtreeJoint = 0x00006141,
    PSXTranAnim = 0x00006400,
    PSXTextureRef = 0x00006500,
    PSXPrimOffsets = 0x00006600,
    PSXMatrix = 0x00006F00,
    P3DHistory = 0x00007000,
    P3DAlign = 0x00007001,
    P3DExportInfo = 0x00007030,
    P3DExportInfoNamedString = 0x00007031,
    P3DExportInfoNamedInt = 0x00007032,
    P3DSgScenegraph = 0x9100,
    P3DSgRoot = 0x9101,
    P3DSgBranch = 0x9102,
    P3DSgTransform = 0x9103,
    P3DSgDrawable = 0x9104,
    P3DSgCamera = 0x9105,
    P3DSgLightgroup = 0x9106,
    P3DSgAttachment = 0x9107,
    P3DSgAttachmentpoint = 0x9108,
    P3DSgVisibility = 0x9109,
    P3DSgTransformAnim = 0x00009150,
    P3DSgTransformController = 0x9151,
    PhyObjectOld = 0x0000C000,
    PhyInertiaMatrix = 0x0000C001,
    PhyCollider = 0x0000C002,
    PhyColliderSphere = 0x0000C003,
    PhyColliderCylinder = 0x0000C004,
    PhyColliderOBBox = 0x0000C005,
    PhyColliderWall = 0x0000C006,
    PhyColliderBBox = 0x0000C007,
    PhyVector = 0x0000C010,
    PhyObjJoint = 0x0000C011,
    PhyObjJointDOF = 0x0000C012,
    PhyObjSelfCollision = 0x0000C020,
    PhyObjSelfCollisionItem = 0x0000C021,
    PhyFootsteps = 0x0000C1000,
    PhyObject = 0x0000C111,
    PhyFlexGeom = 0x0000C200,
    PhyFlexJoint = 0x0000C201,
    PhyFlexParam = 0x0000C210,
    PhyFlexFixParticle = 0x0000C211,
    PhyFlexMapVL = 0x0000C212,
    PhyFlexTriMap = 0x0000C213,
    PhyFlexEdgeMap = 0x0000C214,
    PhyFlexEdgeLen = 0x0000C215,
    PhyFlexCollJoint = 0x0000C216,
    PhyFlexJointDef = 0x0000C220,
    PhyLink = 0x0000C320,
    PhyLinkIK = 0x0000C321,
    PhyLinkReach = 0x0000C322,
    PhyLinkTracker = 0x0000C323,
    PhyLinkTarget = 0x0000C330,
    PhyTargetNode = 0x0000C331,
    PhyTargetPose = 0x0000C332,
    Mesh = 0x00010000,
    Skin = 0x00010001,
    OldPrimGroup = 0x00010002,
    BBox = 0x00010003,
    BSphere = 0x00010004,
    PositionList = 0x00010005,
    NormalList = 0x00010006,
    UVList = 0x00010007,
    ColourList = 0x00010008,
    IndexList = 0x0001000A,
    MatrixList = 0x0001000B,
    WeightList = 0x0001000C,
    MatrixPalette = 0x0001000D,
    OldOffsetList = 0x0001000E,
    InstanceInfo = 0x0001000F,
    PackedNormalList = 0x00010010,
    VertexShader = 0x00010011,
    PrimGroupMemoryImageVertex = 0x00010012,
    PrimGroupMemoryImageIndex = 0x00010013,
    PrimGroupMemoryImageVertexDescription = 0x00010014,
    TangentList = 0x00010015,
    BiNormalList = 0x00010016,
    RenderStatus = 0x00010017,
    OldExpressionOffsets = 0x00010018,
    ShadowSkin = 0x00010019,
    ShadowMesh = 0x0001001A,
    Topology = 0x0001001B,
    MultiColourList = 0x0001001C,
    MeshStats = 0x0001001D,
    PrimGroup = 0x00010020,
    VertexCompressionHint = 0x00010021,
    Shader = 0x00011000,
    ShaderDefinition = 0x00011001,
    ShaderTextureParam = 0x00011002,
    ShaderIntParam = 0x00011003,
    ShaderFloatParam = 0x00011004,
    ShaderColourParam = 0x00011005,
    ShaderVectorParam = 0x00011006,
    ShaderMatrixParam = 0x00011007,
    GameAttr = 0x00012000,
    GameAttrIntParam = 0x00012001,
    GameAttrFloatParam = 0x00012002,
    GameAttrColourParam = 0x00012003,
    GameAttrVectorParam = 0x00012004,
    GameAttrMatrixParam = 0x00012005,
    Light = 0x00013000,
    LightDirection = 0x00013001,
    LightPosition = 0x00013002,
    LightConeParam = 0x00013003,
    LightShadow = 0x00013004,
    LightPhotonMap = 0x00013005,
    LightDecayRange = 0x00013006,
    LightDecayRangeRotationY = 0x00013007,
    LightIlluminationType = 0x00013008,
    Locator = 0x00014000,
    V14ParticleSystem = 0x00015000,
    V14ParticleSystemUnknown15101 = 0x00015101,
    V14ParticleSystemUnknown15102 = 0x00015102,
    V14ParticleSystemUnknown15103 = 0x00015103,
    V14ParticleSystemUnknown15140 = 0x00015140,
    V14ParticleSystemUnknown15200 = 0x00015200,
    V14ParticleSystemUnknown15210 = 0x00015210,
    V14ParticleSystemUnknown15211 = 0x00015211,
    V14ParticleSystemUnknown15212 = 0x00015212,
    V14ParticleSystemUnknown15213 = 0x00015213,
    V14ParticleSystemUnknown15214 = 0x00015214,
    V14ParticleSystemUnknown15215 = 0x00015215,
    V14ParticleSystemUnknown15216 = 0x00015216,
    V14ParticleSystemUnknown15217 = 0x00015217,
    V14ParticleSystemUnknown15218 = 0x00015218,
    V14ParticleSystemUnknown15219 = 0x00015219,
    V14ParticleSystemUnknown1521A = 0x0001521A,
    V14ParticleSystemUnknown1521B = 0x0001521B,
    V14ParticleSystemUnknown1521C = 0x0001521C,
    V14ParticleSystemUnknown1521D = 0x0001521D,
    V14ParticleSystemUnknown1521E = 0x0001521E,
    V14ParticleSystemUnknown1521F = 0x0001521F,
    V14ParticleSystemUnknown15220 = 0x00015220,
    V14ParticleSystemUnknown15221 = 0x00015221,
    V14ParticleSystemUnknown15222 = 0x00015222,
    V14ParticleSystemUnknown15223 = 0x00015223,
    V14ParticleSystemUnknown15224 = 0x00015224,
    V14ParticleSystemUnknown15225 = 0x00015225,
    V14ParticleSystemUnknown15226 = 0x00015226,
    V14ParticleSystemUnknown15227 = 0x00015227,
    V14ParticleSystemUnknown15228 = 0x00015228,
    V14ParticleSystemUnknown15229 = 0x00015229,
    V14ParticleSystemUnknown15400 = 0x00015400,
    V14ParticleSystemUnknown15401 = 0x00015401,
    V14ParticleSystemUnknown15402 = 0x00015402,
    V14ParticleSystemUnknown15501 = 0x00015501,
    V14ParticleSystemUnknown15502 = 0x00015502,
    /// Originally OldParticleSystem
    OldParticleSystemFactory = 0x00015800,
    /// Originally WorldEffect
    OldParticleSystem = 0x00015801,
    OldBaseParticleArray = 0x00015802,
    OldSpriteParticleArray = 0x00015803,
    OldDrawableParticleArray = 0x00015804,
    OldBaseEmitter = 0x00015805,
    OldSpriteEmitter = 0x00015806,
    OldDrawableEmitter = 0x00015807,
    OldParticleAnimation = 0x00015808,
    OldEmitterAnimation = 0x00015809,
    OldGeneratorAnimation = 0x0001580A,
    OldParticleInstancingInfo = 0x0001580B,
    ParticleSystem = 0x0001580C,
    SpriteParticleEmitter = 0x00015900,
    ParticlePointGenerator = 0x00015B00,
    Unknown15F00 = 0x00015F00,
    OpticEffectCoronaV14 = 0x00016000,
    OpticEffectLensFlareParentV14 = 0x00016001,
    OpticEffectLensFlareV14 = 0x00016002,
    OpticEffectVectorV14 = 0x00016f00,
    OpticEffectLensFlareGroup = 0x00016006,
    OpticEffectLensFlare = 0x00016007,
    OldBillboardQuadV14 = 0x00017000,
    OldBillboardQuad = 0x00017001,
    OldBillboardQuadGroup = 0x00017002,
    OldBillboardDisplayInfo = 0x00017003,
    OldBillboardPerspectiveInfo = 0x00017004,
    Unknown17005 = 0x00017005,
    BillboardQuadGroup = 0x00017006,
    Unknown17007 = 0x00017007,
    Unknown17009 = 0x00017009,
    Unknown1700A = 0x0001700A,
    Unknown1700D = 0x0001700D,
    FrontendProject = 0x00018000,
    FrontendScreen = 0x00018001,
    FrontendPage = 0x00018002,
    FrontendLayer = 0x00018003,
    FrontendGroup = 0x00018004,
    FrontendMovie = 0x00018005,
    FrontendMultiSprite = 0x00018006,
    FrontendMultiText = 0x00018007,
    FrontendPure3DObject = 0x00018008,
    FrontendPolygon = 0x00018009,
    FrontendSprite = 0x0001800A,
    FrontendStringTextBible = 0x0001800B,
    FrontendStringHardCoded = 0x0001800C,
    FrontendTextBible = 0x0001800D,
    FrontendLanguage = 0x0001800E,
    FrontendImageResource = 0x00018100,
    FrontendPure3DResource = 0x00018101,
    FrontendOldResourceTextStyle = 0x00018102,
    FrontendOldResourceTextBible = 0x00018103,
    FrontendTextStyleResource = 0x00018104,
    FrontendTextBibleResource = 0x00018105,
    Texture = 0x00019000,
    Image = 0x00019001,
    ImageData = 0x00019002,
    ImageFilename = 0x00019003,
    VolumeImage = 0x00019004,
    Sprite = 0x00019005,
    AnimatedObjectFactory = 0x00020000,
    AnimatedObject = 0x00020001,
    AnimatedObjectAnimation = 0x00020002,
    Expression = 0x00021000,
    ExpressionGroup = 0x00021001,
    ExpressionMixer = 0x00021002,
    TextureFont = 0x00022000,
    TextureGlyphList = 0x00022001,
    ImageFont = 0x00022002,
    ImageGlyphList = 0x00022003,
    Skeleton2 = 0x00023000,
    SkeletonJoint2 = 0x00023001,
    SkeletonPartition = 0x00023002,
    SkeletonLimb = 0x00023003,
    Scenegraph = 0x00120100,
    OldScenegraphRoot = 0x00120101,
    OldScenegraphBranch = 0x00120102,
    OldScenegraphTransform = 0x00120103,
    OldScenegraphVisibility = 0x00120104,
    OldScenegraphAttachment = 0x00120105,
    OldScenegraphAttachmentPoint = 0x00120106,
    OldScenegraphDrawable = 0x00120107,
    OldScenegraphCamera = 0x00120108,
    OldScenegraphLightGroup = 0x00120109,
    OldScenegraphSortOrder = 0x0012010A,
    ScenegraphRoot = 0x0012010B,
    ScenegraphBranch = 0x0012010C,
    ScenegraphTransform = 0x0012010D,
    ScenegraphDrawable = 0x0012010F,
    Animation = 0x00121000,
    AnimationGroup = 0x00121001,
    AnimationGroupList = 0x00121002,
    AnimationSize = 0x00121004,
    AnimationHeader = 0x00121006,
    AnimationChannelCount = 0x00121007,
    Float1Channel = 0x00121100,
    Float2Channel = 0x00121101,
    Vector1DOFChannel = 0x00121102,
    Vector2DOFChannel = 0x00121103,
    Vector3DOFChannel = 0x00121104,
    QuaternionChannel = 0x00121105,
    StringChannel = 0x00121106,
    EntityChannel = 0x00121107,
    BoolChannel = 0x00121108,
    ColourChannel = 0x00121109,
    EventChannel = 0x0012110A,
    EventObjectChannel = 0x0012110B,
    EventObjectDataChannel = 0x0012110C,
    EventObjectDataImageChannel = 0x0012110D,
    IntChannel = 0x0012110E,
    QuaternionFormatChannel = 0x0012110F,
    ChannelInterpolationMode = 0x00121110,
    CompressedQuaternionChannel = 0x00121111,
    OldFrameController = 0x00121200,
    FrameController = 0x00121201,
    MultiController2 = 0x00121202,
    Unknown121203 = 0x00121203,
    Unknown121204 = 0x00121204,
    OldColourOffsetList = 0x00121300,
    OldVectorOffsetList = 0x00121301,
    OldVector2OffsetList = 0x00121302,
    OldIndexOffsetList = 0x00121303,
    OldVertexAnimKeyFrame = 0x00121304,
    AnimationListVector = 0x00121400,
    AnimationListVector2 = 0x00121401,
    AnimationKeyFrame = 0x00121402,
    SortOrder = 0x00122000,
    CompositeDrawable2 = 0x00123000,
    CompositeDrawablePrimitive = 0x00123001,
    // Simulation System 0x07000000 - 0x07ffffff
    CollisionObject = 0x07010000,
    CollisionVolume = 0x07010001,
    CollisionSphere = 0x07010002,
    CollisionCylinder = 0x07010003,
    /// Originally CollisionOBB
    CollisionOblongBox = 0x07010004,
    CollisionWall = 0x07010005,
    /// Originally CollisionAABB
    CollisionBoundingBox = 0x07010006,
    CollisionVector = 0x07010007,
    CollisionVolumeOwner = 0x07010021,
    CollisionVolumeOwnerName = 0x07010022,
    CollisionObjectAttribute = 0x07010023,
    PhysicsObject = 0x07011000,
    PhysicsInertiaMatrix = 0x07011001,
    PhysicsVector = 0x07011002,
    PhysicsJoint = 0x07011020,
    StatePropDataV1 = 0x08020000,
    StatePropStateDataV1 = 0x08020001,
    StatePropVisibilitiesData = 0x08020002,
    StatePropFrameControllerData = 0x08020003,
    StatePropEventData = 0x08020004,
    StatePropCallbackData = 0x08020005,
    /// Originally Root
    DataFile = 0xFF443350,
    DataFileCompressed = 0x5A443350,
    // SRR2
    Wall = 0x03000000,
    FenceLine = 0x03000001,
    RoadNodeSegment = 0x03000002,
    RoadNode = 0x03000003,
    IntersectionLocatorNode = 0x03000004,
    /// Originally SRR2Locator
    WBLocator = 0x03000005,
    /// Originally TriggerVolume
    WBTriggerVolume = 0x03000006,
    /// Originally Spline
    WBSpline = 0x03000007,
    PropInstanceList = 0x03000008,
    /// Originally CubeShape
    RoadSegmentData = 0x03000009,
    /// Originally Rail
    WBRail = 0x0300000A,
    PedNode = 0x0300000B,
    /// Really "EXTRA_MATRIX"
    /// Originally SRR2LocatorMatrix
    WBMatrix = 0x0300000C,
    PedNodeSegment = 0x0300000D,
    TerrainTypeList = 0x0300000E,
    CarCameraData = 0x03000100,
    WalkerCameraData = 0x03000101,
    /// Originally RandomTexture
    SFXChunkSet = 0x03000110,
    /// Originally CollisionEffect
    ObjectAttributes = 0x03000600,
    PhysicsWrapper = 0x03000601,
    AttributeTable = 0x03000602,
    /// Originally ExplosionEffectType
    BreakableObject = 0x03001000,
    /// Originally ParticleEmitterType
    InstanceableParticleSystem = 0x03001001,
    /// Originally StaticWorldMesh
    EntityDSG = 0x03F00000,
    /// Originally StaticMeshCollision
    StaticPhysicsDSG = 0x03F00001,
    /// Originally BreakableWorldProp
    DynamicPhysicsDSG = 0x03F00002,
    /// Originally GroundCollision
    IntersectDSG = 0x03F00003,
    /// Originally SectorList
    TreeDSG = 0x03F00004,
    /// Originally SectorContainer
    ContiguousBinNode = 0x03F00005,
    /// Originally Sector
    SpatialNode = 0x03F00006,
    /// Originally WallCollisionContainer
    FenceDSG = 0x03F00007,
    /// Originally DynamicWorldMesh
    AnimatedColliderDSG = 0x03F00008,
    /// Originally StaticCollisionlessWorldProp
    InstanceableEntityDSG = 0x03F00009,
    /// Originally StaticWorldProp
    InstanceableStaticPhysicsDSG = 0x03F0000A,
    /// Originally WorldSky
    WorldSphereDSG = 0x03F0000B,
    /// Originally WorldMesh
    AnimatedDSG = 0x03F0000C,
    LensFlareDSG = 0x03F0000D,
    /// Originally BreakableWorldProp2
    InstanceableAnimatedDynamicPhysicsDSG = 0x03F0000E,
    /// Originally BreakableDrawable2
    AnimatedDSGWrapper = 0x03F0000F,
    /// Originally BreakableDrawable
    AnimatedObjectDSGWrapper = 0x03F00010,
}
