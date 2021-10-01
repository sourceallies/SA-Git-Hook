# SA Git Hook

## Install

### Pre-requisite

- Installed [Rust](https://www.rust-lang.org/tools/install)
- (Windows only) Install Git Bash
    - cargo must be in your path

### Steps

1. Clone this repo
2. In the _Hookstaller_ directory, run the __install.sh__ script
3. Follow the steps given in the prompt

#### Manual install for a specific repo

Copy the post-commit executable (<path-to-this-dir>/Hookstaller/target/release/post-commit) into the `.git/hooks`
directory of a given repo.
