use crate::data_access::repository::{Repository, RepositoryTrait};
use crate::dto::{commit_dto::CommitDTO, branch_dto::BranchDTO, status_dto::StatusDTO};

pub struct GitOperationsState;

impl GitOperationsState {
    pub fn new() -> Self {
        GitOperationsState
    }
}

pub trait GitOperationsTrait {
    fn init_repository(&mut self) -> Result<(), String>;
    fn add_to_stage(&mut self, file_path: &str) -> Result<(), String>;
    // fn commit_changes(&mut self, commit: CommitDTO) -> Result<(), String>;
    // fn create_branch(&mut self, branch: BranchDTO) -> Result<(), String>;
    // fn checkout_branch(&mut self, branch_name: &str) -> Result<(), String>;
    // fn merge_branch(&mut self, branch_name: &str) -> Result<(), String>;
    // fn view_log(&mut self) -> Result<Vec<CommitDTO>, String>;
    // fn check_status(&mut self) -> Result<StatusDTO, String>;
    // fn show_diff(&mut self) -> Result<(), String>;
}

pub struct GitOperations {
    state: GitOperationsState,
    repository: Repository,
}

impl GitOperations {
    pub fn new(root_path: &str) -> Self {
        GitOperations {
            state: GitOperationsState::new(),
            repository: Repository::new(root_path),
        }
    }
}

impl GitOperationsTrait for GitOperations {
    fn init_repository(&mut self) -> Result<(), String> {
        if self.repository.repo_exists()? {
            return Err("A Git repository already exists in directory.".to_string());
        }

        match self.repository.create_repo_structure() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to initialize repository: {}", e)),
        }
    }

    fn add_to_stage(&mut self, file_path: &str) -> Result<(), String> {
        if !self.repository.repo_exists()? {
            return Err("No Git repository found in directory.".to_string());
        }

        // Check if the file exists
        if !self.repository.file_exists(file_path)? {
            return Err(format!("File {} does not exist", file_path));
        }

        // Check if the file is already staged
        if self.repository.file_staged(file_path)? {
            return Err(format!("File {} is already staged", file_path));
        }

        // Add the file to the staging area
        self.repository.stage_file(file_path)?;

        Ok(())
    }

    // fn commit_changes(&mut self, commit: CommitDTO) -> Result<(), String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to commit staged changes
    //     Ok(())
    // }
    //
    // fn create_branch(&mut self, branch: BranchDTO) -> Result<(), String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to create a new branch
    //     Ok(())
    // }
    //
    // fn checkout_branch(&mut self, branch_name: &str) -> Result<(), String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to switch to an existing branch
    //     Ok(())
    // }
    //
    // fn merge_branch(&mut self, branch_name: &str) -> Result<(), String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to merge a branch into the current branch
    //     Ok(())
    // }
    //
    // fn view_log(&mut self) -> Result<Vec<CommitDTO>, String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to view commit history
    //     Ok(vec![])
    // }
    //
    // fn check_status(&mut self) -> Result<StatusDTO, String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to check the current status of the working directory
    //     Ok(StatusDTO {
    //         staged_files: vec![],
    //         unstaged_files: vec![],
    //         untracked_files: vec![],
    //     })
    // }
    //
    // fn show_diff(&mut self) -> Result<(), String> {
    //     self.ensure_repo_exists(&self.state.root_path)?;
    //     // Function to show differences between staged and working directory
    //     Ok(())
    // }
}