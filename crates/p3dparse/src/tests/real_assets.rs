use std::path::Path;

use bytes::Bytes;

use crate::{
    chunk::{
        data::{kinds::image::ImageFormat, ChunkData},
        type_identifiers::ChunkType,
    },
    parse_file,
};

fn get_asset(name: &str) -> Option<Vec<u8>> {
    let path = Path::new("real_assets");
    if path.is_dir() {
        let asset = path.join(name);
        Some(std::fs::read(asset).unwrap())
    } else {
        None
    }
}

#[test]
fn test_mesh() {
    if let Some(mesh) = get_asset("simplified_letter_A.p3d") {
        let file = parse_file(Bytes::from(mesh)).unwrap();
        assert_eq!(file.typ, ChunkType::DataFile);
        assert_eq!(file.data, ChunkData::None);
        for (index, chunk) in file.children.iter().enumerate() {
            match index {
                0 => {
                    assert_eq!(chunk.typ, ChunkType::Texture);
                    assert_eq!(chunk.get_name(), "swatchX.bmp");
                    match &chunk.data {
                        ChunkData::Texture(name, version, texture) => {
                            assert_eq!(name.0, "swatchX.bmp");
                            assert_eq!(version.0, 14000);
                            assert_eq!(texture.width, 8);
                            assert_eq!(texture.height, 8);
                            assert_eq!(texture.bpp, 8);
                            assert_eq!(texture.alpha_depth, 0);
                            assert_eq!(texture.num_mip_maps, 1);
                            assert_eq!(texture.texture_type, 1);
                            assert_eq!(texture.usage, 0);
                            assert_eq!(texture.priority, 0);
                        }
                        _ => panic!("Invalid data for Texture"),
                    }

                    for (index, chunk) in chunk.children.iter().enumerate() {
                        match index {
                            0 => {
                                assert_eq!(chunk.typ, ChunkType::Image);
                                assert_eq!(chunk.get_name(), "swatchX.bmp");
                                match &chunk.data {
                                    ChunkData::Image(name, version, image) => {
                                        assert_eq!(name.0, "swatchX.bmp");
                                        assert_eq!(version.0, 14000);
                                        assert_eq!(image.width, 8);
                                        assert_eq!(image.height, 8);
                                        assert_eq!(image.bpp, 8);
                                        assert_eq!(image.palettized, 1);
                                        assert_eq!(image.has_alpha, 0);
                                        assert_eq!(image.image_format, ImageFormat::PNG);
                                    }
                                    _ => panic!("Invalid data for Image"),
                                }

                                assert_eq!(chunk.children.len(), 1);
                                assert_eq!(chunk.children[0].typ, ChunkType::ImageData);
                                assert_eq!(chunk.children[0].get_name(), "");
                                if let ChunkData::ImageRaw(raw) = &chunk.children[0].data {
                                    assert_eq!(raw.data.len(), 861);
                                } else {
                                    panic!("Invalid data for ImageData");
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                1 => assert_eq!(chunk.typ, ChunkType::Shader),
                2 => assert_eq!(chunk.typ, ChunkType::Mesh),
                _ => unreachable!(),
            }
        }
    }
}
