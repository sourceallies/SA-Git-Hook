mod util;

use std::process::Command;
use std::error::Error;

use std::str::{FromStr, SplitWhitespace};

use std::fmt::{Debug, Display, Formatter};
use std::{thread, env};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use crate::ResponseState::{Running, Success, Failed};
use crate::util::Config;
use std::collections::HashSet;
use std::path::PathBuf;


#[derive(Clone)]
struct DiffStats {
    files_changed: u32,
    deletions: u32,
    insertions: u32,
    extensions: HashSet<String>,
}

impl DiffStats {

    fn from_git_cmd() -> Result<DiffStats, Box<dyn Error>> {
        let mut git_cmd = Command::new("git");
        let args = vec!["diff", "--numstat", "HEAD^", "HEAD"];
        git_cmd.args(args);

        let output = git_cmd.output()?;
        if !output.status.success() {
            Err("Failed to Get Diff From Git")?;
        }
        let mut extensions = HashSet::new();
        let mut insertions = 0;
        let mut deletions = 0;
        let mut files_changed = 0;

        let output_str = String::from_utf8(output.stdout)?;
        for line in output_str.trim().lines() {
            let mut splits = line.trim().split_whitespace();
            insertions += u32::from_str(splits.next().unwrap())?;
            deletions += u32::from_str(splits.next().unwrap())?;
            let file_extension = DiffStats::get_file_extension(&mut splits);
            if file_extension.is_some() {
                extensions.insert(file_extension.unwrap());
            }
            files_changed += 1;
        }

        Ok(DiffStats {
            files_changed,
            deletions,
            insertions,
            extensions
        })
    }

    fn post_to_remote(self, config: Config) {
        let response_state = Arc::new(Mutex::new(Running));

        let inside_response_state = Arc::clone(&response_state);
        let timeout = config.timeout.clone();
        thread::spawn(move || {
            let client = reqwest::blocking::Client::new();

            println_log("Sent Diff Stats");
            let response = client.post(&config.endpoint).body(self.to_json(&config)).send();
            match response {
                Ok(r) => {
                    let status = r.status();
                    if status.is_success() {
                        *inside_response_state.lock().unwrap() = Success;
                    } else {
                        *inside_response_state.lock().unwrap() = Failed;
                    }
                }
                Err(e) => {
                    println_log(format!("{}", e));
                    *inside_response_state.lock().unwrap() = Failed;
                }
            }
        });
        let start_time = Instant::now();
        loop {
            if start_time.elapsed() > timeout {
                break;
            }
            let state = response_state.lock().unwrap();
            let is_running = matches!(*state, Running);
            drop(state);

            if is_running {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            break;
        }
    }

    fn get_file_extension(splits: &mut SplitWhitespace) -> Option<String> {
        let os = env::consts::OS;
        let file_name = splits.next().unwrap().rsplit(match os { "windows" => '\\', _ => '/' }).next().unwrap();
        if file_name.contains('.') {
            return Some(format!(".{}", file_name.rsplit('.').next().unwrap()));
        }
        return None
    }

    fn to_json(&self, config: &Config) -> String {
        format!("{{ {}, {}, {}, {}, {}, {} }}",
                generate_json_key_value_string("files_changed", self.files_changed),
                generate_json_key_value_string("insertions", self.insertions),
                generate_json_key_value_string("deletions", self.deletions),
                generate_json_key_value_string("extensions", self.extensions_to_string()),
                generate_json_key_value_string("team_name", value_string(&config.team_name)),
                generate_json_key_value_string("username", value_string(&config.username)),
        )
    }

    fn extensions_to_string(&self) -> String {
        let extensions = self.extensions.iter().map(|x| format!("\"{}\",", x)).collect::<String>();
        let mut chars = extensions.chars();
        chars.next_back();
        format!("[{}]", chars.as_str())
    }
}

impl Display for DiffStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "insertions: {}, deletions: {}, files_changed: {} extensions: {:?}", self.insertions, self.deletions, self.files_changed, self.extensions)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ResponseState {
    Running,
    Failed,
    Success,
}

fn generate_json_key_value_string<K: Display, V: Display>(key: K, value: V) -> String {
    format!("\"{}\": {}", key, value)
}

fn value_string<V: Display>(value: V) -> String {
    format!("\"{}\"", value)
}

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("[Git-Hook]: {}", output.as_ref())
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}

fn uninstall_hook() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from_str("./")?;

    println_log(std::fs::canonicalize(path).unwrap().to_str().unwrap());

    Ok(())
}

fn main() {
    let config = match Config::read_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println_log(format!("Invalid Config: {}", e));
            println_log("Uninstalling Hook");
            match uninstall_hook() {
                Err(e) => {
                    println_log(e.to_string());
                }
                Ok(_) => {
                    println_log("Uninstalled Hook to reinstall run the install again");
                }
            }
            return;
        }
    };

    let stats = match DiffStats::from_git_cmd() {
        Ok(x) => x,
        Err(e) => {
            println_log(e.to_string());
            return;
        }
    };
    stats.post_to_remote(config);
}
