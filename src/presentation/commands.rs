use crate::service::git_operations::{GitOperations, GitOperationsTrait};

fn handle_init_command(git_operations: &mut dyn GitOperationsTrait) {
    match git_operations.init_repository() {
        Ok(_) => println!("Repository initialized successfully."),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }
}

fn handle_add_command(git_operations: &mut dyn GitOperationsTrait, file_path: &str) {
    match git_operations.add_to_stage(file_path) {
        Ok(_) => println!("File added to staging area."),
        Err(e) => println!("Failed to add file to staging area: {}", e),
    }
}

pub fn handle_command(command: &str, _args: &[&str]) {
    let mut git_operations = GitOperations::new("C:\\Users\\ollie\\Documents\\test");

    // Function to handle different commands
    match command {
        "help" => {
            // Print the possible commands
            println!("Possible commands:");
            println!("help - Show available commands");
            println!("init - Initialize a new repository");
            println!("add <file> - Add a file to the staging area");
            println!("commit <message> - Commit changes to the repository");
            println!("branch <name> - Create a new branch");
            println!("checkout <name> - Switch to a branch");
            println!("merge <name> - Merge a branch into the current branch");
            println!("log - View commit history");
            println!("status - Check repository status");
            println!("diff - Show differences");
        }
        "init" => {
            handle_init_command(&mut git_operations);
        }
        "add" => {
            // Add a file to the staging area getting the file path from the arguments
            handle_add_command(&mut git_operations, _args[0]);
        }
        // "commit" => {
        //     // Commit changes
        //     println!("Commit changes");
        // }
        // "branch" => {
        //     // Create a new branch
        //     println!("Create a new branch");
        // }
        // "checkout" => {
        //     // Switch to a branch
        //     println!("Switch to a branch");
        // }
        // "merge" => {
        //     // Merge a branch
        //     println!("Merge a branch");
        // }
        // "log" => {
        //     // View commit history
        //     println!("View commit history");
        // }
        // "status" => {
        //     // Check repository status
        //     println!("Check repository status");
        // }
        // "diff" => {
        //     // Show differences
        //     println!("Show differences");
        // }
        _ => {
            // Print error message
            println!("Invalid command '{}'. Use 'help' to see available commands.", command);
        }
    }
}