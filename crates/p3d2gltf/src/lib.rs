use eyre::eyre;
use gltf_builder::glTFBuilder;
use gltf_json::{
    material::EmissiveFactor,
    mesh::{Mode, Primitive},
    scene::UnitQuaternion,
    Index,
};
use itertools::Itertools;
use p3dhl::{Mesh, PrimGroup, Shader, Skeleton, SkeletonJoint, Skin};
use p3dparse::chunk::{
    data::kinds::{image::ImageFormat, shared::QuaternionExt},
    Chunk,
};
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
        builder.insert_positions(mesh_idx, prim_group_idx, vertices)?;
    }

    if let Some(normals) = group.normals {
        builder.insert_normals(mesh_idx, prim_group_idx, normals)?;
    }

    if let Some(uv_map) = group.uv_map {
        let inverse_y: Vec<_> = uv_map.iter().map(|[x, y]| [*x, -*y]).collect();
        builder.insert_uv_map(mesh_idx, prim_group_idx, &inverse_y)?;
    }

    if let Some(indices) = group.indices {
        builder.insert_indices(mesh_idx, prim_group_idx, indices)?;
    }

    if let (Some(matrices), Some(palette), Some(weights)) =
        (group.matrices, group.matrix_palettes, group.weights)
    {
        let (joints, weights): (Vec<[u16; 4]>, Vec<[f32; 4]>) = matrices
            .iter()
            .zip(weights.iter())
            .enumerate()
            .map(|(_idx, (affecting_joints, joint_weights))| {
                let real_joints = affecting_joints.map(|f| palette[f as usize] as u16);

                let mut w_weight =
                    (1.0 - joint_weights[0] - joint_weights[1] - joint_weights[2]).abs();

                if w_weight < 0.000001 {
                    w_weight = 0.;
                }

                let x_weight = joint_weights[0];
                let y_weight = joint_weights[1];
                let z_weight = joint_weights[2];

                let mut joints = [
                    real_joints[1],
                    real_joints[2],
                    real_joints[3],
                    // Alpha is used as the last joint for weighting purposes.
                    real_joints[0],
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
    shader: &Shader,
    textures: &[(&str, ImageFormat, &[u8])],
) -> Result<Index<gltf_json::Material>> {
    let texture_to_export = if let Some(tex) = shader.texture {
        let option = textures.iter().find(|(name, _, _)| *name == tex);
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

fn export_mesh_to_gltf(mesh: Mesh, dest: &Path) -> Result<()> {
    let mut builder = glTFBuilder::new();
    builder.set_generator(&format!("Khronos glTF p3d2gltf v{}", VERSION));

    let shaders = export_shaders_to_gltf(&mut builder, &mesh.shaders, &mesh.textures)?;

    let mesh_idx = builder.insert_mesh(mesh.name);

    for group in mesh.prim_groups {
        let group_idx = export_primgroup_to_gltf(&mut builder, mesh_idx, &group)?;
        if let Some(shader) = shaders.get(group.shader) {
            builder.set_primitive_material(mesh_idx, group_idx, *shader);
        }
    }

    let mesh_node = builder.insert_mesh_node(mesh.name, mesh_idx);
    builder.insert_scene("scene", true, &[mesh_node]);

    let string = builder.build()?;

    std::fs::write(dest.join(mesh.name).with_extension("gltf"), string)?;

    Ok(())
}

fn export_joint_to_gltf(
    builder: &mut glTFBuilder,
    joint: &SkeletonJoint,
) -> Index<gltf_json::Node> {
    let (translation, rotation, scale) = joint.rest_pose.decompose();

    builder.insert_node(gltf_json::Node {
        camera: Default::default(),
        children: Default::default(),
        extensions: Default::default(),
        extras: Default::default(),
        matrix: Default::default(),
        mesh: Default::default(),
        name: Some(joint.name.into()),
        // rotation: None,
        rotation: if rotation.iter().any(|f| *f != 0.) {
            Some(UnitQuaternion(rotation.normalize()))
        } else {
            None
        },
        scale: None,
        // scale: if scale.iter().any(|f| *f != 1.) {
        //     // Some(scale)
        // } else {
        //     None
        // },
        translation: Some(translation),
        skin: Default::default(),
        weights: Default::default(),
    })
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

        for joint in iter {
            let joint_idx = export_joint_to_gltf(builder, joint);
            exported_joints.push(joint_idx);
            builder.insert_node_child(exported_joints[joint.parent], joint_idx);
        }

        Ok((
            builder.insert_skin(gltf_json::Skin {
                extensions: None,
                extras: Default::default(),
                inverse_bind_matrices: None,
                joints: exported_joints,
                name: Some("Skeleton".into()),
                skeleton: Some(root_idx),
            }),
            root_idx,
        ))
    } else {
        Err(eyre!("Skeleton joint list was empty."))
    }
}

fn export_skin_to_gltf(skin: Skin, dest: &Path) -> Result<()> {
    let mut builder = glTFBuilder::new();
    builder.set_generator(&format!("Khronos glTF p3d2gltf v{}", VERSION));

    let shaders = export_shaders_to_gltf(&mut builder, &skin.shaders, &skin.textures)?;

    let mesh_idx = builder.insert_mesh(skin.name);

    for group in skin.prim_groups {
        let group_idx = export_primgroup_to_gltf(&mut builder, mesh_idx, &group)?;
        if let Some(shader) = shaders.get(group.shader) {
            builder.set_primitive_material(mesh_idx, group_idx, *shader);
        }
    }

    let nodes = if let Some(skeleton) = skin.skeleton {
        let (skele_idx, skele_root) = export_skeleton_to_gltf(&mut builder, &skeleton)?;
        vec![
            builder.insert_mesh_skin_node(skin.name, mesh_idx, skele_idx),
            skele_root,
        ]
    } else {
        vec![builder.insert_mesh_node(skin.name, mesh_idx)]
    };

    builder.insert_scene("scene", true, &nodes);

    let string = builder.build()?;

    std::fs::write(dest.join(skin.name).with_extension("gltf"), string)?;

    Ok(())
}

pub fn export_all_to_gltf(tree: &[Chunk], dest: &Path) -> Result<()> {
    let hltypes = p3dhl::parse_high_level_types(tree)?;

    for hlt in hltypes {
        match hlt {
            p3dhl::HighLevelType::Mesh(mesh) => export_mesh_to_gltf(mesh, dest)?,
            p3dhl::HighLevelType::Skin(skin) => export_skin_to_gltf(skin, dest)?,
            _ => todo!(),
        }
    }

    Ok(())
}
