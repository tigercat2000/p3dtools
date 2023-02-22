use p3dparse::chunk::{data::kinds::shared::Vector3, type_identifiers::ChunkType, Chunk};

pub struct FullMesh {
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<Vector3>,
    pub binormals: Vec<Vector3>,
    pub indices: Vec<u32>,
}

impl FullMesh {
    pub fn parse(mesh: &Chunk) -> Self {
        assert_eq!(mesh.typ, ChunkType::Mesh);
        for prim_group in mesh.get_children_of_type(ChunkType::PrimGroup) {
            println!("Prim Group: {:#?}", prim_group);
        }
        todo!()
    }
}
