use crate::FileTypes;

mod real_assets;

#[test]
fn test_file_types() {
    assert_eq!(
        FileTypes::try_from(0x00005A52)
            .expect("FileType RZ was not parsed when it should have been"),
        FileTypes::RZ
    );
}
