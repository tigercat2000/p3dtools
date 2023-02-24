use crate::{chunk::data::kinds::mesh::VertexType, FileTypes};

mod real_assets;

#[test]
fn test_file_types() {
    assert_eq!(
        FileTypes::try_from(0x00005A52)
            .expect("FileType RZ was not parsed when it should have been"),
        FileTypes::RZ
    );
}

#[test]
/// Ensure that the serde implementation for VertexType encodes it as a plain u32
fn test_serde_vertextype() {
    let vert = VertexType::new()
        .with_uv_count(1)
        .with_has_normal(true)
        .with_has_position(true);

    assert_eq!(serde_json::to_string(&vert).unwrap(), "8209");
}
