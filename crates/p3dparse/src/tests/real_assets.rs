use std::path::Path;

use bytes::Bytes;

use crate::{
    chunk::{
        data::{
            kinds::{image::ImageFormat, shader_param::ShaderParamValue},
            ChunkData,
        },
        type_identifiers::ChunkType,
        VecChunkExtension,
    },
    parse_file,
};

fn get_asset(name: &str) -> Option<Vec<u8>> {
    let path = Path::new("src/tests/real_assets");
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
        let root = file.get_root().unwrap();
        assert_eq!(root.typ, ChunkType::DataFile);
        assert_eq!(root.data, ChunkData::None);
        for (index, chunk) in root.get_children(&file).iter().enumerate() {
            match index {
                0 => {
                    assert_eq!(chunk.typ, ChunkType::Texture);
                    assert_eq!(chunk.get_name(), "swatchX.bmp\0(Texture)");
                    match &chunk.data {
                        ChunkData::Texture(name, version, texture) => {
                            assert_eq!(name.0, "swatchX.bmp\0");
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

                    for (index, chunk) in chunk.get_children(&file).iter().enumerate() {
                        match index {
                            0 => {
                                assert_eq!(chunk.typ, ChunkType::Image);
                                assert_eq!(chunk.get_name(), "swatchX.bmp\0(Image)");
                                match &chunk.data {
                                    ChunkData::Image(name, version, image) => {
                                        assert_eq!(name.0, "swatchX.bmp\0");
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
                                assert_eq!(
                                    chunk.get_child(&file, 0).unwrap().typ,
                                    ChunkType::ImageData
                                );
                                assert_eq!(
                                    chunk.get_child(&file, 0).unwrap().get_name(),
                                    "(ImageData)"
                                );
                                if let ChunkData::ImageRaw(raw) =
                                    &chunk.get_child(&file, 0).unwrap().data
                                {
                                    assert_eq!(raw.data.len(), 861);
                                } else {
                                    panic!("Invalid data for ImageData");
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                1 => {
                    assert_eq!(chunk.typ, ChunkType::Shader);
                    assert_eq!(chunk.get_name(), "pure3dSimpleShader1\0(Shader)");

                    for (index, chunk) in chunk.get_children(&file).iter().enumerate() {
                        match index {
                            0 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderTextureParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "TEX\0");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Texture("swatchX.bmp\0".to_owned())
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            1 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "LIT\0");
                                        assert_eq!(param.value, ShaderParamValue::Int(1));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            2 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "2SID");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            3 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "SHMD");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Int(0),
                                            "Failed on {}",
                                            chunk.get_lineage(&file)
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            4 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "FIMD");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            5 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "BLMD");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            6 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "PLMD");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            7 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "UVMD");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            8 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "MMIN");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            9 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "ACMP");
                                        assert_eq!(param.value, ShaderParamValue::Int(4));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            10 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "ATST");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            11 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "MMAX");
                                        assert_eq!(param.value, ShaderParamValue::Int(7));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            12 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "MMEX");
                                        assert_eq!(param.value, ShaderParamValue::Int(0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            13 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderFloatParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "SHIN");
                                        assert_eq!(param.value, ShaderParamValue::Float(10.0));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            14 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderFloatParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "MSHP");
                                        assert_eq!(param.value, ShaderParamValue::Float(0.5));
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            15 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "SPEC");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Colour((0x00, 0x00, 0x00, 0xff))
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            16 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "DIFF");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Colour((0xff, 0xff, 0xff, 0xff))
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            17 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "AMBI");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Colour((0xff, 0xff, 0xff, 0xff))
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            18 => {
                                assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                                match &chunk.data {
                                    ChunkData::ShaderParam(param) => {
                                        assert_eq!(param.param, "EMIS");
                                        assert_eq!(
                                            param.value,
                                            ShaderParamValue::Colour((0x00, 0x00, 0x00, 0xff))
                                        );
                                    }
                                    _ => panic!("Invalid data for ShaderTextureParam"),
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                2 => assert_eq!(chunk.typ, ChunkType::MeshStats),
                _ => unreachable!(),
            }
        }
    } else {
        eprintln!("Skipping test due to inability to find assets.");
    }
}
