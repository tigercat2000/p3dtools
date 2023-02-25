use eyre::eyre;
use p3dparse::chunk::{
    data::{
        kinds::{
            self,
            mesh::{OldPrimGroup, PrimitiveType},
            shader_param::ShaderParam,
            shared::{Matrix, Vector2, Vector3},
        },
        ChunkData,
    },
    type_identifiers::ChunkType,
    Chunk,
};

pub type Result<T> = std::result::Result<T, eyre::Error>;

pub trait FromChunk<'a> {
    type Output;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Shader<'a> {
    pub name: &'a str,
    pub params: Vec<&'a ShaderParam>,
}

impl<'a> Shader<'a> {
    fn from_data(chunk: &'a Chunk, name: &'a str, params: u32, tree: &'a [Chunk]) -> Result<Self> {
        let mut shader = Shader {
            name,
            params: Vec::with_capacity(params as usize),
        };

        for child in chunk.get_children(tree) {
            if let ChunkData::ShaderParam(param) = &child.data {
                shader.params.push(param)
            }
        }

        Ok(shader)
    }
}

impl<'a> FromChunk<'a> for Shader<'a> {
    type Output = Shader<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::Shader, ChunkData::Shader(name, _version, shader)) => {
                Shader::from_data(chunk, &name.0, shader.num_params, tree)
            }
            (typ, data) => Err(eyre!(
                "Shader expected ChunkType::Shader with ChunkData::Shader but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

/// Used for both [`Mesh`] and [`Skin`]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PrimGroup<'a> {
    pub shader: Option<Shader<'a>>,
    pub primitive_type: PrimitiveType,
    pub vertices: Option<&'a Vec<Vector3>>,
    pub normals: Option<&'a Vec<Vector3>>,
    pub tangents: Option<&'a Vec<Vector3>>,
    pub binormals: Option<&'a Vec<Vector3>>,
    pub indices: Option<&'a Vec<u32>>,
    pub uv_map: Option<&'a Vec<Vector2>>,
    pub matrices: Option<&'a Vec<u32>>,
    pub matrix_palettes: Option<&'a Vec<u32>>,
    pub weights: Option<&'a Vec<Vector3>>,
}

impl<'a> PrimGroup<'a> {
    fn from_data(chunk: &'a Chunk, data: &'a OldPrimGroup, tree: &'a [Chunk]) -> Result<Self> {
        let mut group = PrimGroup {
            shader: None,
            primitive_type: data.primitive_type,
            vertices: None,
            normals: None,
            tangents: None,
            binormals: None,
            indices: None,
            uv_map: None,
            matrices: None,
            matrix_palettes: None,
            weights: None,
        };

        for child in chunk.get_children(tree) {
            match (&child.typ, &child.data) {
                (ChunkType::PositionList, ChunkData::PositionList(vertices)) => {
                    group.vertices = Some(&vertices.positions);
                }
                (ChunkType::NormalList, ChunkData::NormalList(normals)) => {
                    group.normals = Some(&normals.normals);
                }
                (ChunkType::TangentList, ChunkData::TangentList(tangents)) => {
                    group.tangents = Some(&tangents.tangents);
                }
                (ChunkType::BinormalList, ChunkData::BinormalList(binormals)) => {
                    group.binormals = Some(&binormals.binormals);
                }
                (ChunkType::IndexList, ChunkData::IndexList(indices)) => {
                    group.indices = Some(&indices.indices);
                }
                (ChunkType::UVList, ChunkData::UVList(uv_map)) => {
                    group.uv_map = Some(&uv_map.UVs);
                }
                (ChunkType::MatrixList, ChunkData::MatrixList(matrices)) => {
                    group.matrices = Some(&matrices.matrices);
                }
                (ChunkType::MatrixPalette, ChunkData::MatrixPalette(matrix_palettes)) => {
                    group.matrix_palettes = Some(&matrix_palettes.matrices);
                }
                (ChunkType::WeightList, ChunkData::WeightList(weights)) => {
                    group.weights = Some(&weights.weights);
                }
                _ => {} // Ignore other children for now
            }
        }

        if let Some(shader) = tree.iter().find(|c| match (&c.typ, &c.data) {
            (ChunkType::Shader, ChunkData::Shader(_, _, shader)) => {
                shader.pddi_shader_name == data.shader_name
            }
            _ => false,
        }) {
            group.shader = Some(Shader::from_chunk(shader, tree)?);
        }

        Ok(group)
    }
}

impl<'a> FromChunk<'a> for PrimGroup<'a> {
    type Output = PrimGroup<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::OldPrimGroup, ChunkData::OldPrimGroup(_version, data)) => {
                PrimGroup::from_data(chunk, data, tree)
            }
            (typ, data) => Err(eyre!(
                "PrimGroup expected ChunkType::OldPrimGroup with ChunkData::OldPrimGroup but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Mesh<'a> {
    pub name: &'a str,
    pub prim_groups: Vec<PrimGroup<'a>>,
}

impl<'a> Mesh<'a> {
    fn from_data(
        chunk: &'a Chunk,
        name: &'a str,
        num_prim_groups: u32,
        tree: &'a [Chunk],
    ) -> Result<Self> {
        let mut prim_groups = Vec::with_capacity(num_prim_groups as usize);

        for child in chunk.get_children_of_type(tree, ChunkType::OldPrimGroup) {
            prim_groups.push(PrimGroup::from_chunk(child, tree)?);
        }

        Ok(Mesh { name, prim_groups })
    }
}

impl<'a> FromChunk<'a> for Mesh<'a> {
    type Output = Mesh<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::Mesh, ChunkData::Mesh(name, _version, mesh)) => {
                Mesh::from_data(chunk, &name.0, mesh.num_prim_groups, tree)
            }
            (typ, data) => Err(eyre!(
                "Mesh expected ChunkType::Mesh with ChunkData::Mesh but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SkeletonJoint<'a> {
    pub name: &'a str,
    pub parent: usize,
    pub dof: i32,
    pub free_axis: i32,
    pub primary_axis: i32,
    pub secondary_axis: i32,
    pub twist_axis: i32,
    pub rest_pose: Matrix,
}

impl<'a> SkeletonJoint<'a> {
    fn from_data(
        _chunk: &'a Chunk,
        name: &'a str,
        data: &'a kinds::skeleton::SkeletonJoint,
        _tree: &'a [Chunk],
    ) -> Result<Self> {
        Ok(SkeletonJoint {
            name,
            parent: data.parent as usize,
            dof: data.dof,
            free_axis: data.free_axis,
            primary_axis: data.primary_axis,
            secondary_axis: data.secondary_axis,
            twist_axis: data.twist_axis,
            rest_pose: data.rest_pose,
        })
    }
}

impl<'a> FromChunk<'a> for SkeletonJoint<'a> {
    type Output = SkeletonJoint<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::P3DSkeletonJoint, ChunkData::SkeletonJoint(name, data)) => {
                SkeletonJoint::from_data(chunk, &name.0, data, tree)
            }
            (typ, data) => Err(eyre!(
                "SkeletonJoint expected ChunkType::P3DSkeletonJoint with ChunkData::SkeletonJoint but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Skeleton<'a> {
    pub name: &'a str,
    pub joints: Vec<SkeletonJoint<'a>>,
}

impl<'a> FromChunk<'a> for Skeleton<'a> {
    type Output = Skeleton<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::P3DSkeleton, ChunkData::Skeleton(name, _version, data)) => {
                let mut joints = Vec::with_capacity(data.num_joints as usize);

                for child in chunk.get_children_of_type(tree, ChunkType::P3DSkeletonJoint) {
                    joints.push(SkeletonJoint::from_chunk(child, tree)?);
                }

                Ok(Skeleton {
                    name: &name.0,
                    joints
                })
            }
            (typ, data) => Err(eyre!(
                "Skeleton expected ChunkType::P3DSkeleton with ChunkData::Skeleton but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Skin<'a> {
    pub name: &'a str,
    pub skeleton: Option<Skeleton<'a>>,
    pub prim_groups: Vec<PrimGroup<'a>>,
}

impl<'a> FromChunk<'a> for Skin<'a> {
    type Output = Skin<'a>;

    fn from_chunk(chunk: &'a Chunk, tree: &'a [Chunk]) -> Result<Self::Output> {
        match (&chunk.typ, &chunk.data) {
            (ChunkType::Skin, ChunkData::Skin(name, _version, data)) => {
                let mut skin = Skin {
                    name: &name.0,
                    skeleton: None,
                    prim_groups: Vec::with_capacity(data.num_prim_groups as usize)
                };

                if let Some(skeleton) = tree.iter().find(|c| {
                    match (&c.typ, &c.data) {
                        (ChunkType::P3DSkeleton, ChunkData::Skeleton(name, _, _)) => {
                            name.0 == data.skeleton_name
                        }
                        _ => false
                    }
                }) {
                    skin.skeleton = Some(Skeleton::from_chunk(skeleton, tree)?);
                }

                for child in chunk.get_children_of_type(tree, ChunkType::OldPrimGroup) {
                    skin.prim_groups.push(PrimGroup::from_chunk(child, tree)?);
                }

                Ok(skin)
            }
            (typ, data) => Err(eyre!(
                "Skeleton expected ChunkType::P3DSkeleton with ChunkData::Skeleton but got a {:?} chunk with {:?}",
                typ,
                data
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum HighLevelType<'a> {
    Mesh(Mesh<'a>),
    Skin(Skin<'a>),
}

// Keep these lifetimes since it matches everything else in the file and makes it clear.
#[allow(clippy::needless_lifetimes)]
pub fn parse_high_level_types<'a>(tree: &'a [Chunk]) -> Result<Vec<HighLevelType<'a>>> {
    let mut types = Vec::new();

    for chunk in tree.iter() {
        match &chunk.typ {
            ChunkType::Mesh => types.push(HighLevelType::Mesh(Mesh::from_chunk(chunk, tree)?)),
            ChunkType::Skin => types.push(HighLevelType::Skin(Skin::from_chunk(chunk, tree)?)),
            _ => {}
        }
    }

    Ok(types)
}

#[cfg(test)]
mod test {
    use p3dparse::chunk::{data::kinds::mesh::VertexType, Span};

    use super::*;

    #[test]
    fn test_mesh() {
        let chunks = [
            Chunk {
                typ: ChunkType::DataFile,
                data: ChunkData::None,
                span: Span {
                    absolute_index: 0,
                    relative_index: 0,
                },
                parent: None,
                children: vec![1],
            },
            Chunk {
                typ: ChunkType::Mesh,
                data: ChunkData::Mesh(
                    kinds::name::Name("testMesh1".into()),
                    kinds::version::Version(0),
                    kinds::mesh::Mesh { num_prim_groups: 1 },
                ),
                span: Span {
                    absolute_index: 1,
                    relative_index: 0,
                },
                parent: Some(0),
                children: vec![2],
            },
            Chunk {
                typ: ChunkType::OldPrimGroup,
                data: ChunkData::OldPrimGroup(
                    kinds::version::Version(0),
                    kinds::mesh::OldPrimGroup {
                        shader_name: "shader1".into(),
                        primitive_type: PrimitiveType::TriangleList,
                        vertex_types: VertexType::from(0),
                        num_vertices: 1,
                        num_indices: 1,
                        num_matrices: 0,
                    },
                ),
                span: Span {
                    absolute_index: 2,
                    relative_index: 0,
                },
                parent: Some(1),
                children: vec![3, 4],
            },
            Chunk {
                typ: ChunkType::PositionList,
                data: ChunkData::PositionList(kinds::mesh::PositionList {
                    positions: vec![(0., 0., 0.)],
                }),
                span: Span {
                    absolute_index: 3,
                    relative_index: 0,
                },
                parent: Some(2),
                children: vec![],
            },
            Chunk {
                typ: ChunkType::IndexList,
                data: ChunkData::IndexList(kinds::mesh::IndexList { indices: vec![1] }),
                span: Span {
                    absolute_index: 3,
                    relative_index: 1,
                },
                parent: Some(2),
                children: vec![],
            },
            Chunk {
                typ: ChunkType::Shader,
                data: ChunkData::Shader(
                    kinds::name::Name("shader1".into()),
                    kinds::version::Version(0),
                    kinds::shader::Shader {
                        pddi_shader_name: "shader1".into(),
                        has_translucency: 0,
                        vertex_needs: VertexType::from(0),
                        vertex_mask: VertexType::from(0),
                        num_params: 0,
                    },
                ),
                span: Span {
                    absolute_index: 4,
                    relative_index: 1,
                },
                parent: Some(0),
                children: vec![],
            },
        ];
        let types = parse_high_level_types(&chunks).expect("Failed to parse High Level Types");

        let first = types.first().expect("High Level Type didn't parse Mesh");

        assert_eq!(
            *first,
            HighLevelType::Mesh(Mesh {
                name: "testMesh1",
                prim_groups: vec![PrimGroup {
                    shader: Some(Shader {
                        name: "shader1",
                        params: vec![]
                    }),
                    primitive_type: PrimitiveType::TriangleList,
                    vertices: Some(&vec![(0., 0., 0.)]),
                    normals: None,
                    tangents: None,
                    binormals: None,
                    indices: Some(&vec![1]),
                    uv_map: None,
                    matrices: None,
                    matrix_palettes: None,
                    weights: None
                }]
            })
        );
    }

    #[test]
    fn test_skin() {
        let chunks = [
            Chunk {
                typ: ChunkType::DataFile,
                data: ChunkData::None,
                span: Span {
                    absolute_index: 0,
                    relative_index: 0,
                },
                parent: None,
                children: vec![1, 5, 6],
            },
            Chunk {
                typ: ChunkType::Skin,
                data: ChunkData::Skin(
                    kinds::name::Name("testMesh1".into()),
                    kinds::version::Version(0),
                    kinds::mesh::Skin {
                        num_prim_groups: 1,
                        skeleton_name: "skeleton1".into(),
                    },
                ),
                span: Span {
                    absolute_index: 1,
                    relative_index: 0,
                },
                parent: Some(0),
                children: vec![2],
            },
            Chunk {
                typ: ChunkType::OldPrimGroup,
                data: ChunkData::OldPrimGroup(
                    kinds::version::Version(0),
                    kinds::mesh::OldPrimGroup {
                        shader_name: "shader1".into(),
                        primitive_type: PrimitiveType::TriangleList,
                        vertex_types: VertexType::from(0),
                        num_vertices: 1,
                        num_indices: 1,
                        num_matrices: 0,
                    },
                ),
                span: Span {
                    absolute_index: 2,
                    relative_index: 0,
                },
                parent: Some(1),
                children: vec![3, 4],
            },
            Chunk {
                typ: ChunkType::PositionList,
                data: ChunkData::PositionList(kinds::mesh::PositionList {
                    positions: vec![(0., 0., 0.)],
                }),
                span: Span {
                    absolute_index: 3,
                    relative_index: 0,
                },
                parent: Some(2),
                children: vec![],
            },
            Chunk {
                typ: ChunkType::IndexList,
                data: ChunkData::IndexList(kinds::mesh::IndexList { indices: vec![1] }),
                span: Span {
                    absolute_index: 4,
                    relative_index: 1,
                },
                parent: Some(2),
                children: vec![],
            },
            Chunk {
                typ: ChunkType::Shader,
                data: ChunkData::Shader(
                    kinds::name::Name("shader1".into()),
                    kinds::version::Version(0),
                    kinds::shader::Shader {
                        pddi_shader_name: "shader1".into(),
                        has_translucency: 0,
                        vertex_needs: VertexType::from(0),
                        vertex_mask: VertexType::from(0),
                        num_params: 0,
                    },
                ),
                span: Span {
                    absolute_index: 5,
                    relative_index: 1,
                },
                parent: Some(0),
                children: vec![],
            },
            Chunk {
                typ: ChunkType::P3DSkeleton,
                data: ChunkData::Skeleton(
                    kinds::name::Name("skeleton1".into()),
                    kinds::version::Version(0),
                    kinds::skeleton::Skeleton { num_joints: 1 },
                ),
                span: Span {
                    absolute_index: 6,
                    relative_index: 2,
                },
                parent: Some(0),
                children: vec![7],
            },
            Chunk {
                typ: ChunkType::P3DSkeletonJoint,
                data: ChunkData::SkeletonJoint(
                    kinds::name::Name("joint1".into()),
                    kinds::skeleton::SkeletonJoint {
                        parent: 0,
                        dof: 0,
                        free_axis: 0,
                        primary_axis: 0,
                        secondary_axis: 0,
                        twist_axis: 0,
                        rest_pose: Matrix::identity(),
                    },
                ),
                span: Span {
                    absolute_index: 7,
                    relative_index: 0,
                },
                parent: Some(6),
                children: vec![],
            },
        ];

        let types = parse_high_level_types(&chunks).expect("Failed to parse High Level Types");

        let first = types.first().expect("High Level Type didn't parse Mesh");

        assert_eq!(
            *first,
            HighLevelType::Skin(Skin {
                name: "testMesh1",
                skeleton: Some(Skeleton {
                    name: "skeleton1",
                    joints: vec![SkeletonJoint {
                        name: "joint1",
                        parent: 0,
                        dof: 0,
                        free_axis: 0,
                        primary_axis: 0,
                        secondary_axis: 0,
                        twist_axis: 0,
                        rest_pose: Matrix::identity()
                    }]
                }),
                prim_groups: vec![PrimGroup {
                    shader: Some(Shader {
                        name: "shader1",
                        params: vec![]
                    }),
                    primitive_type: PrimitiveType::TriangleList,
                    vertices: Some(&vec![(0., 0., 0.)]),
                    normals: None,
                    tangents: None,
                    binormals: None,
                    indices: Some(&vec![1]),
                    uv_map: None,
                    matrices: None,
                    matrix_palettes: None,
                    weights: None
                }]
            })
        );
    }
}
