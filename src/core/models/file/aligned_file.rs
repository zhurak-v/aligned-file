use std::io;
use std::fs::File;
use std::path::PathBuf;

use crate::core::models::alignment::alignment::Alignment;

pub struct AlignedFile {
    path_buf: PathBuf,
    header: AlignedFileHeader,
    metadata: AlignedFileMetadata,
    data: AlignedFileData,
}

impl AlignedFile {
    fn create_files(path_buf: &PathBuf) -> io::Result<(File, File)> {
        let data_file: File = File::create(path_buf)?;
        let metadata_path: PathBuf = PathBuf::from(format!(
            "{}.metadata",
            path_buf.to_string_lossy()
        ));
        let metadata_file: File = File::create(metadata_path)?;

        Ok((data_file, metadata_file))
    }
}

impl AlignedFile {
    pub fn create(path_buf: PathBuf, alignment: Alignment) -> AlignedFile {
        let header: AlignedFileHeader = AlignedFileHeader::new(alignment);
        let metadata: AlignedFileMetadata = AlignedFileMetadata::new();
        let data: AlignedFileData = AlignedFileData::new();

        let files: (File, File) = AlignedFile::create_files(&path_buf).unwrap();

        AlignedFile {
            path_buf,
            header,
            metadata,
            data,
        }
    }
}

pub struct AlignedFileHeader {
    magic: [u8; 8],
    version: u32,
    alignment: Alignment,
}

impl AlignedFileHeader {
    pub const MAGIC: [u8; 8] = *b"zhurak-v";
    pub const VERSION: u32 = 1;

    pub fn new(alignment: Alignment) -> AlignedFileHeader {
        AlignedFileHeader {
            magic: AlignedFileHeader::MAGIC,
            version: AlignedFileHeader::VERSION,
            alignment,
        }
    }
}

pub struct AlignedFileMetadata {
    free_blocks: FreeBlocks,
}

impl AlignedFileMetadata {
    pub fn new() -> AlignedFileMetadata {
        AlignedFileMetadata {
            free_blocks: FreeBlocks::new(),
        }
    }
}

pub struct FreeBlocks {
    blocks: Vec<AlignedFileBlock>,
}

impl FreeBlocks {
    pub fn new() -> FreeBlocks {
        FreeBlocks {
            blocks: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        (*self).blocks.len()
    }
}

pub struct AlignedFileBlock {
    offset: usize,
    length: usize,
}

pub struct AlignedFileData;

impl AlignedFileData {
    pub fn new() -> AlignedFileData {
        AlignedFileData
    }
}