use std::path::PathBuf;
use aligned_file::core::models::alignment::alignment::Alignment;
use aligned_file::core::models::file::aligned_file::AlignedFile;

fn main() {
    let aligned_file: AlignedFile = AlignedFile::create(
        PathBuf::from("data/file.af"), Alignment::Byte512
    );
}