
// $ setup-sai-hooks
mod util;
use std::error::Error;
use std::process::Command;
use std::path::Path;
use std::fs;
use crate::util::{check_y_n, get_input, Config};

// git config --global core.hooksPath /path/to/my/centralized/hooks
fn install_git_hook() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_input("Should this hook be installed globally? (Y|N)")?);
    let post_commit_executable_path = "../PostCommit/target/release/post-commit";
    if should_apply_globally {
        let mut git_cmd = Command::new("git");
        // todo: get path to post-commit executable
        let args = vec!["config", "--global", "core.hooksPath", post_commit_executable_path];
        git_cmd.args(args);
    } else {
        loop {
            let repo_dir = get_input("What is an absolute path to a git repo? (Press q and enter to quit.)")?;
            if repo_dir == "q" {
                break;
            }
            if repo_dir == "" {
                continue;
            }
            // Check repo_directory for .git/hooks folder

            let hooks_dir = match repo_dir.ends_with('/') { true => repo_dir, false => repo_dir + "/" } + ".git/hooks/";
            if Path::new(&hooks_dir).exists() {
                // TODO: Place "post-commit" binary inside folder
                fs::copy(post_commit_executable_path, hooks_dir)?;
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
