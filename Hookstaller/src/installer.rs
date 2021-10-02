
// $ setup-sai-hooks
mod util;
use std::error::Error;
use std::process::Command;
use std::path::{PathBuf};
use std::fs;
use std::env;
use std::str::FromStr;
use crate::util::{check_y_n, get_input, Config};

fn install_git_hook() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_installer_input("Should this hook be installed globally? (Y|N)")?);

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
        loop {
            let repo_dir = get_installer_input("What is an absolute path to a git repo? (Press q and enter to quit.)")?;
            if repo_dir == "q" {
                break;
            }
            if repo_dir == "" {
                continue;
            }

            // Check repo_directory for .git/hooks folder
            let mut git_hooks_path = PathBuf::new();
            git_hooks_path.push(".git");
            git_hooks_path.push("hooks");

            let mut hooks_dir = PathBuf::from_str(&repo_dir)?;
            hooks_dir.push(git_hooks_path);

            if hooks_dir.exists() {
                let os = env::consts::OS;
                let post_commit_file = match os { "windows" => "post-commit.exe", _ => "post-commit" };
                hooks_dir.push(post_commit_file);
                println_log(format!("Installing from {} to {}", post_commit_executable_path.to_str().unwrap(), hooks_dir.to_str().unwrap()));
                fs::copy(post_commit_executable_path.as_path(), hooks_dir)?;
            } else {
                println_error("Given directory is not a git repository.");
            }
        }

    }
    Ok(())
}

fn get_installer_input<S: AsRef<str>>(output: S) -> Result<String, Box<dyn Error>> {
    get_input(log_format(output))
}

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("[Git-Hook-Installer]: {}", output.as_ref())
}

fn println_error<S: AsRef<str>>(output: S) {
    println!("\\e[1;96;127m{}\\e[0m\n", log_format(output));
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}

fn create_from_input() -> Result<Config, Box<dyn Error>> {
    let team_name = get_installer_input("What is your Team Name?")?;
    let email = get_installer_input("What is your Source Allies Email?")?;

    Ok(Config {
        team_name,
        email
    })
}

fn main() {
    let cfg = match create_from_input() {
        Ok(cfg) => cfg,
        Err(e) => {
            println_error(e.to_string());
            return;
        }
    };
    match cfg.save_to_file() {
        Err(e) => {
            println_error(e.to_string());
        }
        _ => {}
    }
    match install_git_hook() {
        Err(e) => {
            println_error(e.to_string());
        }
        _ => {}
    }
}
