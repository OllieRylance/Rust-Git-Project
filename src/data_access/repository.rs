use crate::data_storage::storage::{Storage, StorageTrait};
use chrono::prelude::*;

pub struct RepositoryState {
    root_path: String,
}

impl RepositoryState {
    pub fn new(root_path: &str) -> Self {
        RepositoryState {
            root_path: root_path.to_string(),
        }
    }
}

pub struct Repository {
    state: RepositoryState,
    storage: Storage,
}

impl Repository {
    pub fn new(root_path: &str) -> Self {
        Repository{
            state: RepositoryState::new(root_path),
            storage: Storage::new(),
        }
    }
}

pub trait RepositoryTrait {
    fn repo_exists(&self) -> Result<bool, String>;
    fn create_repo_structure(&self) -> Result<(), String>;
    fn file_exists(&self, file_path: &str) -> Result<bool, String>;
    fn file_staged(&self, file_path: &str) -> Result<bool, String>;
    fn stage_file(&self, file_path: &str) -> Result<(), String>;
    // fn create_commit_object(&self, message: &str);
    // fn add_branch(&self, branch_name: &str);
    // fn switch_branch(&self, branch_name: &str);
    // fn apply_merge(&self, branch_name: &str);
    // fn get_commit_history(&self);
    // fn get_status(&self);
    // fn get_diff(&self);
}

impl RepositoryTrait for Repository {
    fn repo_exists(&self) -> Result<bool, String> {
        let git_dir = format!("{}/.git", self.state.root_path);
        self.storage.file_or_directory_exists(&git_dir)
    }

    fn create_repo_structure(&self) -> Result<(), String> {
        // Create the .git directory
        let git_dir = format!("{}/.git", self.state.root_path);
        self.storage.create_directory(&git_dir)?;

        Ok(())
    }

    fn file_exists(&self, file_path: &str) -> Result<bool, String> {
        // Add the file_path to the root_path to get the full path
        let file_path = format!("{}/{}", self.state.root_path, file_path);

        self.storage.file_or_directory_exists(&file_path)
    }

    fn file_staged(&self, file_path: &str) -> Result<bool, String> {
        // Function to check if a file is staged
        // To test if a file is staged, we can check if the file exists in the staging area which is a directory in the .git/index file
        let index_path = format!("{}/.git/index", self.state.root_path);
        
        // Check if the index file exists
        match self.storage.file_or_directory_exists(&index_path) {
            Ok(exists) => {
                if !exists {
                    return Ok(false);
                }
            },
            Err(e) => return Err(e),
        }
        
        // Check the contents of the index file to see if the file is staged
        let index_content = match self.storage.read_file(&index_path) {
            Ok(content) => content,
            Err(e) => return Err(e),
        };

        let index_lines: Vec<&str> = index_content.split('\n').collect();

        for line in index_lines {
            let parts: Vec<&str> = line.split(";").collect();
            if parts.len() > 0 && parts[0] == file_path {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn stage_file(&self, file_path: &str) -> Result<(), String> {
        // Function to handle file staging logic
        // To stage a file, we need to add the file path to the index file in the .git directory
        let index_path = format!("{}/.git/index", self.state.root_path);

        // Read the current content of the index file
        let index_content = match self.storage.read_file(&index_path) {
            Ok(content) => content,
            Err(_) => "".to_string(),
        };

        // Append the file path and a human readable timestamp to the index content
        let timestamp = Local::now().to_rfc3339();
        let new_index_content = format!("{}{};{}\n", index_content, file_path, timestamp);

        // Write the updated content back to the index file and return the result
        self.storage.overwrite_file(&index_path, &new_index_content)
    }

    // fn create_commit_object(&self, message: &str) {
    //     // Function to handle commit creation logic
    // }
    //
    // fn add_branch(&self, branch_name: &str) {
    //     // Function to add branch information to repository
    // }
    //
    // fn switch_branch(&self, branch_name: &str) {
    //     // Function to change the current branch
    // }
    //
    // fn apply_merge(&self, branch_name: &str) {
    //     // Function to handle the merging logic
    // }
    //
    // fn get_commit_history(&self) {
    //     // Function to retrieve commit history
    // }
    //
    // fn get_status(&self) {
    //     // Function to gather status information
    // }
    //
    // fn get_diff(&self) {
    //     // Function to compute differences
    // }
}