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
use std::{fs::File, io::BufWriter, io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct PrimGroup {
    pub shader_name: String,
    pub prim_type: PrimitiveType,
    pub offset_vertex: usize,
    pub offset_normal: usize,
    pub offset_uv: usize,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<Vector3>,
    pub binormals: Vec<Vector3>,
    pub indices: Vec<u32>,
    pub uv_map: Vec<Vector2>,
}

impl PrimGroup {
    pub fn write_vertices<W: Write>(&self, mut writer: W) -> Result<usize, std::io::Error> {
        let count = self.vertices.len();
        for x in &self.vertices {
            writeln!(writer, "v {} {} {}", x.0, x.1, x.2)?;
        }
        Ok(count)
    }

    pub fn write_normals<W: Write>(&self, mut writer: W) -> Result<usize, std::io::Error> {
        let count = self.normals.len();
        for x in &self.normals {
            writeln!(writer, "vn {} {} {}", x.0, x.1, x.2)?;
        }
        Ok(count)
    }

    pub fn write_uv_map<W: Write>(&self, mut writer: W) -> Result<usize, std::io::Error> {
        let count = self.uv_map.len();
        for x in &self.uv_map {
            writeln!(writer, "vt {} {}", x.0, x.1)?;
        }
        Ok(count)
    }

    pub fn write_faces<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        writeln!(writer, "usemtl {}", self.shader_name)?;
        match self.prim_type {
            PrimitiveType::TriangleList => {
                for (one, two, three) in self.indices.iter().tuples() {
                    // Obj format starts numbering at 1, so always offset by 1
                    let (one, two, three) =
                        (*one as usize + 1, *two as usize + 1, *three as usize + 1);
                    // Write the triangle backwards for correct face normal
                    writeln!(
                        writer,
                        "f {}/{}/{} {}/{}/{} {}/{}/{}",
                        three + self.offset_vertex,
                        three + self.offset_uv,
                        three + self.offset_normal,
                        two + self.offset_vertex,
                        two + self.offset_uv,
                        two + self.offset_normal,
                        one + self.offset_vertex,
                        one + self.offset_uv,
                        one + self.offset_normal,
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

#[derive(Debug, Clone)]
pub struct FullMesh {
    pub name: String,
    pub prim_groups: Vec<PrimGroup>,
}

impl FullMesh {
    pub fn parse(mesh: &Chunk, file: &[Chunk]) -> Result<Self, std::io::Error> {
        let mut full_mesh = FullMesh {
            name: "".to_owned(),
            prim_groups: Vec::new(),
        };

        assert_eq!(mesh.typ, ChunkType::Mesh);

        if let ChunkData::Mesh(mesh_name, _, data) = &mesh.data {
            full_mesh.name = mesh_name.0.clone();
            full_mesh.prim_groups.reserve(data.num_prim_groups as usize);
        }

        let mut offset_vertex = 0;
        let mut offset_normal = 0;
        let mut offset_uv = 0;
        for prim_group in mesh.get_children_of_type(file, ChunkType::OldPrimGroup) {
            let (prim_type, shader_name) =
                if let ChunkData::OldPrimGroup(_, data) = &prim_group.data {
                    (data.primitive_type, data.shader_name.clone())
                } else {
                    eprintln!("Skipping prim group due to invalid data");
                    continue;
                };

            let mut pgroup = PrimGroup {
                shader_name,
                prim_type,
                offset_vertex,
                offset_normal,
                offset_uv,
                vertices: Vec::new(),
                normals: Vec::new(),
                tangents: Vec::new(),
                binormals: Vec::new(),
                indices: Vec::new(),
                uv_map: Vec::new(),
            };

            for child in prim_group.get_children(file) {
                match child.typ {
                    ChunkType::PositionList => {
                        if let ChunkData::PositionList(position_list) = &child.data {
                            for x in &position_list.positions {
                                pgroup.vertices.push(*x);
                                offset_vertex += 1;
                            }
                        }
                    }
                    ChunkType::NormalList => {
                        if let ChunkData::NormalList(normal_list) = &child.data {
                            for x in &normal_list.normals {
                                pgroup.normals.push(*x);
                                offset_normal += 1;
                            }
                        }
                    }
                    ChunkType::IndexList => {
                        if let ChunkData::IndexList(index_list) = &child.data {
                            for x in &index_list.indices {
                                pgroup.indices.push(*x);
                            }
                        }
                    }
                    ChunkType::UVList => {
                        if let ChunkData::UVList(uv_list) = &child.data {
                            for x in &uv_list.UVs {
                                pgroup.uv_map.push(*x);
                                offset_uv += 1;
                            }
                        }
                    }
                    ChunkType::TangentList => {
                        if let ChunkData::TangentList(tangent_list) = &child.data {
                            for x in &tangent_list.tangents {
                                pgroup.tangents.push(*x);
                            }
                        }
                    }
                    ChunkType::BinormalList => {
                        if let ChunkData::BinormalList(binormal_list) = &child.data {
                            for x in &binormal_list.binormals {
                                pgroup.binormals.push(*x);
                            }
                        }
                    }
                    _ => {}
                }
            }

            full_mesh.prim_groups.push(pgroup);
        }

        Ok(full_mesh)
    }

    pub fn write_obj(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut stream = BufWriter::new(File::create(path)?);

        writeln!(stream, "s 1")?;

        for group in &self.prim_groups {
            group.write_vertices(&mut stream)?;
        }

        for group in &self.prim_groups {
            group.write_normals(&mut stream)?;
        }

        for group in &self.prim_groups {
            group.write_uv_map(&mut stream)?;
        }

        writeln!(stream, "g {}", self.name)?;
        for group in &self.prim_groups {
            group.write_faces(&mut stream)?;
        }

        Ok(())
    }
}
