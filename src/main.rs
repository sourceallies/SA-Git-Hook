
// git diff --shortstat HEAD^ HEAD

use std::process::Command;

fn main() {
    let args = vec!["diff", "--shortstat", "HEAD^", "HEAD"];
    let git_cmd = Command::new("git")
        .args(args)
        .output()
        .expect("Could not get diff");

    println!("Output: {}", String::from_utf8(git_cmd.stdout).expect("Couldn't Parse Utf8"));
}
