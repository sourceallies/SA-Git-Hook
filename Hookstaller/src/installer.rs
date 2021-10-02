mod util;
use std::error::Error;
use std::process::Command;
use std::path::{PathBuf};
use std::{fs, thread};
use std::env;
use std::str::FromStr;
use crate::util::{check_y_n, get_input, Config, ThreadStream};
use crate::util::setup_crossterm_input;
use crossterm::event::{KeyEvent, Event, KeyCode};
use std::time::{Duration, Instant};
use crossterm::event;
use std::sync::mpsc;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::stdout;
use tui::backend::{CrosstermBackend, Backend};
use tui::{Terminal, Frame};
use std::sync::mpsc::RecvTimeoutError;
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::widgets::{Block, Borders, Paragraph, Wrap, Sparkline, Row, Table, Cell};
use tui::text::{Span, Spans, Text};
use tui::style::{Style, Modifier, Color};

struct InstallerApp {

}

impl InstallerApp {

    fn new() -> Self {
        InstallerApp{}
    }

    fn tick(&mut self) {

    }

    fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let block = Block::default()
            .borders(Borders::all())
            .title("SAI Git Hook Installer")
            .title_alignment(Alignment::Center);
        let mut text = Vec::new();
        let bold_style = Style::default()
            .add_modifier(Modifier::BOLD.union(Modifier::UNDERLINED))
            .fg(Color::White);
        text.push(Span::styled("Welcome to SA Git Hook Installer", bold_style));
        let height = (frame.size().height as i32 - 2) as f32 * 0.25 ;
        let welcome_screen = Table::new(vec![
            Row::default()
                .bottom_margin(height as u16),
            Row::new(vec![Spans::from(text)])
        ]).block(block)
            .column_spacing(10)
            .widths([Constraint::Percentage(50), ].as_ref());

        frame.render_widget(welcome_screen, frame.size());
    }

    fn action(&mut self, code: KeyCode ) -> bool {
        match code {
            KeyCode::Char('q') => {
                return false;
            }
            _ => {}
        }
        return true;
    }

    fn stop(self) {

    }
}

fn install_git_hook() -> Result<(), Box<dyn Error>> {
    let should_apply_globally = check_y_n(get_installer_input("Should this hook be installed globally? (Y|N)")?);

    if should_apply_globally {
        let mut exec_path = std::env::current_dir()?;
        exec_path.push("target");
        exec_path.push("release");
        let mut git_cmd = Command::new("git");
        let args = vec!["config", "--global", "core.hooksPath", exec_path.to_str().unwrap()];
        git_cmd.args(args);
        git_cmd.output()?;
    } else {
        let os = env::consts::OS;
        let post_commit_file = match os { "windows" => "post-commit.exe", _ => "post-commit" };
        let mut post_commit_executable_path = PathBuf::new();
        post_commit_executable_path.push("target");
        post_commit_executable_path.push("release");
        post_commit_executable_path.push(post_commit_file);
        loop {
            let repo_dir = get_installer_input("What is an absolute path to a git repo? (Press q and enter to quit.)")?;
            if repo_dir == "q" {
                break;
            }
            if repo_dir == "" {
                continue;
            }

            // Check repo_directory for .git/hooks folder
            let mut git_hooks_path = PathBuf::new();
            git_hooks_path.push(".git");
            git_hooks_path.push("hooks");

            let mut hooks_dir = PathBuf::from_str(&repo_dir)?;
            hooks_dir.push(git_hooks_path);

            if hooks_dir.exists() {
                hooks_dir.push(post_commit_file);
                println_log(format!("Installing from {} to {}", post_commit_executable_path.to_str().unwrap(), hooks_dir.to_str().unwrap()));
                fs::copy(post_commit_executable_path.as_path(), hooks_dir)?;
            } else {
                println_error("Given directory is not a git repository.");
            }
        }

    }
    Ok(())
}

fn get_installer_input<S: AsRef<str>>(output: S) -> Result<String, Box<dyn Error>> {
    get_input(log_format(output))
}

fn log_format<S: AsRef<str>>(output: S) -> String {
    format!("[Git-Hook-Installer]: {}", output.as_ref())
}

fn println_error<S: AsRef<str>>(output: S) {
    println!("\\e[1;96;127m{}\\e[0m\n", log_format(output));
}

fn println_log<S: AsRef<str>>(output: S) {
    println!("{}", log_format(output));
}

fn create_from_input() -> Result<Config, Box<dyn Error>> {
    let team_name = get_installer_input("What is your Team Name?")?;
    let username = get_installer_input("What is your Username?")?;

    Ok(Config {
        team_name,
        username
    })
}

fn main() -> Result<(), Box<dyn Error>> {

    enable_raw_mode()?;
    let stdout = stdout();

    let mut app = InstallerApp::new();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let input_handler = setup_crossterm_input();

    terminal.clear()?;
    loop {
        app.tick();
        terminal.draw(|f| {
            app.draw(f);
        })?;
        let key_event = match input_handler.stream.as_ref().unwrap().recv_timeout(Duration::from_millis(50)) {
            Ok(x) => x,
            Err(e) => {
                if matches!(e, RecvTimeoutError::Disconnected){
                    break;
                }
                continue;
            }
        };
        if !app.action(key_event.code) {
            break;
        }
    }

    disable_raw_mode()?;
    app.stop();
    input_handler.stop();
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())

    // let cfg = match create_from_input() {
    //     Ok(cfg) => cfg,
    //     Err(e) => {
    //         println_error(e.to_string());
    //         return;
    //     }
    // };
    // match cfg.save_to_file() {
    //     Err(e) => {
    //         println_error(e.to_string());
    //     }
    //     _ => {}
    // }
    // match install_git_hook() {
    //     Err(e) => {
    //         println_error(e.to_string());
    //     }
    //     _ => {}
    // }
}
