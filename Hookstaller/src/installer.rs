
// $ setup-sai-hooks
mod util;
use std::error::Error;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::str::FromStr;
use crate::util::{check_y_n, get_input, Config};

// git config --global core.hooksPath /path/to/my/centralized/hooks
fn install_git_hook() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_input("Should this hook be installed globally? (Y|N)")?);

    if should_apply_globally {
        let mut exec_path = std::env::current_dir()?;
        exec_path.push("target");
        exec_path.push("release");
        let mut git_cmd = Command::new("git");
        let args = vec!["config", "--global", "core.hooksPath", exec_path.to_str().unwrap()];
        git_cmd.args(args);
        git_cmd.output()?;
    } else {
        let mut post_commit_executable_path = PathBuf::new();
        post_commit_executable_path.push("target");
        post_commit_executable_path.push("release");
        post_commit_executable_path.push("post-commit");
        println!("post_commit_executable_path {}", post_commit_executable_path.to_str().unwrap());
        loop {
            let repo_dir = get_input("What is an absolute path to a git repo? (Press q and enter to quit.)")?;
            if repo_dir == "q" {
                break;
            }
            if repo_dir == "" {
                continue;
            }

            // Check repo_directory for .git/hooks folder
            println!("Before git_hooks_path");
            let mut git_hooks_path = PathBuf::new();
            git_hooks_path.push(".git");
            git_hooks_path.push("hooks");
            println!("pushed git_hooks_path {}", git_hooks_path.to_str().unwrap());

            let mut hooks_dir = PathBuf::from_str(&repo_dir)?;
            hooks_dir.push(git_hooks_path);
            println!("hooks_dir {}", hooks_dir.to_str().unwrap());

            if hooks_dir.exists() {
                hooks_dir.push("post-commit");
                println!("post_commit_executable_path {}", post_commit_executable_path.to_str().unwrap());
                fs::copy(post_commit_executable_path.as_path(), hooks_dir)?;
            } else {
                println!("Given directory is not a git repository.");
            }
        }

    }
    Ok(())
}

fn main() {
    let cfg = match Config::create_from_input() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    match cfg.save_to_file() {
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
    match install_git_hook() {
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
}
