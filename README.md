# SA Git Hook

SA Git Hook is a [post commit hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) that sends commit stats to
lambda that is used to populate a commit stats leaderboard.

## Disclaimer: __Do not use this hook without explicit client approval__

This hook will send traffic to a url upon each local commit. Security analysis may find that this git metadata could be used to identify repos, partners, file types, and languages. When consulting, __do not use this hook on client computers nor client repos__ without having a conversation and receiving explicit permission.

## Links

[Dashboard](http://git-metrics.sandbox-dev.sourceallies.com)

## Collected Data

### Config

- Username
- Team name

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
  "extension": [
    ".rs",
    ".yaml"
  ]
}
```

## Recommended Install

### Steps
1. Grab the binaries for your platform under [Releases](https://github.com/sourceallies/SA-Git-Hook/releases/) zip file
1. Go into the platform directory (Windows or macOS) by `cd` or `dir` into those directories.
1. Run the __install__ binary (__.exe__ for Windows)

## Manual Install

### Requirements

- Installed [Rust](https://www.rust-lang.org/tools/install)
- (Windows only) Install Git Bash
  - cargo must be in your path

### Steps

__Windows users:__ Do the following in a Git Bash terminal

1. Clone this repo
1. In the _Hookstaller_ directory, run the __install.sh__ script

    ```bash
    cd SA-Git-Hook/Hookstaller
    ./install.sh
    ```

1. Follow the steps given in the prompt

#### Manual install for a specific repo

Copy the post-commit executable (`/Hookstaller/target/release/post-commit`) to the `.git/hooks` directory of a given repo.

  ```bash
  cp ./Hookstaller/target/release/post-commit $(YOUR_REPO_NAME)/.git/hooks)
  ```

## Uninstall

### If installed globally

Run `git config --global --unset core.hooksPath`

### Per repository

Remove the `post-commit` executable in the `.git/hooks/` directory in the repository you would like to remove the hook
from.

## Authors

### Active

- [@SourceBolt](https://github.com/SourceBolt)
- [@grey-lovelace](https://github.com/grey-lovelace)

### Reserve

- [@AndreasChristianson](https://github.com/AndreasChristianson)
- [@JDawson98](https://github.com/JDawson98)
- [@GeneTinderholm](https://github.com/GeneTinderholm)
