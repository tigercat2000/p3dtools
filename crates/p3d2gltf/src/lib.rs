use eyre::eyre;
use gltf_builder::glTFBuilder;
use gltf_json::{material::EmissiveFactor, mesh::Primitive, Index};
use p3dhl::{Mesh, PrimGroup, Shader};
use p3dparse::chunk::{data::kinds::image::ImageFormat, Chunk};
use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

type Result<T> = std::result::Result<T, eyre::Error>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn export_primgroup_to_gltf(
    builder: &mut glTFBuilder,
    mesh_idx: Index<gltf_json::Mesh>,
    group: &PrimGroup,
) -> Result<Index<Primitive>> {
    let prim_group_idx = builder.insert_primitive(mesh_idx);

    if let Some(vertices) = group.vertices {
        builder.insert_positions(mesh_idx, prim_group_idx, vertices)?;
    }

    if let Some(normals) = group.normals {
        builder.insert_normals(mesh_idx, prim_group_idx, normals)?;
    }

    if let Some(uv_map) = group.uv_map {
        builder.insert_uv_map(mesh_idx, prim_group_idx, uv_map)?;
    }

    if let Some(indices) = group.indices {
        builder.insert_indices(mesh_idx, prim_group_idx, indices)?;
    }

    Ok(prim_group_idx)
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

fn export_shader_to_gltf(
    builder: &mut glTFBuilder,
    mesh: &Mesh,
    shader: &Shader,
) -> Result<Index<gltf_json::Material>> {
    let texture_to_export = if let Some(tex) = shader.texture {
        let option = mesh.textures.iter().find(|(name, _, _)| *name == tex);
        if option.is_none() {
            eprintln!("Warning, failed to find requested texture {}", tex);
        }
        option
    } else {
        None
    };

    let texture_idx = if let Some(texture) = texture_to_export {
        Some(export_texture_to_gltf(builder, texture)?)
    } else {
        None
    };

    Ok(builder.insert_material(
        shader.name,
        None,
        gltf_json::material::AlphaMode::Opaque,
        if let Some(two_sided) = shader.two_sided {
            two_sided
        } else {
            false
        },
        gltf_json::material::PbrMetallicRoughness {
            base_color_factor: gltf_json::material::PbrBaseColorFactor([1., 1., 1., 1.]),
            base_color_texture: texture_idx.map(|texture| gltf_json::texture::Info {
                index: texture,
                tex_coord: 0,
                extensions: None,
                extras: Default::default(),
            }),
            metallic_factor: gltf_json::material::StrengthFactor(0.0),
            roughness_factor: gltf_json::material::StrengthFactor(1.0),
            metallic_roughness_texture: None,
            extensions: Default::default(),
            extras: Default::default(),
        },
        None,
        None,
        None,
        if let Some(factor) = shader.emissive {
            let mut iter = factor.iter().map(|f| *f as f32).take(3);
            EmissiveFactor([
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ])
        } else {
            EmissiveFactor::default()
        },
    ))
}

fn export_shaders_to_gltf(
    builder: &mut glTFBuilder,
    mesh: &Mesh,
) -> Result<HashMap<String, Index<gltf_json::Material>>> {
    mesh.shaders
        .iter()
        .map(|shader| {
            Ok((
                shader.name.into(),
                export_shader_to_gltf(builder, mesh, shader)?,
            ))
        })
        .collect()
}

fn export_mesh_to_gltf(mesh: Mesh, dest: &Path) -> Result<()> {
    let mut builder = glTFBuilder::new();

    let shaders = export_shaders_to_gltf(&mut builder, &mesh)?;

    let mesh_idx = builder.insert_mesh(mesh.name);

    for group in mesh.prim_groups {
        let group_idx = export_primgroup_to_gltf(&mut builder, mesh_idx, &group)?;
        if let Some(shader) = shaders.get(group.shader) {
            builder.set_primitive_material(mesh_idx, group_idx, *shader);
        }
    }

    let string = builder.build()?;

    std::fs::write(dest.join(mesh.name).with_extension("gltf"), string)?;

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
