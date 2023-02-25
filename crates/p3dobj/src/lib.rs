use itertools::Itertools;
use p3dparse::chunk::{
    data::{
        kinds::{
            mesh::PrimitiveType,
            shared::{Vector2, Vector3},
        },
        ChunkData,
    },
    type_identifiers::ChunkType,
    Chunk,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct FullMesh {
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<Vector3>,
    pub binormals: Vec<Vector3>,
    pub indices: Vec<u32>,
    pub uv_map: Vec<Vector2>,
    pub prim_type: PrimitiveType,
    pub name: String,
}

impl FullMesh {
    pub fn parse(mesh: &Chunk, file: &[Chunk]) -> Self {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut uv_map = Vec::new();
        let mut tangents = Vec::new();
        let mut binormals = Vec::new();
        let mut prim_type = PrimitiveType::TriangleList;
        let mut name = "".into();

        assert_eq!(mesh.typ, ChunkType::Mesh);

        if let ChunkData::Mesh(mesh_name, _, _) = &mesh.data {
            name = mesh_name.0.clone();
        }

        for prim_group in mesh.get_children_of_type(file, ChunkType::OldPrimGroup) {
            // TODO: better support for multiple prim groups
            if let ChunkData::OldPrimGroup(_, data) = &prim_group.data {
                prim_type = data.primitive_type;
            }

            for child in prim_group.get_children(file) {
                match child.typ {
                    ChunkType::PositionList => {
                        if let ChunkData::PositionList(position_list) = &child.data {
                            for vertex in &position_list.positions {
                                vertices.push(*vertex);
                            }
                        }
                    }
                    ChunkType::NormalList => {
                        if let ChunkData::NormalList(normal_list) = &child.data {
                            for normal in &normal_list.normals {
                                normals.push(*normal);
                            }
                        }
                    }
                    ChunkType::IndexList => {
                        if let ChunkData::IndexList(index_list) = &child.data {
                            for index in &index_list.indices {
                                indices.push(*index);
                            }
                        }
                    }
                    ChunkType::UVList => {
                        if let ChunkData::UVList(uv_list) = &child.data {
                            for uv in &uv_list.UVs {
                                uv_map.push(*uv);
                            }
                        }
                    }
                    ChunkType::TangentList => {
                        if let ChunkData::TangentList(tangent_list) = &child.data {
                            for tangent in &tangent_list.tangents {
                                tangents.push(*tangent);
                            }
                        }
                    }
                    ChunkType::BinormalList => {
                        if let ChunkData::BinormalList(binormal_list) = &child.data {
                            for binormal in &binormal_list.binormals {
                                binormals.push(*binormal);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        FullMesh {
            vertices,
            normals,
            tangents,
            binormals,
            indices,
            uv_map,
            prim_type,
            name,
        }
    }

    fn write_obj(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut stream = BufWriter::new(File::create(path)?);
        assert!(!self.vertices.is_empty());

        writeln!(stream, "# Vertices")?;
        for vertex in &self.vertices {
            writeln!(stream, "v {} {} {}", vertex.0, vertex.1, vertex.2)?;
        }

        writeln!(stream, "# Normals")?;
        for normal in &self.normals {
            writeln!(stream, "vn {} {} {}", normal.0, normal.1, normal.2)?;
        }

        writeln!(stream, "# UVs")?;
        for uv in &self.uv_map {
            writeln!(stream, "vt {} {}", uv.0, uv.1)?;
        }

        writeln!(stream, "# Faces")?;
        match &self.prim_type {
            PrimitiveType::TriangleList => {
                for (index1, index2, index3) in self.indices.iter().tuples() {
                    writeln!(
                        stream,
                        "f {}/{}/{} {}/{}/{} {}/{}/{}",
                        index1 + 1,
                        index1 + 1,
                        index1 + 1,
                        index2 + 1,
                        index2 + 1,
                        index2 + 1,
                        index3 + 1,
                        index3 + 1,
                        index3 + 1
                    )?;
                }
            }
            PrimitiveType::TriangleStrip => unimplemented!(),
            PrimitiveType::LineList => unimplemented!(),
            PrimitiveType::LineStrip => unimplemented!(),
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mesh() {
        let asset =
            std::fs::read("../p3dparse/src/tests/real_assets/simplified_letter_A.p3d").unwrap();
        let file = p3dparse::parse_file(p3dparse::Bytes::from(asset)).unwrap();

        let mesh = file.iter().find(|c| c.typ == ChunkType::Mesh).unwrap();

        let mesh = FullMesh::parse(mesh, &file);

        mesh.write_obj(Path::new("./simplified_letter_A.obj"))
            .unwrap();
    }
}
