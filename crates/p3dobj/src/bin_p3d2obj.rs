#![feature(file_create_new)]
use clap::{arg, command, value_parser};
use p3dobj::FullMesh;
use p3dparse::{chunk::type_identifiers::ChunkType, Bytes};
use std::{fs::File, io::Read, path::PathBuf};

fn main() {
    let matches = command!()
        .arg(
            arg!(-i --in <FILE> "Source p3d file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --out <FOLDER> "Destination folder")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    match (
        matches.get_one::<PathBuf>("in"),
        matches.get_one::<PathBuf>("out"),
    ) {
        (Some(src), Some(dest)) => {
            let mut input =
                File::open(src).unwrap_or_else(|_| panic!("Failed to open file {:?}", src));

            let mut input_bytes = Vec::new();
            input.read_to_end(&mut input_bytes).unwrap();
            let p3d_file = p3dparse::parse_file(Bytes::from(input_bytes)).unwrap();

            for mesh in p3d_file.iter().filter(|c| c.typ == ChunkType::Mesh) {
                let mesh = FullMesh::parse(mesh, &p3d_file).unwrap();
                mesh.write_obj(&dest.join(mesh.name.clone()).with_extension("obj"))
                    .unwrap();
            }
        }
        _ => unreachable!(),
    }
}
