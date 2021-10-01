use std::process::Command;
use std::error::Error;
use std::str::{FromStr, Split};
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use crate::ResponseState::{Running, Success, Failed};

static TIMEOUT_DURATION: Duration = Duration::from_secs(3);
static ENDPOINT: &str = "https://hnxgs8zjjd.execute-api.us-east-1.amazonaws.com/test/stuffs";

#[derive(Copy, Clone)]
struct DiffStats {
    files_changed: u32,
    insertions: u32,
    deletions: u32,
    email: String
}

impl DiffStats {
    fn from_string(s: String) -> Result<DiffStats, Box<dyn Error>> {
        let mut diff_output = s.trim().split(',');

        let files_changed = get_number_from_diff(&mut diff_output)?;
        let insertions = get_number_from_diff(&mut diff_output)?;
        let deletions = get_number_from_diff(&mut diff_output)?;

        Ok(DiffStats { files_changed, insertions, deletions })
    }
    fn to_json(&self) -> String {
        format!("{{ \"files_changed\": {}, \"insertions\": {}, \"deletions\": {} }}", self.files_changed, self.insertions, self.deletions)
    }
}

impl Display for DiffStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "insertions: {}, deletions: {}, files_changed: {}", self.insertions, self.deletions, self.files_changed)
    }
}

fn get_number_from_diff(split: &mut Split<char>) -> Result<u32, ParseIntError> {
    u32::from_str(split.next()
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap())
}

fn run_git_cmd(cmd: &mut Command) -> Result<DiffStats, Box<dyn Error>> {
    let output = cmd.output()?;
    let output_str = String::from_utf8(output.stdout)?;

    DiffStats::from_string(output_str)
}

#[allow(dead_code)]
#[derive(Debug)]
enum ResponseState {
    Running,
    Failed,
    Success
}

fn post_to_remote(stats: &DiffStats) {
    let stats = stats.clone();
    let response_state = Arc::new(Mutex::new(Running));

    let inside_response_state = Arc::clone(&response_state);
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let response = client.post(ENDPOINT).body(stats.to_json()).send();
        match response {
            Ok(r) => {
                let status = r.status();
                println!("{}, {}", status.as_str(), r.text().unwrap());
                if status.is_success() {
                    *inside_response_state.lock().unwrap() = Success;
                } else {
                    *inside_response_state.lock().unwrap() = Failed;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                *inside_response_state.lock().unwrap() = Failed;
            }
        }
    });
    let start_time = Instant::now();
    loop {
        if start_time.elapsed() > TIMEOUT_DURATION {
            println!("POST to remote timed out");
            break;
        }
        let state = response_state.lock().unwrap();
        let is_running = matches!(*state, Running);
        let state_str = format!("{:?}",&state);
        drop(state);

        if is_running {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        println!("{:?}", state_str);
        break;
    }
}

fn main() {
    let mut git_cmd = Command::new("git");
    let args = vec!["diff", "--shortstat", "HEAD^", "HEAD"];
    git_cmd.args(args);

    match run_git_cmd(&mut git_cmd) {
        Err(e) => {
            println!("Error: {}", e);
        }
        Ok(stats) => {
            post_to_remote(&stats);
        }
    }
}
