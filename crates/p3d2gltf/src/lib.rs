use base64::Engine;
use eyre::eyre;
use gltf_builder::glTFBuilder;
use gltf_json::{image::MimeType, validation::Validate, Index};
use p3dhl::{Mesh, PrimGroup};
use p3dparse::chunk::{
    data::kinds::{
        image::ImageFormat,
        mesh,
        shared::{Colour, Vector2, Vector3},
    },
    Chunk,
};
use serde_json::json;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

type Result<T> = std::result::Result<T, eyre::Error>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn export_primgroup_to_gltf(
    builder: &mut glTFBuilder,
    mesh_idx: Index<gltf_json::Mesh>,
    group: &PrimGroup,
) -> Result<()> {
    let prim_group_idx = builder.insert_primitive(mesh_idx);

    if let Some(vertices) = group.vertices {
        builder.insert_positions(mesh_idx, prim_group_idx, vertices)?;
    }

    Ok(())
}

fn export_texture_to_gltf(
    builder: &mut glTFBuilder,
    (name, format, data): &(&str, ImageFormat, &[u8]),
) -> Result<Index<gltf_json::Texture>> {
    let mime_type = match format {
        ImageFormat::PNG => Some(gltf_json::image::MimeType("image/png".into())),
        f => return Err(eyre!("GLTF only accepts PNG currently, not {:?}", f)),
    };

    let image_idx = builder.insert_image(name, mime_type, data)?;
    builder.insert_texture(name, image_idx)
}

fn export_mesh_to_gltf(mesh: Mesh, dest: &Path) -> Result<()> {
    let mut builder = glTFBuilder::new();

    let mesh_idx = builder.insert_mesh(mesh.name);

    for group in mesh.prim_groups {
        export_primgroup_to_gltf(&mut builder, mesh_idx, &group)?;
    }

    for shader in mesh.shaders {
        let texture_to_export = if let Some(tex) = shader.texture {
            let option = mesh.textures.iter().find(|(name, _, _)| *name == tex);
            if option.is_none() {
                eprintln!("Warning, failed to find requested texture {}", tex);
            }
            option
        } else {
            None
        };

        if let Some(texture) = texture_to_export {
            export_texture_to_gltf(&mut builder, texture)?;
        }
    }

    Ok(())
}

pub fn export_all_to_gltf(tree: &[Chunk], dest: &Path) -> Result<()> {
    let hltypes = p3dhl::parse_high_level_types(tree)?;

    for hlt in hltypes {
        match hlt {
            p3dhl::HighLevelType::Mesh(mesh) => export_mesh_to_gltf(mesh, dest)?,
            p3dhl::HighLevelType::Skin(skin) => {} // export_skin_to_gltf(skin, dest)?,
            _ => todo!(),
        }
    }

    Ok(())
}
