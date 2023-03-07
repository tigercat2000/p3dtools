use eyre::eyre;
use gltf_builder::glTFBuilder;
use gltf_json::{
    material::EmissiveFactor,
    mesh::{Mode, Primitive},
    Index, Node,
};
use itertools::Itertools;
use nalgebra::Transform3;
use p3dhl::{Mesh, PrimGroup, Shader, Skeleton, SkeletonJoint, Skin};
use p3dparse::chunk::{data::kinds::image::ImageFormat, Chunk};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

type Result<T> = std::result::Result<T, eyre::Error>;
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// "Normalizes" weights such that x+y+z+w = 1
fn renormalize(target: &mut [f32; 4]) {
    let sum = target.iter().sum::<f32>();

    for x in target {
        *x /= sum;
    }
}

fn export_primgroup_to_gltf(
    builder: &mut glTFBuilder,
    mesh_idx: Index<gltf_json::Mesh>,
    group: &PrimGroup,
) -> Result<Index<Primitive>> {
    let mode = match group.primitive_type {
        p3dparse::chunk::data::kinds::mesh::PrimitiveType::TriangleList => Mode::Triangles,
        p3dparse::chunk::data::kinds::mesh::PrimitiveType::TriangleStrip => Mode::TriangleStrip,
        p3dparse::chunk::data::kinds::mesh::PrimitiveType::LineList => Mode::Lines,
        p3dparse::chunk::data::kinds::mesh::PrimitiveType::LineStrip => Mode::LineStrip,
    };

    let prim_group_idx = builder.insert_primitive(mesh_idx, mode);

    if let Some(vertices) = group.vertices {
        let vertices: Vec<_> = vertices
            .iter()
            .map(|f| {
                let f: [f32; 3] = f.vector.into();
                [f[0], f[1], f[2]]
            })
            .collect();
        builder.insert_positions(mesh_idx, prim_group_idx, &vertices)?;
    }

    if let Some(normals) = group.normals {
        let normals: Vec<_> = normals
            .iter()
            .map(|f| {
                let f: [f32; 3] = f.vector.into();
                [f[0], f[1], f[2]]
            })
            .collect();
        builder.insert_normals(mesh_idx, prim_group_idx, &normals)?;
    }

    if let Some(uv_map) = group.uv_map {
        let uv_map: Vec<_> = uv_map
            .iter()
            .map(|f| {
                let f: [f32; 2] = (*f).into();
                [f[0], -f[1]]
            })
            .collect();
        builder.insert_uv_map(mesh_idx, prim_group_idx, &uv_map)?;
    }

    if let Some(indices) = group.indices {
        builder.insert_indices(mesh_idx, prim_group_idx, indices)?;
    }

    match (group.matrices, group.matrix_palettes, group.weights) {
        (Some(matrices), Some(palette), Some(weights)) => {
            let (joints, weights): (Vec<[u16; 4]>, Vec<[f32; 4]>) = matrices
                .iter()
                .zip(weights.iter())
                .enumerate()
                .map(|(_idx, (affecting_joints, joint_weights))| {
                    let real_joints = affecting_joints.map(|f| palette[f as usize] as u16);

                    let x_weight = joint_weights.x;
                    let y_weight = joint_weights.y;
                    let z_weight = joint_weights.z;

                    let mut w_weight =
                        (1.0 - (joint_weights.x + joint_weights.y + joint_weights.z)).abs();

                    if w_weight < 0.000001 {
                        w_weight = 0.;
                    }

                    let mut joints = [
                        real_joints[0],
                        real_joints[1],
                        real_joints[2],
                        real_joints[3],
                    ];

                    let mut weights = [x_weight, y_weight, z_weight, w_weight];

                    // We have to do duplicate filtering...
                    // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#skinned-mesh-attributes
                    // Joints MUST NOT contain more than one non-zero weight for a given vertex.
                    let mut already_seen = HashSet::new();

                    for (idx, joint) in joints.iter().enumerate() {
                        if weights[idx] > 0. {
                            if already_seen.contains(joint) {
                                weights[idx] = 0.;
                            }
                            already_seen.insert(joint);
                        }
                    }

                    // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#skinned-mesh-attributes
                    // When the weights are stored using float component type, their linear sum SHOULD be as close as reasonably possible to 1.0 for a given vertex.
                    renormalize(&mut weights);

                    for (idx, weight) in weights.iter().enumerate() {
                        if *weight == 0.0 {
                            // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#skinned-mesh-attributes
                            // Unused joint values (i.e., joints with a weight of zero) SHOULD be set to zero.
                            joints[idx] = 0;
                        }
                    }

                    (joints, weights)
                })
                .unzip();

            builder.insert_weights(mesh_idx, prim_group_idx, &weights)?;
            builder.insert_joints(mesh_idx, prim_group_idx, &joints)?;
        }
        (Some(matrices), Some(palette), None) => {
            let (joints, weights): (Vec<[u16; 4]>, Vec<[f32; 4]>) = matrices
                .iter()
                .enumerate()
                .map(|(_idx, affecting_joints)| {
                    // In the case where there is no weight chunk, we want a weight of 1 for the first joint entry.
                    let real_joint = [palette[affecting_joints[0] as usize] as u16, 0, 0, 0];
                    (real_joint, [1., 0., 0., 0.])
                })
                .unzip();

            builder.insert_weights(mesh_idx, prim_group_idx, &weights)?;
            builder.insert_joints(mesh_idx, prim_group_idx, &joints)?;
        }
        (None, None, None) => {
            // Skinless is okay
        }
        (a, b, c) => eprintln!(
            "Unsupported configuration: Matrices {:?}, Palette {:?}, Weights {:?}",
            a.is_some(),
            b.is_some(),
            c.is_some()
        ),
    }

    Ok(prim_group_idx)
}

// fn export_texture_to_gltf(
//     builder: &mut glTFBuilder,
//     (name, format, data): &(&str, ImageFormat, &[u8]),
// ) -> Result<Index<gltf_json::Texture>> {
//     let mime_type = match format {
//         ImageFormat::PNG => Some(gltf_json::image::MimeType("image/png".into())),
//         f => return Err(eyre!("GLTF only accepts PNG currently, not {:?}", f)),
//     };

//     let image_idx = builder.insert_image(name, mime_type, data)?;
//     builder.insert_texture(name, image_idx)
// }

fn export_texture_to_gltf(
    builder: &mut glTFBuilder,
    (name, format): &(&str, Option<ImageFormat>),
) -> Result<Index<gltf_json::Texture>> {
    let mime_type = match format {
        Some(ImageFormat::PNG) => Some(gltf_json::image::MimeType("image/png".into())),
        _ => None,
    };

    let image_idx = builder.insert_image_uri(name, mime_type, &format!("{name}.png"))?;
    builder.insert_texture(name, image_idx)
}

fn export_shader_to_gltf(
    builder: &mut glTFBuilder,
    shader: &Shader,
    textures: &[(&str, ImageFormat, &[u8])],
) -> Result<Index<gltf_json::Material>> {
    let texture_idx = if let Some(tex) = shader.texture {
        if !textures.iter().any(|(name, _, _)| *name == tex) {
            eprintln!(
                "Warning: Texture {:?} was not present in file, it will have to be supplemented.",
                tex
            );
        }

        Some(export_texture_to_gltf(builder, &(tex, None))?)
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
            let mut iter = factor.iter().map(|f| (*f as f32) / 255.0).skip(1);
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
    shaders: &[Shader],
    textures: &[(&str, ImageFormat, &[u8])],
) -> Result<HashMap<String, Index<gltf_json::Material>>> {
    shaders
        .iter()
        .unique_by(|f| f.name)
        .map(|shader| {
            Ok((
                shader.name.into(),
                export_shader_to_gltf(builder, shader, textures)?,
            ))
        })
        .collect()
}

fn export_all_texture_images(folder: &Path, textures: &[(&str, ImageFormat, &[u8])]) -> Result<()> {
    for texture in textures {
        export_image_to_accompany(folder, texture)?;
    }
    Ok(())
}

fn export_image_to_accompany(
    folder: &Path,
    (name, _, data): &(&str, ImageFormat, &[u8]),
) -> Result<()> {
    // Have to not use with_extension because we want ugly files like "pants_belt.bmp.png"
    let path = folder.join(format!("{name}.png"));

    println!(
        "Exporting image {:?} to {:?} ({} bytes)",
        name,
        path,
        data.len(),
    );
    std::fs::write(path, data)?;

    Ok(())
}

fn export_mesh_to_gltf(mesh: Mesh, builder: &mut glTFBuilder) -> Result<Index<Node>> {
    let shaders = export_shaders_to_gltf(builder, &mesh.shaders, &mesh.textures)?;

    let mesh_idx = builder.insert_mesh(mesh.name);

    for group in mesh.prim_groups {
        let group_idx = export_primgroup_to_gltf(builder, mesh_idx, &group)?;
        if let Some(shader) = shaders.get(group.shader) {
            builder.set_primitive_material(mesh_idx, group_idx, *shader);
        }
    }

    let mesh_node = builder.insert_mesh_node(mesh.name, mesh_idx);

    Ok(mesh_node)
}

fn export_joint_to_gltf(
    builder: &mut glTFBuilder,
    joint: &SkeletonJoint,
) -> Index<gltf_json::Node> {
    builder.insert_node(gltf_json::Node {
        camera: Default::default(),
        children: Default::default(),
        extensions: Default::default(),
        extras: Default::default(),
        matrix: {
            if joint.rest_pose != Transform3::identity() {
                Some(transform_to_f32x16(joint.rest_pose))
            } else {
                None
            }
        },
        mesh: Default::default(),
        name: Some(joint.name.into()),
        rotation: None,
        scale: None,
        translation: None,
        skin: Default::default(),
        weights: Default::default(),
    })
}

/// SHAR transforms are stored in DirectX format (row-major), where we need OpenGL format (column-major)
/// This just means we have to transpose before we flatten.
fn transform_to_f32x16(transform: Transform3<f32>) -> [f32; 16] {
    let transform = transform.into_inner().transpose();

    let mut f = [0.; 16];
    let arr: [[f32; 4]; 4] = transform.into();

    arr.iter()
        .flatten()
        .enumerate()
        .for_each(|(k, v)| f[k] = *v);

    if let Some(last) = f.last_mut() {
        // Floating point errors make the glTF validator cry
        *last = 1.0;
    }

    f
}

// Returns root skeleton node
fn export_skeleton_to_gltf(
    builder: &mut glTFBuilder,
    skeleton: &Skeleton,
) -> Result<(Index<gltf_json::Skin>, Index<gltf_json::Node>)> {
    let mut iter = skeleton.joints.iter();
    if let Some(root) = iter.next() {
        let root_idx = export_joint_to_gltf(builder, root);

        let mut exported_joints = vec![root_idx];

        let mut bind_matrices: Vec<[f32; 16]> = Vec::new();

        bind_matrices.push(if let Some(matrix) = root.inverse_world_matrix {
            transform_to_f32x16(matrix)
        } else {
            let matrix: Transform3<f32> = Transform3::identity();
            transform_to_f32x16(matrix.try_inverse().unwrap())
        });

        for joint in iter {
            let joint_idx = export_joint_to_gltf(builder, joint);
            exported_joints.push(joint_idx);
            builder.insert_node_child(exported_joints[joint.parent], joint_idx);
            if let Some(matrix) = joint.inverse_world_matrix {
                bind_matrices.push(transform_to_f32x16(matrix));
            } else {
                let matrix: Transform3<f32> = Transform3::identity();
                bind_matrices.push(transform_to_f32x16(matrix.try_inverse().unwrap()));
            }
        }

        let skin = builder.insert_skin(gltf_json::Skin {
            extensions: None,
            extras: Default::default(),
            inverse_bind_matrices: None,
            joints: exported_joints,
            name: Some("Skeleton".into()),
            skeleton: Some(root_idx),
        });

        builder.insert_inverse_bind_matrices(skin, &bind_matrices)?;

        Ok((skin, root_idx))
    } else {
        Err(eyre!("Skeleton joint list was empty."))
    }
}

fn export_skin_to_gltf(skin: Skin, builder: &mut glTFBuilder) -> Result<Vec<Index<Node>>> {
    let shaders = export_shaders_to_gltf(builder, &skin.shaders, &skin.textures)?;

    let mesh_idx = builder.insert_mesh(skin.name);

    for group in skin.prim_groups {
        let group_idx = export_primgroup_to_gltf(builder, mesh_idx, &group)?;
        if let Some(shader) = shaders.get(group.shader) {
            builder.set_primitive_material(mesh_idx, group_idx, *shader);
        }
    }

    let nodes = if let Some(skeleton) = skin.skeleton {
        let (skele_idx, skele_root) = export_skeleton_to_gltf(builder, &skeleton)?;
        vec![
            builder.insert_mesh_skin_node(skin.name, mesh_idx, skele_idx),
            skele_root,
        ]
    } else {
        vec![builder.insert_mesh_node(skin.name, mesh_idx)]
    };

    Ok(nodes)
}

pub fn export_all_to_gltf(filename: &Path, tree: &[Chunk], dest: &Path) -> Result<()> {
    let mut builder = glTFBuilder::new();
    builder.set_generator(&format!("Khronos glTF p3d2gltf v{}", VERSION));

    let mut nodes = vec![];
    let hltypes = p3dhl::parse_high_level_types(tree)?;

    for hlt in hltypes {
        match hlt {
            p3dhl::HighLevelType::Mesh(mesh) => {
                nodes.push(export_mesh_to_gltf(mesh, &mut builder)?)
            }
            p3dhl::HighLevelType::Skin(skin) => {
                nodes.extend(export_skin_to_gltf(skin, &mut builder)?)
            }
            p3dhl::HighLevelType::AllTextures(textures) => {
                export_all_texture_images(dest, &textures.textures)?;
            }
            _ => {}
        };
    }

    builder.insert_scene("scene", true, &nodes);
    let string = builder.build()?;
    std::fs::write(
        dest.join(filename.file_name().unwrap())
            .with_extension("gltf"),
        string,
    )?;

    Ok(())
}
