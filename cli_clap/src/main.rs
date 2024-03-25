use clap::{command, Arg};

fn main() {
    let match_result = command!()
        .arg(Arg::new("workspace").required(true).index(1))
        .get_matches();

    let workspace_str = match_result.get_one::<String>("workspace").unwrap();

    let workspace_dirs = vec!["D:\\Projects"];

    for workspace_dir in workspace_dirs {
        let workspace_file = std::fs::read_dir(workspace_dir)
            .unwrap()
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.file_name().into_string().ok().and_then(|s| {
                        if s.ends_with(".code-workspace") {
                            Some(s)
                        } else {
                            None
                        }
                    })
                })
            })
            .find(|s| s.to_lowercase().starts_with(&workspace_str.to_lowercase()));

        if let Some(workspace_file) = workspace_file {
            let workspace_path = std::path::Path::new(workspace_dir).join(workspace_file);
            println!("Opening workspace: {}", workspace_path.display());
            std::process::Command::new("explorer")
                .arg(workspace_path)
                .spawn()
                .expect("Failed to open workspace");
            return;
        }
    }
}
