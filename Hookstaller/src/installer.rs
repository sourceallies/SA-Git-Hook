mod util;
use std::error::Error;
use std::process::Command;
use std::path::{PathBuf};
use std::fs;
use std::env;
use std::str::FromStr;
use crate::util::{check_y_n, get_input, Config};

static APP_HEADER: &str = "[Git-Hook-Installer]";

fn install_git_hook() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_input(log_format("Should this hook be installed globally? (Y|N)"))?);
    let os = env::consts::OS;
    let post_commit_executable = PathBuf::from(match os { "windows" => "post-commit.exe", _ => "post-commit" });

    if !post_commit_executable.exists() {
        //TODO: Maybe check for the post-commit hook inside the config directory?
        Err("Can't Find Git Hook in current directory")?;
    }

    if should_apply_globally {
        let mut global_path = Config::config_dir_path();
        global_path.push(post_commit_executable.as_path());
        fs::copy(post_commit_executable, global_path)?;

        let mut git_cmd = Command::new("git");
        let path = Config::config_dir_path();
        let args = vec!["config", "--global", "core.hooksPath", path.to_str().unwrap()];
        git_cmd.args(args);
        //TODO: Check status to see if failed and try again?
        git_cmd.output()?;
    } else {
        loop {
            let repo_dir = get_input(log_format("What is an absolute path to a git repo? (Press q and enter to quit.)"))?;
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
                hooks_dir.push(post_commit_executable.as_path());
                println_log(format!("Installing {} to {}", post_commit_executable.to_str().unwrap(), hooks_dir.to_str().unwrap()));
                fs::copy(post_commit_executable.as_path(), hooks_dir)?;
            } else {
                println_log("Given directory is not a git repository.");
            }
        }

    }
    Ok(())
}

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("{}: {}", APP_HEADER, output.as_ref())
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}

fn main() {
    let cfg = match Config::read_input(APP_HEADER) {
        Ok(cfg) => cfg,
        Err(e) => {
            println_log(e.to_string());
            return;
        }
    };
    match cfg.save_to_file() {
        Err(e) => {
            println_log(e.to_string());
        }
        _ => {}
    }
    match install_git_hook() {
        Err(e) => {
            println_log(e.to_string());
        }
        _ => {}
    }
}
