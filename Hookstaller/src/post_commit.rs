mod util;

use std::process::Command;
use std::error::Error;

use std::str::{FromStr};
use std::num::ParseIntError;
use std::fmt::{Debug, Display, Formatter};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use crate::ResponseState::{Running, Success, Failed};
use crate::util::Config;

//TODO: Make timeout in config file and also endpoint
static TIMEOUT_DURATION: Duration = Duration::from_secs(1);
static ENDPOINT: &str = "https://hnxgs8zjjd.execute-api.us-east-1.amazonaws.com/test/stuffs";

#[derive(Clone)]
struct DiffStats {
    files_changed: u32,
    insertions: u32,
    deletions: u32,
}

impl DiffStats {
    fn from_string(s: String) -> Result<DiffStats, Box<dyn Error>> {
        let diff_output = s.trim().split(',');
        let mut files_changed = 0;
        let mut  insertions = 0;
        let mut deletions = 0;
        for split in diff_output {
            check_log_output(&mut files_changed, split, "changed")?;
            check_log_output(&mut insertions, split, "insertion")?;
            check_log_output(&mut deletions, split, "deletion")?;
        }


        Ok(DiffStats { files_changed, insertions, deletions })
    }
}

impl Display for DiffStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "insertions: {}, deletions: {}, files_changed: {}", self.insertions, self.deletions, self.files_changed)
    }
}

fn check_log_output(variable: &mut u32, split: &str, key: &str) -> Result<(), ParseIntError> {
    if split.contains(key) {
        *variable = u32::from_str(split
            .trim()
            .split(' ')
            .next()
            .unwrap())?;
    }
    Ok(())
}

fn run_git_cmd(cmd: &mut Command) -> Result<DiffStats, Box<dyn Error>> {
    let output = cmd.output()?;
    if !output.status.success() {
        Err("Failed to Get Diff From Git")?;
    }
    let output_str = String::from_utf8(output.stdout)?;

    DiffStats::from_string(output_str)
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

fn stats_and_config_to_json(stats: &DiffStats, config: &Config) -> String {
    format!("{{ {}, {}, {}, {}, {} }}",
            generate_json_key_value_string("files_changed", stats.files_changed),
            generate_json_key_value_string("insertions", stats.insertions),
            generate_json_key_value_string("deletions", stats.deletions),
            generate_json_key_value_string("team_name", value_string(&config.team_name)),
            generate_json_key_value_string("email", value_string(&config.email)),
    )
}

fn post_to_remote(stats: DiffStats, config: Config) {
    let response_state = Arc::new(Mutex::new(Running));

    let inside_response_state = Arc::clone(&response_state);
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();

        println_log("Sent Diff Stats");
        let response = client.post(ENDPOINT).body(stats_and_config_to_json(&stats, &config)).send();
        match response {
            Ok(r) => {
                let status = r.status();
                if status.is_success() {
                    *inside_response_state.lock().unwrap() = Success;
                } else {
                    *inside_response_state.lock().unwrap() = Failed;
                }
            }
            Err(_) => {
                *inside_response_state.lock().unwrap() = Failed;
            }
        }
    });
    let start_time = Instant::now();
    loop {
        if start_time.elapsed() > TIMEOUT_DURATION {
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

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("[Git-Hook]: {}", output.as_ref())
}

fn println_error<S: AsRef<str>>(output: S) {
    println!("\\e[1;96;127m{}\\e[0m\n", log_format(output));
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}

fn main() {
    let config = match Config::read_from_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println_error(format!("Invalid Config: {}", e));
            return;
        }
    };

    let mut git_cmd = Command::new("git");
    let args = vec!["diff", "--shortstat", "HEAD^", "HEAD"];
    git_cmd.args(args);

    match run_git_cmd(&mut git_cmd) {
        Err(e) => {
            println_error(format!("Error: {}", e));
        }
        Ok(stats) => {
            post_to_remote(stats, config);
        }
    }
}
