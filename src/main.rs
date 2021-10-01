use std::process::Command;
use std::error::Error;
use std::str::{FromStr, Split};
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};

struct DiffStats {
    files_changed: u32,
    insertions: u32,
    deletions: u32,
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

fn main() {
    let mut git_cmd = Command::new("git");
    let args = vec!["diff", "--shortstat", "HEAD^", "HEAD"];
    git_cmd.args(args);

    match run_git_cmd(&mut git_cmd) {
        Err(e) => {
            println!("Error: {}", e);
        }
        Ok(stats) => {
            println!("stats: {}", stats.to_json());
        }
    }
}
