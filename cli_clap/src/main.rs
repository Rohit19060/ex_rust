use clap::{command, Arg};
use std::fs;
use std::io::{self};
use std::path::Path;
use std::process::Command;

static WORKSPACE_DIRS: &str = "D:\\Projects";

fn main() {
    let matches = command!()
        .arg(Arg::new("workspace").required(true).index(1))
        .get_matches();

    let workspace_input = matches.get_one::<String>("workspace").unwrap();
    println!("Workspace Input: {}", workspace_input);

    // Get the list of workspace files in the workspace directory
    let workspace_files = match get_workspace_files(WORKSPACE_DIRS) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error reading workspace directory: {}", err);
            pause_before_exit();
            return;
        }
    };

    prompt_for_workspace_file(&workspace_files, workspace_input);
    pause_before_exit();
}

fn get_workspace_files(dir: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
            if file_name.ends_with(".code-workspace") {
                files.push(file_name.to_string());
            }
        }
    }
    Ok(files)
}

fn prompt_for_workspace_file(workspace_files: &[String], initial_input: &str) {
    let mut input = initial_input.to_string();

    loop {
        match get_workspace_file(workspace_files, &input) {
            Some(workspace_file) => {
                open_workspace_file(&workspace_file);
                break;
            }
            None => {
                println!("Workspace file not found. Please enter the workspace file name or type 'exit' to quit:");
                for file in workspace_files {
                    println!("{}", file);
                }

                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let input_trimmed = input.trim();

                if input_trimmed.eq_ignore_ascii_case("exit") {
                    return;
                } else {
                    input = input_trimmed.to_string();
                }
            }
        }
    }
}

fn get_workspace_file(workspace_files: &[String], input: &str) -> Option<String> {
    let input_lower = input.trim().to_lowercase();
    workspace_files
        .iter()
        .find(|&file| file.to_lowercase().starts_with(&input_lower))
        .cloned()
}

fn open_workspace_file(workspace_file: &str) {
    let workspace_path = Path::new(WORKSPACE_DIRS).join(workspace_file);
    if workspace_path.exists() {
        println!("Opening workspace file: {}", workspace_path.display());
        if let Err(err) = Command::new("code").arg(workspace_path).spawn() {
            eprintln!("Failed to open the workspace file: {}", err);
        }
    } else {
        println!("Workspace file not found: {}", workspace_file);
    }
}

fn pause_before_exit() {
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
