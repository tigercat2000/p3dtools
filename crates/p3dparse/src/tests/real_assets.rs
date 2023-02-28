use crate::{
    chunk::{
        data::{
            kinds::{image::ImageFormat, mesh::VertexType, shader_param::ShaderParamValue},
            ChunkData,
        },
        type_identifiers::ChunkType,
        Chunk, VecChunkExtension,
    },
    parse_file,
};
use bytes::Bytes;
use float_eq::assert_float_eq;
use std::path::Path;

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
    // Helper Functions
    fn check_texture(file: &[Chunk], chunk: &Chunk) {
        assert_eq!(chunk.typ, ChunkType::Texture);
        assert_eq!(chunk.get_name(), "swatchX.bmp:Texture:1:0");
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
            _ => panic!("Invalid data for {:?}", chunk.typ),
        }

        for (index, chunk) in chunk.get_children(file).enumerate() {
            match index {
                0 => {
                    assert_eq!(chunk.typ, ChunkType::Image);
                    assert_eq!(chunk.get_name(), "swatchX.bmp:Image:2:0");
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
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }

                    assert_eq!(chunk.children_len(), 1);
                    assert_eq!(chunk.get_child(file, 0).unwrap().typ, ChunkType::ImageData);
                    assert_eq!(
                        chunk.get_child(file, 0).unwrap().get_name(),
                        "<no name>:ImageData:3:0"
                    );
                    if let ChunkData::ImageRaw(raw) = &chunk.get_child(file, 0).unwrap().data {
                        assert_eq!(raw.data.len(), 861);
                    } else {
                        panic!("Invalid data for {:?}", chunk.typ);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn check_shader(file: &[Chunk], chunk: &Chunk) {
        assert_eq!(chunk.typ, ChunkType::Shader);
        assert_eq!(chunk.get_name(), "pure3dSimpleShader1:Shader:4:1");

        for (index, chunk) in chunk.get_children(file).enumerate() {
            match index {
                0 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderTextureParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "TEX");
                            assert_eq!(
                                param.value,
                                ShaderParamValue::Texture("swatchX.bmp".to_owned())
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                1 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "LIT");
                            assert_eq!(param.value, ShaderParamValue::Int(1));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                2 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "2SID");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                3 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "SHMD");
                            assert_eq!(param.value, ShaderParamValue::Int(1));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                4 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "FIMD");
                            assert_eq!(param.value, ShaderParamValue::Int(1));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                5 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "BLMD");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                6 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "PLMD");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                7 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "UVMD");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                8 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "MMIN");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                9 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "ACMP");
                            assert_eq!(param.value, ShaderParamValue::Int(4));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                10 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "ATST");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                11 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "MMAX");
                            assert_eq!(param.value, ShaderParamValue::Int(7));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                12 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderIntParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "MMEX");
                            assert_eq!(param.value, ShaderParamValue::Int(0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                13 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderFloatParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "SHIN");
                            assert_eq!(param.value, ShaderParamValue::Float(10.0));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                14 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderFloatParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "MSHP");
                            assert_eq!(param.value, ShaderParamValue::Float(0.5));
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                15 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "SPEC");
                            assert_eq!(
                                param.value,
                                ShaderParamValue::Colour([0x00, 0x00, 0x00, 0xff])
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                16 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "DIFF");
                            assert_eq!(
                                param.value,
                                ShaderParamValue::Colour([0xff, 0xff, 0xff, 0xff])
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                17 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "AMBI");
                            assert_eq!(
                                param.value,
                                ShaderParamValue::Colour([0xff, 0xff, 0xff, 0xff])
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                18 => {
                    assert_eq!(chunk.typ, ChunkType::ShaderColourParam);
                    match &chunk.data {
                        ChunkData::ShaderParam(param) => {
                            assert_eq!(param.param, "EMIS");
                            assert_eq!(
                                param.value,
                                ShaderParamValue::Colour([0x00, 0x00, 0x00, 0xff])
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn check_mesh(file: &[Chunk], chunk: &Chunk) {
        assert_eq!(chunk.typ, ChunkType::Mesh);
        assert_eq!(chunk.get_name(), "letterAShape:Mesh:24:2");
        match &chunk.data {
            ChunkData::Mesh(name, version, mesh) => {
                assert_eq!(name.0, "letterAShape");
                assert_eq!(version.0, 0);
                assert_eq!(mesh.num_prim_groups, 1);
            }
            _ => panic!("Invalid data for {:?}", chunk.typ),
        }
        for (index, chunk) in chunk.get_children(file).enumerate() {
            match index {
                0 => {
                    assert_eq!(chunk.typ, ChunkType::OldPrimGroup);
                    match &chunk.data {
                        ChunkData::OldPrimGroup(version, prim_group) => {
                            assert_eq!(version.0, 0);
                            assert_eq!(
                                prim_group.vertex_types,
                                VertexType::new()
                                    .with_uv_count(1)
                                    .with_has_normal(true)
                                    .with_has_position(true)
                            );
                            assert_eq!(prim_group.num_vertices, 172);
                            assert_eq!(prim_group.num_indices, 348);
                            assert_eq!(prim_group.num_matrices, 0);
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }

                    for (index, chunk) in chunk.get_children(file).enumerate() {
                        match index {
                            0 => {
                                assert_eq!(chunk.typ, ChunkType::VertexShader);
                                match &chunk.data {
                                    ChunkData::VertexShader(shader) => {
                                        assert_eq!(shader.vertex_shader_name, "");
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            1 => {
                                assert_eq!(chunk.typ, ChunkType::PositionList);
                                match &chunk.data {
                                    ChunkData::PositionList(pos) => {
                                        assert_eq!(pos.positions.len(), 172);
                                        assert_float_eq!(
                                            *pos.positions.first().unwrap(),
                                            [-0.4554235, -0.3471976, -0.5358086],
                                            abs <= [0.000001, 0.000001, 0.000001]
                                        );
                                        assert_float_eq!(
                                            *pos.positions.last().unwrap(),
                                            [0.4086182, -0.3471976, -0.4072655],
                                            abs <= [0.000001, 0.000001, 0.000001]
                                        );
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            2 => {
                                assert_eq!(chunk.typ, ChunkType::PackedNormalList);
                                match &chunk.data {
                                    ChunkData::PackedNormalList(normals) => {
                                        assert_eq!(normals.normals.len(), 172);
                                        assert_eq!(*normals.normals.first().unwrap(), 127u8);
                                        assert_eq!(*normals.normals.last().unwrap(), 165u8);
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            3 => {
                                assert_eq!(chunk.typ, ChunkType::NormalList);
                                match &chunk.data {
                                    ChunkData::NormalList(normals) => {
                                        assert_eq!(normals.normals.len(), 172);
                                        assert_float_eq!(
                                            *normals.normals.first().unwrap(),
                                            [-0.7941978, 0.07634445, -0.6028444],
                                            abs <= [0.000001, 0.000001, 0.000001]
                                        );
                                        assert_float_eq!(
                                            *normals.normals.last().unwrap(),
                                            [0.7470749, -0.1463135, 0.6484377],
                                            abs <= [0.000001, 0.000001, 0.000001]
                                        );
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            4 => {
                                assert_eq!(chunk.typ, ChunkType::UVList);
                                match &chunk.data {
                                    ChunkData::UVList(uvs) => {
                                        assert_eq!(uvs.channel, 0);
                                        assert_eq!(uvs.UVs.len(), 172);
                                        assert_float_eq!(
                                            *uvs.UVs.first().unwrap(),
                                            [0.2872413, 0.7369288],
                                            abs <= [0.000001, 0.00000],
                                        );
                                        assert_float_eq!(
                                            *uvs.UVs.last().unwrap(),
                                            [0.7021316, 0.6979652],
                                            abs <= [0.000001, 0.00000],
                                        );
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            5 => {
                                assert_eq!(chunk.typ, ChunkType::IndexList);
                                // Spot check on name and lineage
                                assert_eq!(chunk.get_name(), "<no name>:IndexList:31:5");
                                assert_eq!(chunk.get_lineage(file), "<no name>:IndexList:31:5 -> <no name>:OldPrimGroup:25:0 -> letterAShape:Mesh:24:2 -> <no name>:DataFile:0:0");
                                match &chunk.data {
                                    ChunkData::IndexList(indices) => {
                                        assert_eq!(indices.indices.len(), 348);
                                        assert_eq!(*indices.indices.first().unwrap(), 0);
                                        assert_eq!(*indices.indices.last().unwrap(), 153);
                                    }
                                    _ => panic!("Invalid data for {:?}", chunk.typ),
                                }
                            }
                            _ => unreachable!("More chunks were present than expected"),
                        }
                    }
                }
                1 => {
                    assert_eq!(chunk.typ, ChunkType::BBox);
                    match &chunk.data {
                        ChunkData::BoundingBox(bbox) => {
                            assert_float_eq!(
                                bbox.low,
                                [-0.4554235, -0.4327585, -0.6202587],
                                abs <= [0.000001, 0.000001, 0.000001]
                            );
                            assert_float_eq!(
                                bbox.high,
                                [0.4086182, 0.3977017, -0.3228154],
                                abs <= [0.000001, 0.000001, 0.000001]
                            );
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                2 => {
                    assert_eq!(chunk.typ, ChunkType::BSphere);
                    match &chunk.data {
                        ChunkData::BoundingSphere(bsphere) => {
                            assert_float_eq!(
                                bsphere.centre,
                                [-0.02340266, -0.01752838, -0.4715371],
                                abs <= [0.000001, 0.000001, 0.000001]
                            );
                            assert_float_eq!(bsphere.radius, 0.5535234, abs <= 0.000001);
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                3 => {
                    assert_eq!(chunk.typ, ChunkType::RenderStatus);
                    match &chunk.data {
                        ChunkData::RenderStatus(status) => {
                            assert_eq!(status.cast_shadow, 1);
                        }
                        _ => panic!("Invalid data for {:?}", chunk.typ),
                    }
                }
                _ => unreachable!("More chunks were present than expected"),
            }
        }
    }

    // Actual assertions
    if let Some(mesh) = get_asset("simplified_letter_A.p3d") {
        let file = parse_file(Bytes::from(mesh)).unwrap();
        let root = file.get_root().unwrap();
        assert_eq!(root.typ, ChunkType::DataFile);
        assert_eq!(root.data, ChunkData::None);

        for (index, chunk) in root.get_children(&file).enumerate() {
            match index {
                0 => check_texture(&file, chunk),
                1 => check_shader(&file, chunk),
                2 => check_mesh(&file, chunk),
                _ => unreachable!(),
            }
        }
    } else {
        eprintln!("Skipping test due to inability to find assets.");
    }
}

#[test]
fn test_real_homer_c() {
    // Actual assertions
    if let Some(mesh) = get_asset("homer_C.p3d") {
        let file = parse_file(Bytes::from(mesh)).unwrap();
        let root = file.get_root().unwrap();
        assert_eq!(root.typ, ChunkType::DataFile);
        assert_eq!(root.data, ChunkData::None);

        assert!(!file.iter().enumerate().any(|(index, c)| {
            if let ChunkData::Unknown = c.data {
                eprintln!("Test failure: Unknown chunk at index {}", index);
                true
            } else {
                false
            }
        }));
    } else {
        eprintln!("Skipping test due to inability to find assets.");
    }
}
