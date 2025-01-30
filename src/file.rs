use std::fs;
use std::io;

#[derive(Debug)]
pub enum FileType {
    Unknown,
    HMG,
    FBK,
    FX4,
}

#[derive(Debug)]
pub struct FileReader {
    file_name: String,
    file_buffer: fs::File,
}

impl FileReader {
    pub fn new(file_name: String) -> FileReader {
        let file_buffer = match fs::File::open(&file_name) {
            Ok(file) => file,
            Err(e) => panic!("Failed to open file: {}", e),
        };

        FileReader {
            file_name,
            file_buffer,
        }
    }

    pub fn stream(&self) -> io::BufReader<fs::File> {
        io::BufReader::new(self.file_buffer.try_clone().unwrap())
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn size(&self) -> u64 {
        self.file_buffer.metadata().unwrap().len()
    }

    pub fn close(&self) {
        drop(&self.file_buffer);
    }
}

pub fn get_file_type(file_name: String) -> FileType {
    let extension =
        &file_name.to_lowercase()[file_name.chars().count() - 3..file_name.chars().count()];
    if extension == "fbk" {
        return FileType::FBK;
    }
    if extension == "hmg" {
        return FileType::HMG;
    }
    if extension == "fx4" {
        return FileType::FX4;
    }
    FileType::Unknown
}
