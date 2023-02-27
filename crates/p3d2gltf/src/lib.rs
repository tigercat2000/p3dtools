use base64::Engine;
use eyre::eyre;
use gltf_json::validation::Validate;
use p3dparse::chunk::{
    data::kinds::shared::{Vector2, Vector3},
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

fn wrap_valid<T>(inner: T) -> gltf_json::validation::Checked<T> {
    gltf_json::validation::Checked::Valid(inner)
}

// glTF is a proper noun
#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
struct glTFBufferBuilder {
    buf_index: u32,
    buffer: Vec<u8>,
    buffer_views: Vec<gltf_json::buffer::View>,
    accessors: Vec<gltf_json::accessor::Accessor>,
}

impl glTFBufferBuilder {
    pub fn new(buf_index: u32) -> Self {
        Self {
            buf_index,
            ..Default::default()
        }
    }

    pub fn build(
        self,
    ) -> (
        Vec<gltf_json::Accessor>,
        Vec<gltf_json::buffer::View>,
        gltf_json::Buffer,
    ) {
        let mut uri = "data:application/octet-stream;base64,".into();
        let byte_length = self.buffer.len() as u32;
        base64::prelude::BASE64_STANDARD.encode_string(self.buffer, &mut uri);

        (
            self.accessors,
            self.buffer_views,
            gltf_json::Buffer {
                byte_length,
                uri: Some(uri),
                extensions: None,
                extras: gltf_json::Extras::default(),
            },
        )
    }
}

impl glTFBufferBuilder {
    fn get_min_components_vec3(vec: &[Vector3]) -> Option<gltf_json::Value> {
        vec.iter()
            .cloned()
            .reduce(|acc, chunk| (acc.0.min(chunk.0), acc.1.min(chunk.1), acc.2.min(chunk.2)))
            .map(|f| json!(f))
    }

    fn get_max_components_vec3(vec: &[Vector3]) -> Option<gltf_json::Value> {
        vec.iter()
            .cloned()
            .reduce(|acc, chunk| (acc.0.max(chunk.0), acc.1.max(chunk.1), acc.2.max(chunk.2)))
            .map(|f| json!(f))
    }

    fn get_min_components_vec2(vec: &[Vector2]) -> Option<gltf_json::Value> {
        vec.iter()
            .cloned()
            .reduce(|acc, chunk| (acc.0.min(chunk.0), acc.1.min(chunk.1)))
            .map(|f| json!(f))
    }

    fn get_max_components_vec2(vec: &[Vector2]) -> Option<gltf_json::Value> {
        vec.iter()
            .cloned()
            .reduce(|acc, chunk| (acc.0.max(chunk.0), acc.1.max(chunk.1)))
            .map(|f| json!(f))
    }

    // fn get_min_components_scalar(vec: &[u32]) -> Option<gltf_json::Value> {
    //     vec.iter()
    //         .reduce(|acc, chunk| acc.min(chunk))
    //         .map(|f| json!(f))
    // }

    // fn get_max_components_scalar(vec: &[u32]) -> Option<gltf_json::Value> {
    //     vec.iter()
    //         .reduce(|acc, chunk| acc.max(chunk))
    //         .map(|f| json!(f))
    // }

    fn create_buffer_view(
        &mut self,
        byte_length: u32,
        byte_offset: u32,
    ) -> gltf_json::Index<gltf_json::buffer::View> {
        self.buffer_views.push(gltf_json::buffer::View {
            buffer: gltf_json::Index::new(self.buf_index),
            byte_length,
            byte_offset: if byte_offset != 0 {
                Some(byte_offset)
            } else {
                None
            },
            byte_stride: None,
            target: None,
            extensions: None,
            extras: Default::default(),
        });

        gltf_json::Index::new((self.buffer_views.len() - 1) as u32)
    }

    fn create_accessor(
        &mut self,
        buffer_view: Option<gltf_json::Index<gltf_json::buffer::View>>,
        count: u32,
        min: Option<gltf_json::Value>,
        max: Option<gltf_json::Value>,
        component_type: gltf_json::accessor::ComponentType,
        type_: gltf_json::accessor::Type,
    ) -> gltf_json::Index<gltf_json::Accessor> {
        self.accessors.push(gltf_json::Accessor {
            buffer_view,
            byte_offset: 0,
            count,
            component_type: wrap_valid(gltf_json::accessor::GenericComponentType(component_type)),
            extensions: None,
            extras: Default::default(),
            type_: wrap_valid(type_),
            min,
            max,
            normalized: false,
            sparse: None,
        });

        gltf_json::Index::new((self.accessors.len() - 1) as u32)
    }

    fn create_accessor_vec3(
        &mut self,
        buffer_view: Option<gltf_json::Index<gltf_json::buffer::View>>,
        vec: &[Vector3],
    ) -> gltf_json::Index<gltf_json::Accessor> {
        self.create_accessor(
            buffer_view,
            vec.len() as u32,
            Self::get_min_components_vec3(vec),
            Self::get_max_components_vec3(vec),
            gltf_json::accessor::ComponentType::F32,
            gltf_json::accessor::Type::Vec3,
        )
    }

    fn create_accessor_vec2(
        &mut self,
        buffer_view: Option<gltf_json::Index<gltf_json::buffer::View>>,
        vec: &[Vector2],
    ) -> gltf_json::Index<gltf_json::Accessor> {
        self.create_accessor(
            buffer_view,
            vec.len() as u32,
            Self::get_min_components_vec2(vec),
            Self::get_max_components_vec2(vec),
            gltf_json::accessor::ComponentType::F32,
            gltf_json::accessor::Type::Vec2,
        )
    }

    fn create_accessor_scalar(
        &mut self,
        buffer_view: Option<gltf_json::Index<gltf_json::buffer::View>>,
        vec: &[u32],
    ) -> gltf_json::Index<gltf_json::Accessor> {
        self.create_accessor(
            buffer_view,
            vec.len() as u32,
            None,
            None,
            // Self::get_min_components_scalar(vec),
            // Self::get_max_components_scalar(vec),
            gltf_json::accessor::ComponentType::U32,
            gltf_json::accessor::Type::Scalar,
        )
    }

    pub fn insert_vec3(&mut self, vec: &[Vector3]) -> gltf_json::Index<gltf_json::Accessor> {
        let before_len = self.buffer.len();
        self.buffer.extend(
            vec.iter()
                .flat_map(|(x, y, z)| [x.to_le_bytes(), y.to_le_bytes(), z.to_le_bytes()])
                .flatten(),
        );
        let after_len = self.buffer.len();

        let buffer_view =
            self.create_buffer_view((after_len - before_len) as u32, before_len as u32);

        self.create_accessor_vec3(Some(buffer_view), vec)
    }

    pub fn insert_vec2(&mut self, vec: &[Vector2]) -> gltf_json::Index<gltf_json::Accessor> {
        let before_len = self.buffer.len();
        self.buffer.extend(
            vec.iter()
                .flat_map(|(x, y)| [x.to_le_bytes(), y.to_le_bytes()])
                .flatten(),
        );
        let after_len = self.buffer.len();

        let buffer_view =
            self.create_buffer_view((after_len - before_len) as u32, before_len as u32);

        self.create_accessor_vec2(Some(buffer_view), vec)
    }

    pub fn insert_scalar(&mut self, vec: &[u32]) -> gltf_json::Index<gltf_json::Accessor> {
        let before_len = self.buffer.len();
        self.buffer.extend(vec.iter().flat_map(|x| x.to_le_bytes()));
        let after_len = self.buffer.len();

        let buffer_view =
            self.create_buffer_view((after_len - before_len) as u32, before_len as u32);

        self.create_accessor_scalar(Some(buffer_view), vec)
    }
}

fn export_prim_group_to_gltf(
    group: p3dhl::PrimGroup,
    buffer: &mut glTFBufferBuilder,
) -> Result<gltf_json::mesh::Primitive> {
    let mut primitive = gltf_json::mesh::Primitive {
        attributes: HashMap::new(),
        extensions: None,
        extras: gltf_json::Extras::default(),
        indices: None,
        material: None,
        mode: wrap_valid(gltf_json::mesh::Mode::Triangles),
        targets: None,
    };

    if let Some(verts) = group.vertices {
        primitive.attributes.insert(
            wrap_valid(gltf_json::mesh::Semantic::Positions),
            buffer.insert_vec3(verts),
        );
    }

    if let Some(normals) = group.normals {
        primitive.attributes.insert(
            wrap_valid(gltf_json::mesh::Semantic::Normals),
            buffer.insert_vec3(normals),
        );
    }

    if let Some(uv_map) = group.uv_map {
        primitive.attributes.insert(
            wrap_valid(gltf_json::mesh::Semantic::TexCoords(0)),
            buffer.insert_vec2(uv_map),
        );
    }

    if let Some(indices) = group.indices {
        primitive.indices = Some(buffer.insert_scalar(indices));
    }

    Ok(primitive)
}

/// Each Mesh is exported to it's own file with one buffer
fn export_mesh_to_gltf(mesh: p3dhl::Mesh, dest: &Path) -> Result<()> {
    let mut buffer = glTFBufferBuilder::new(0);

    let mut primitives = vec![];
    for group in mesh.prim_groups {
        let x = export_prim_group_to_gltf(group, &mut buffer)?;
        primitives.push(x);
    }

    let gltf_mesh = gltf_json::Mesh {
        extensions: None,
        primitives,
        extras: gltf_json::Extras::default(),
        weights: None,
    };

    let (accessors, buffer_views, buffer) = buffer.build();

    let node = gltf_json::Node {
        camera: None,
        children: None,
        extensions: None,
        extras: gltf_json::Extras::default(),
        matrix: None,
        mesh: Some(gltf_json::Index::new(0)),
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    };

    let root = gltf_json::Root {
        accessors,
        animations: Vec::new(),
        asset: gltf_json::Asset {
            copyright: None,
            extensions: None,
            extras: gltf_json::Extras::default(),
            generator: Some(format!("Khronos glTF p3d2gltf converter v{}", VERSION)),
            min_version: None,
            version: "2.0".into(),
        },
        buffers: vec![buffer],
        buffer_views,
        scene: Some(gltf_json::Index::new(0)),
        extensions: None,
        extras: gltf_json::Extras::default(),
        extensions_used: Vec::new(),
        extensions_required: Vec::new(),
        cameras: Vec::new(),
        images: Vec::new(),
        materials: Vec::new(),
        meshes: vec![gltf_mesh],
        nodes: vec![node],
        samplers: Vec::new(),
        scenes: vec![gltf_json::Scene {
            extensions: None,
            extras: gltf_json::Extras::default(),
            nodes: vec![gltf_json::Index::new(0)],
        }],
        skins: Vec::new(),
        textures: Vec::new(),
    };

    let mut errors = Vec::new();
    root.validate(&root, gltf_json::Path::new, &mut |path, error| {
        errors.push((path(), error))
    });

    // Make sure our mesh is valid!
    if !errors.is_empty() {
        return Err(eyre!("{:#?}", errors));
    }

    let serde_str = serde_json::ser::to_string_pretty(&root)?;

    let mut writer = BufWriter::new(File::create(dest.join(mesh.name).with_extension("gltf"))?);

    write!(writer, "{}", serde_str)?;

    Ok(())
}

pub fn export_all_to_gltf(tree: &[Chunk], dest: &Path) -> Result<()> {
    let hltypes = p3dhl::parse_high_level_types(tree)?;

    for hlt in hltypes {
        match hlt {
            p3dhl::HighLevelType::Mesh(mesh) => export_mesh_to_gltf(mesh, dest)?,
            p3dhl::HighLevelType::Skin(_) => todo!(),
            _ => todo!(),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
}
