use itertools::Itertools;
use p3dhl::{HighLevelType, Mesh};
use p3dparse::chunk::{
    data::kinds::{image::ImageFormat, mesh::PrimitiveType, shader_param::ShaderParamValue},
    Chunk,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

type Result<T> = std::result::Result<T, eyre::Error>;

trait WriteObj {
    fn write_vertices<W: Write>(&self, mut _writer: W) -> Result<()> {
        Ok(())
    }

    fn write_normals<W: Write>(&self, mut _writer: W) -> Result<()> {
        Ok(())
    }

    fn write_uv_map<W: Write>(&self, mut _writer: W) -> Result<()> {
        Ok(())
    }

    fn write_mtl<W: Write>(
        &self,
        mut _writer: W,
        _textures: &[(&str, ImageFormat, &[u8])],
    ) -> Result<()> {
        Ok(())
    }

    fn write_faces<W: Write>(
        &self,
        mut _writer: W,
        _offset_vertex: usize,
        _offset_uv: usize,
        _offset_normal: usize,
    ) -> Result<()> {
        Ok(())
    }

    fn write_obj(&self, _dest: &Path) -> Result<()> {
        Ok(())
    }
}

impl<'a> WriteObj for p3dhl::PrimGroup<'a> {
    fn write_vertices<W: Write>(&self, mut writer: W) -> Result<()> {
        if let Some(vertices) = self.vertices {
            for x in vertices {
                writeln!(writer, "v {} {} {}", x.0, x.1, x.2)?;
            }
        }
        Ok(())
    }

    fn write_normals<W: Write>(&self, mut writer: W) -> Result<()> {
        if let Some(normals) = self.normals {
            for x in normals {
                writeln!(writer, "vn {} {} {}", x.0, x.1, x.2)?;
            }
        }
        Ok(())
    }

    fn write_uv_map<W: Write>(&self, mut writer: W) -> Result<()> {
        if let Some(uv_map) = self.uv_map {
            for x in uv_map {
                writeln!(writer, "vt {} {}", x.0, x.1)?;
            }
        }
        Ok(())
    }

    fn write_faces<W: Write>(
        &self,
        mut writer: W,
        offset_vertex: usize,
        offset_uv: usize,
        offset_normal: usize,
    ) -> Result<()> {
        writeln!(writer, "usemtl {}", self.shader)?;

        match self.primitive_type {
            PrimitiveType::TriangleList => {
                if let Some(indices) = self.indices {
                    for (one, two, three) in indices.iter().tuples() {
                        // Obj format starts numbering at 1, so always offset by 1
                        let (one, two, three) =
                            (*one as usize + 1, *two as usize + 1, *three as usize + 1);
                        // Write the triangle backwards for correct face normal
                        writeln!(
                            writer,
                            "f {}/{}/{} {}/{}/{} {}/{}/{}",
                            three + offset_vertex,
                            three + offset_uv,
                            three + offset_normal,
                            two + offset_vertex,
                            two + offset_uv,
                            two + offset_normal,
                            one + offset_vertex,
                            one + offset_uv,
                            one + offset_normal,
                        )?;
                    }
                }
            }
            PrimitiveType::TriangleStrip => unimplemented!(),
            PrimitiveType::LineList => unimplemented!(),
            PrimitiveType::LineStrip => unimplemented!(),
        }

        Ok(())
    }
}

impl<'a> WriteObj for p3dhl::Shader<'a> {
    fn write_mtl<W: Write>(
        &self,
        mut writer: W,
        textures: &[(&str, ImageFormat, &[u8])],
    ) -> Result<()> {
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
                let extension = if let Some((_, format, _)) = textures.iter().find(|x| x.0 == *tex)
                {
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

struct PrimGroup<'a> {
    prim_group: &'a p3dhl::PrimGroup<'a>,
    offset_vertex: usize,
    offset_uv: usize,
    offset_normal: usize,
}

impl<'a> WriteObj for Mesh<'a> {
    fn write_obj(&self, dest: &Path) -> Result<()> {
        let mut stream = BufWriter::new(File::create(dest)?);

        writeln!(stream, "s 1")?;

        let mut offset_vertex = 0;
        let mut offset_uv = 0;
        let mut offset_normal = 0;

        let groups: Vec<_> = self
            .prim_groups
            .iter()
            .map(|prim_group| {
                let ret = PrimGroup {
                    prim_group,
                    offset_vertex,
                    offset_uv,
                    offset_normal,
                };

                if let Some(x) = prim_group.vertices {
                    offset_vertex += x.len()
                }
                if let Some(x) = prim_group.uv_map {
                    offset_uv += x.len()
                }
                if let Some(x) = prim_group.normals {
                    offset_normal += x.len()
                }

                ret
            })
            .collect();

        for group in &groups {
            group.prim_group.write_vertices(&mut stream)?;
        }

        for group in &groups {
            group.prim_group.write_normals(&mut stream)?;
        }

        for group in &groups {
            group.prim_group.write_uv_map(&mut stream)?;
        }

        writeln!(stream, "g {}", self.name)?;

        for group in &groups {
            group.prim_group.write_faces(
                &mut stream,
                group.offset_vertex,
                group.offset_uv,
                group.offset_normal,
            )?;
        }

        let mut mtl = BufWriter::new(File::create(dest.with_extension("mtl"))?);
        for shader in &self.shaders {
            shader.write_mtl(&mut mtl, &self.textures)?;
        }

        for (name, format, image) in &self.textures {
            let mut pic_writer = BufWriter::new(File::create(dest.with_file_name(
                // Format here instead of with_extension is deliberate because we want to create files like "santa2.bmp.png" which path will try and "fix"
                format!("{}.{}", name, format.get_extension()),
            ))?);
            pic_writer.write_all(image)?;
        }

        Ok(())
    }
}

pub fn export_all_to_obj(tree: &[Chunk], dest: &Path) -> Result<()> {
    let high_level_types = p3dhl::parse_high_level_types(tree)?;

    for typ in high_level_types {
        if let HighLevelType::Mesh(mesh) = typ {
            mesh.write_obj(&dest.join(mesh.name).with_extension("obj"))?;
        }
    }

    Ok(())
}
