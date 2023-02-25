use itertools::Itertools;
use p3dparse::chunk::{
    data::{
        kinds::{
            image::ImageFormat,
            mesh::PrimitiveType,
            shader_param::{ShaderParam, ShaderParamValue},
            shared::{Vector2, Vector3},
        },
        ChunkData,
    },
    type_identifiers::ChunkType,
    Chunk,
};
use std::{collections::HashMap, fs::File, io::BufWriter, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub struct Shader {
    pub name: String,
    pub params: Vec<ShaderParam>,
}

impl Shader {
    pub fn parse(mesh: &Chunk, file: &[Chunk]) -> Result<Self, std::io::Error> {
        assert_eq!(mesh.typ, ChunkType::Shader);

        let name = if let ChunkData::Shader(name, _, _) = &mesh.data {
            name.0.clone()
        } else {
            panic!("Invalid ChunkData for Shader");
        };

        let mut params = Vec::new();

        for param in mesh.get_children(file) {
            if let ChunkData::ShaderParam(param) = &param.data {
                params.push(param.clone());
            }
        }

        Ok(Shader { name, params })
    }

    pub fn write_mtl<W: Write>(
        &self,
        mut writer: W,
        textures: &HashMap<String, (ImageFormat, Vec<u8>)>,
    ) -> Result<(), std::io::Error> {
        writeln!(writer, "newmtl {}", self.name)?;
        if let Some(inner) = self.params.iter().find(|f| f.param == "AMBI") {
            if let ShaderParamValue::Colour(color) = &inner.value {
                writeln!(
                    writer,
                    "Ka {} {} {}",
                    (color.0 as f32) / 255.0,
                    (color.1 as f32) / 255.0,
                    (color.2 as f32) / 255.0
                )?;
            }
        }
        if let Some(inner) = self.params.iter().find(|f| f.param == "DIFF") {
            if let ShaderParamValue::Colour(color) = &inner.value {
                writeln!(
                    writer,
                    "Kd {} {} {}",
                    (color.0 as f32) / 255.0,
                    (color.1 as f32) / 255.0,
                    (color.2 as f32) / 255.0
                )?;
            }
        } else {
            // We always need a diffuse
            writeln!(writer, "Kd 1 1 1")?;
        }
        if let Some(inner) = self.params.iter().find(|f| f.param == "SPEC") {
            if let ShaderParamValue::Colour(color) = &inner.value {
                writeln!(
                    writer,
                    "Ks {} {} {}",
                    (color.0 as f32) / 255.0,
                    (color.1 as f32) / 255.0,
                    (color.2 as f32) / 255.0
                )?;
            }
        }
        if let Some(inner) = self.params.iter().find(|f| f.param == "TEX") {
            if let ShaderParamValue::Texture(tex) = &inner.value {
                let extension = if let Some((format, _)) = textures.get(tex) {
                    format.get_extension()
                } else {
                    eprintln!("Unable to find texture {:?}", tex);
                    "png"
                };
                // TODO: Check the actual type of the asset it's referring to
                writeln!(writer, "map_Kd {}.{}", tex, extension)?;
            }
        }
        Ok(())
    }
}

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
    pub shaders: HashMap<String, Shader>,
    pub textures: HashMap<String, (ImageFormat, Vec<u8>)>,
}

impl FullMesh {
    pub fn parse(mesh: &Chunk, file: &[Chunk]) -> Result<Self, std::io::Error> {
        let mut full_mesh = FullMesh {
            name: "".to_owned(),
            prim_groups: Vec::new(),
            shaders: HashMap::new(),
            textures: HashMap::new(),
        };

        assert_eq!(mesh.typ, ChunkType::Mesh);

        if let ChunkData::Mesh(mesh_name, _, data) = &mesh.data {
            full_mesh.name = mesh_name.0.clone();
            full_mesh.prim_groups.reserve(data.num_prim_groups as usize);
        }

        let shader_result: Result<Vec<_>, _> = file
            .iter()
            .filter(|f| f.typ == ChunkType::Shader)
            .map(|c| Shader::parse(c, file))
            .collect();

        let shaders = shader_result?;

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

            full_mesh
                .shaders
                .entry(shader_name.clone())
                .or_insert_with(|| {
                    if let Some(shader) = shaders.iter().find_map(|s| {
                        if s.name == shader_name {
                            Some(s.clone())
                        } else {
                            None
                        }
                    }) {
                        shader
                    } else {
                        panic!(
                            "Non-existent shader {} referenced by prim group",
                            shader_name
                        );
                    }
                });

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

        full_mesh.textures = file
            .iter()
            .filter(|f| f.typ == ChunkType::Texture)
            // Filter for valid data & in active shader list
            .filter_map(|f| {
                if let ChunkData::Texture(name, _, _) = &f.data {
                    // Make sure
                    if full_mesh.shaders.values().any(|f| {
                        f.params.iter().any(|p| {
                            p.param == "TEX" && p.value == ShaderParamValue::Texture(name.0.clone())
                        })
                    }) {
                        Some((name.0.clone(), f))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter_map(|(name, f)| {
                if let Ok(image_chunk) = f.get_child(file, 0) {
                    if let ChunkData::Image(_, _, data) = &image_chunk.data {
                        if let Ok(image_raw) = image_chunk.get_child(file, 0) {
                            if let ChunkData::ImageRaw(raw) = &image_raw.data {
                                return Some((name, (data.image_format, raw.data.clone())));
                            }
                        }
                    }
                }
                None
            })
            .collect();

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

        let mut mtl = BufWriter::new(File::create(path.with_extension("mtl"))?);
        for shader in self.shaders.values() {
            shader.write_mtl(&mut mtl, &self.textures)?;
        }

        for (name, (format, image)) in &self.textures {
            let mut pic_writer = BufWriter::new(File::create(path.with_file_name(
                // Format here instead of with_extension is deliberate because we want to create files like "santa2.bmp.png" which path will try and "fix"
                format!("{}.{}", name, format.get_extension()),
            ))?);
            pic_writer.write_all(image)?;
        }

        Ok(())
    }
}
