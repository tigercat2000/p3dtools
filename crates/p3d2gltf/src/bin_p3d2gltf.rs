use clap::{arg, command, value_parser};
use indicatif::ProgressBar;
use p3d2gltf::export_all_to_gltf;
// use p3d2gltf::export_all_to_gltf;
use p3dparse::Bytes;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

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
        .arg(arg!(-r --recurse "Recurse into directories").value_parser(value_parser!(bool)))
        .arg(arg!(--list "List valid meshes to export").value_parser(value_parser!(bool)))
        .get_matches();

    match (
        matches.get_one::<PathBuf>("in"),
        matches.get_one::<PathBuf>("out"),
    ) {
        (Some(src), Some(dest)) => {
            if src.is_dir() {
                if !matches.get_flag("recurse") {
                    eprintln!("Not recursing into directory without -r flag.");
                    return;
                }

                let files: Vec<_> = WalkDir::new(src)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_name().to_string_lossy().ends_with(".p3d"))
                    .collect();

                let bar = ProgressBar::new(files.len() as u64);

                for entry in files {
                    match File::open(entry.path()) {
                        Ok(input) => {
                            match export_file(input, entry.path(), dest, matches.get_flag("list")) {
                                Ok(_) => {}
                                Err(e) => println!("{:?}", e),
                            }
                        }
                        Err(e) => {
                            println!("Unable to read file {:?}: {:?}", entry, e)
                        }
                    }

                    bar.inc(1);
                }

                bar.finish()
            } else if src.is_file() {
                let input =
                    File::open(src).unwrap_or_else(|_| panic!("Failed to open file {:?}", src));
                export_file(input, src, dest, matches.get_flag("list")).unwrap();
            } else {
                eprintln!("{:?} is not a file or directory", src);
            }
        }
        _ => unreachable!(),
    }
}

fn export_file(
    mut input: File,
    src: &Path,
    dest: &Path,
    list: bool,
) -> Result<(), eyre::ErrReport> {
    let mut input_bytes = Vec::new();
    input.read_to_end(&mut input_bytes).unwrap();

    match p3dparse::parse_file(Bytes::from(input_bytes)) {
        Ok(p3d_file) => {
            std::fs::create_dir_all(dest)
                .unwrap_or_else(|_| panic!("Failed to create directory {:?}", dest));

            if list {
                match p3dhl::parse_high_level_types(&p3d_file) {
                    Ok(hlt) => {
                        for x in hlt {
                            match x {
                                p3dhl::HighLevelType::Mesh(m) => println!("Mesh: {}", m.name),
                                p3dhl::HighLevelType::Skin(s) => println!("Skin: {}", s.name),
                                _ => {}
                            }
                        }
                    }
                    Err(e) => return Err(eyre::eyre!("Failed to parse file: {:#?}", e)),
                }
            } else {
                match export_all_to_gltf(src, &p3d_file, dest) {
                    Ok(_) => {
                        // if let Some(name) = src.file_name() {
                        //     println!("Exported file {}", name.to_string_lossy());
                        // } else {
                        //     println!("Exported file {:?}", src);
                        // }
                    }
                    Err(e) => {
                        return Err(eyre::eyre!(
                            "Failed to export file {:?} due to error: {}",
                            src,
                            e
                        ));
                    }
                }
            }
        }
        Err(e) => {
            return Err(eyre::eyre!(
                "Failed to export file {:?}, could not parse file: {:#?}",
                src,
                e
            ));
        }
    }

    Ok(())
}
