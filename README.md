# SA Git Hook

SA Git Hook is a [post commit hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) that sends commit stats to
lambda that is used to populate a commit stats leaderboard.

## Collected Data

### Config

- Source Allies Email
- Source Allies Team name

### Commit Stats

- Number of insertions from a commit
- Number of deletions from a commit
- Number of files changed
- Language of the files changed

Example payload

```json
{
  "username": "test username",
  "team_name": "team name",
  "insertions": 13,
  "deletions": 28,
  "files_changed": 3,
  "extension": [".rs", ".yaml"]
}
```

## Install

### Requirements

- Installed [Rust](https://www.rust-lang.org/tools/install)
- (Windows only) Install Git Bash
    - cargo must be in your path

### Steps

__Windows users:__ Do the following in a Git Bash terminal

1. Clone this repo
2. In the _Hookstaller_ directory, run the __install.sh__ script
    1. `./install.sh`
3. Follow the steps given in the prompt

#### Manual install for a specific repo

Copy the post-commit executable (<path-to-this-dir>/Hookstaller/target/release/post-commit) into the `.git/hooks`
directory of a given repo.
