mod util;
use std::error::Error;
use std::process::Command;
use std::path::{PathBuf};


use std::str::FromStr;
use crate::util::{APP_DIR_NAME, APP_BIN_DIR_NAME};
use clap::{App, Arg};
use crate::util::fs::{app_dir_path, app_bin_dir_path, app_hooks_dir_path, is_git_directory, os_specific_binary_name, create_if_not_exists};
use crate::util::{APP_HOOK_DIR_NAME, HOOK_BIN_NAME, INSTALLER_BIN_NAME, APP_BIN_NAME};
use crate::util::config::Config;
use crate::util::input::{check_y_n, get_input};

static APP_HEADER: &str = "[Commit-Collective-Installer]";

macro_rules! soft_unwrap  {
    ($name:expr) => {
        match $name {
            Ok(x) => x,
            Err(e) => {
                println_log(e.to_string());
                return;
            }
        }
    }
}

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("{}: {}", APP_HEADER, output.as_ref())
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}


pub fn create_app_dir() -> Result<PathBuf, Box<dyn Error>> {
    let mut app_dir = app_dir_path();

    if !app_dir.exists() {
        std::fs::create_dir(&app_dir)?;
    }

    app_dir.push(APP_BIN_DIR_NAME);

    if !app_dir.exists() {
        std::fs::create_dir(&app_dir)?;
    }
    app_dir.pop();

    app_dir.push(APP_HOOK_DIR_NAME);

    if !app_dir.exists() {
        std::fs::create_dir(&app_dir)?;
    }
    app_dir.pop();

    Ok(app_dir)
}

/// Moves the installer and the post-commit hook into the bin folder and renames the installer
fn move_executables() -> Result<(), Box<dyn Error>> {
    let mut executable_directory = std::env::current_exe()?;
    executable_directory.pop();

    let mut installer_executable = executable_directory.clone();
    let installer_exec_name = os_specific_binary_name(INSTALLER_BIN_NAME);
    installer_executable.push(&installer_exec_name);

    if !installer_executable.exists() {
        Err("Install Binary can't be found")?;
    }

    let mut hook_executable = executable_directory.clone();
    let hook_exec_name = os_specific_binary_name(HOOK_BIN_NAME);
    hook_executable.push(&hook_exec_name);

    if !hook_executable.exists() {
        Err("Hook Binary can't be found")?;
    }

    let mut installer_to_path = create_if_not_exists(app_bin_dir_path())?;
    installer_to_path.push(installer_exec_name);

    if installer_to_path != installer_executable {
        std::fs::copy(installer_executable, &installer_to_path)?;
        let mut app_bin = installer_to_path.clone();
        app_bin.pop();
        app_bin.push(os_specific_binary_name(APP_BIN_NAME));
        std::fs::rename(&installer_to_path, app_bin)?;
    }

    let mut hook_to_path = create_if_not_exists(app_hooks_dir_path())?;
    hook_to_path.push(hook_exec_name);

    if hook_to_path != hook_executable {
        std::fs::copy(hook_executable, &hook_to_path)?;
    }

    Ok(())
}

fn install_global_git_hook() -> Result<(), Box<dyn Error>> {
    let mut git_cmd = Command::new("git");
    let path = app_bin_dir_path();
    if !path.exists() {
        Err("Git Hook bin directory doesn't exist or is missing.")?
    }
    let args = vec!["config", "--global", "core.hooksPath", path.to_str().unwrap()];
    git_cmd.args(args);
    //TODO: Check status to see if failed and try again?
    git_cmd.output()?;
    Ok(())
}

fn manual_hook_install() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_input(log_format("Should this hook be installed globally? (Y|N)"))?);
    if should_apply_globally {
        install_global_git_hook()?;
    } else {
        loop {
            let repo_dir = get_input(log_format("What is an path to a git repo? (Enter q or nothing to quit.)"))?;
            let repo_dir = repo_dir.trim();
            if repo_dir.is_empty() || repo_dir == "q" {
                break;
            }
            let path = PathBuf::from_str(repo_dir)?;
            install_to_directory(path.to_str().unwrap())?;
        }
    }
    Ok(())
}

fn install_to_directory(path: &str) -> Result<(), Box<dyn Error>> {
    let mut path = is_git_directory(path)?;

    let cfg = Config::read_config(APP_HEADER)?;
    cfg.save_to_file()?;

    path.push(".git");
    path.push("hooks");

    let hook_name = os_specific_binary_name(HOOK_BIN_NAME);
    path.push(&hook_name);

    let mut hook_path = app_hooks_dir_path();
    hook_path.push(&hook_name);

    if !hook_path.exists() {
        move_executables()?;
    }

    std::fs::copy(&hook_path, &path)?;

    println_log(format!("Installed {} to {}", hook_path.to_str().unwrap(), path.to_str().unwrap()));

    Ok(())
}

fn full_install() -> Result<(), Box<dyn Error>> {

    create_if_not_exists(app_dir_path())?;

    let cfg = match Config::read_existing_config() {
        Ok(cfg) => {
            match check_y_n(get_input(log_format("Config already exists would you like to override (y|n)"))?) {
                true => Config::read_input(APP_HEADER)?,
                false => cfg
            }
        },
        Err(_) => Config::read_input(APP_HEADER)?
    };

    cfg.save_to_file()?;

    move_executables()?;

    manual_hook_install()?;

    Ok(())
}

fn uninstall_app() {
    match uninstall_app_dir() {
        Err(e) => {
            println_log(e.to_string());
        }
        _ => {
            println_log("Uninstalled Git Hook");
        }
    }
    match uninstall_global_git_hook() {
        Err(e) => {
            println_log(e.to_string());
        }
        _ => {
            println_log("Uninstalled Global Git Hook");
        }
    }
}

fn uninstall_app_dir() -> Result<(), Box<dyn Error>> {
    let app_dir = app_dir_path();
    if app_dir == dirs::home_dir().unwrap() {
        Err("Can't uninstall home directory")?;
    }
    if !app_dir.ends_with(APP_DIR_NAME) {
        Err("Can't uninstall not app directory")?;
    }
    if app_dir.exists() {
        std::fs::remove_dir_all(app_dir)?;
    }
    Ok(())
}

fn uninstall_global_git_hook() -> Result<(), Box<dyn Error>> {
    let mut git_cmd = Command::new("git");
    let args = vec!["config", "--global", "--unset", "core.hooksPath"];
    git_cmd.args(args);
    git_cmd.output()?;
    Ok(())
}

fn main() {

    //TODO: -c Config to specify a config for a directory.
    let matches = App::new("Commit Collective Installer")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Source Allies")
        .about("Installs the Commit Collective git hook")
        .arg(Arg::with_name("DIR")
            .required(false)
            .index(1)
            .help("Directroy to install the git hook. If the hook isn't set up yet then it installs it before placing in directory"))
        .arg(Arg::with_name("u")
            .short("u")
            .long("uninstall")
            .help("Uninstalls the Commit Collective Hook"))
        .get_matches();

    if matches.is_present("u") {
        uninstall_app();
        return;
    }

    if matches.is_present("DIR") {
        soft_unwrap!(install_to_directory(matches.value_of("DIR").unwrap()));
    } else {
        soft_unwrap!(full_install());
    }
}
