use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub struct Storage;

impl Storage {
    pub fn new() -> Self {
        Storage
    }
}

pub trait StorageTrait {
    fn file_or_directory_exists(&self, file_path: &str) -> Result<bool, String>;
    fn create_file(&self, file_path: &str) -> Result<(), String>;
    fn create_directory(&self, dir_path: &str) -> Result<(), String>;
    fn read_file(&self, file_path: &str) -> Result<String, String>;
    fn overwrite_file(&self, file_path: &str, content: &str) -> Result<(), String>;
    fn delete_file(&self, file_path: &str) -> Result<(), String>;
}

impl StorageTrait for Storage {
    fn file_or_directory_exists(&self, file_or_directory_path: &str) -> Result<bool, String> {
        Ok(Path::new(file_or_directory_path).exists())
    }

    fn create_file(&self, file_path: &str) -> Result<(), String> {
        File::create(file_path).map(|_| ()).map_err(|e| e.to_string())
    }

    fn create_directory(&self, dir_path: &str) -> Result<(), String> {
        fs::create_dir_all(dir_path).map_err(|e| e.to_string())
    }

    fn read_file(&self, file_path: &str) -> Result<String, String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| e.to_string())?;
        Ok(content)
    }

    fn overwrite_file(&self, file_path: &str, content: &str) -> Result<(), String> {
        let mut file = File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes()).map_err(|e| e.to_string())
    }

    fn delete_file(&self, file_path: &str) -> Result<(), String> {
        fs::remove_file(file_path).map_err(|e| e.to_string())
    }
}