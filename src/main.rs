use std::process::Command;
use std::error::Error;
use std::str::{FromStr, Split};
use std::num::ParseIntError;

fn get_number_from_diff(split: &mut Split<char>) -> Result<u32, ParseIntError> {
    u32::from_str(split.next()
        .unwrap()
        .trim()
        .split(' ')
        .next()
        .unwrap())
}

fn run_git_cmd(cmd: &mut Command) -> Result<(), Box<dyn Error>> {
    let output = cmd.output()?;
    let output_str = String::from_utf8(output.stdout)?;
    let mut diff_output = output_str.trim().split(',');

    let files_changed = get_number_from_diff(&mut diff_output)?;
    let insertions = get_number_from_diff(&mut diff_output)?;
    let deletions = get_number_from_diff(&mut diff_output)?;
    println!("files changed: {}, insertions: {}, deletions: {}", files_changed, insertions, deletions);

    Ok(())
}

fn main() {
    let mut git_cmd = Command::new("git");
    let args = vec!["diff", "--shortstat", "HEAD^", "HEAD"];
    git_cmd.args(args);

    match run_git_cmd(&mut git_cmd) {
        Err(e) => {
            println!("Error: {}", e);
        }
        _ => {}
    }
}
