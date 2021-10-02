use std::error::Error;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use crate::util::ThreadState::{Running, Stopped};
use crossterm::event::{KeyEvent, Event};
use std::time::{Duration, Instant};
use std::thread;
use crossterm::event;
use tui::widgets::{Widget, Block};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::text::Text;
use tui::style::Style;

//TODO: Create timeout for config file that we can use as a duration for waiting to get success.
pub struct Config {
    pub team_name: String,
    pub username: String
}

pub static CONFIG_FILE_NAME: &str =  ".sai_git_hook_config";

#[allow(dead_code)]
pub fn get_input<S: AsRef<str>>(prompt: S) -> Result<String, Box<dyn Error>> {
    print!("{} ", prompt.as_ref());
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}

#[allow(dead_code)]
pub fn check_y_n<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().to_lowercase() == "y"
}

#[allow(dead_code)]
impl Config {
    pub fn read_from_config() -> Result<Self, Box<dyn Error>> {
        let file = File::open(&Config::config_file_path())?;
        let mut reader = BufReader::new(file);

        Ok(Config {
            team_name: Config::value_from_config_reader(&mut reader)?,
            username: Config::value_from_config_reader(&mut reader)?
        })
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&Config::config_file_path())?;

        let mut writer = BufWriter::new(file);

        let message = format!("team_name={}\nusername={}\n", self.team_name, self.username);
        writer.write_all(message.as_bytes())?;

        Ok(())
    }
    fn config_file_path() -> PathBuf {
        let mut config_file_path = dirs::home_dir().unwrap();
        config_file_path.push(CONFIG_FILE_NAME);
        config_file_path
    }

    fn value_from_config_reader(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut value = String::new();
        reader.read_line(&mut value)?;
        let value = value.trim().split('=').skip(1).next().unwrap().to_string();
        Ok(value)
    }
}

pub enum ThreadState {
    Running,
    Stopped,
    Crashed(Box<dyn Error + Send + 'static>)
}

pub struct ThreadStream<T> {
    state: Arc<Mutex<ThreadState>>,
    pub stream: Option<Receiver<T>>
}

impl<T> ThreadStream<T> {
    pub fn channel() -> (Sender<T>, Self) {
        let (sender, receiver) = mpsc::channel();
        (sender, ThreadStream {
            state: Arc::new(Mutex::new(Running)),
            stream: Some(receiver)
        })
    }

    pub fn is_stopped(&self) -> bool {
        !matches!(*self.state.lock().unwrap(), Running)
    }

    pub fn stop(self) {
        *self.state.lock().unwrap() = Stopped;
    }
}

impl<T> Clone for ThreadStream<T> {
    fn clone(&self) -> Self {
        //Can only have 1 Receiver
        ThreadStream {
            state: Arc::clone(&self.state),
            stream: None
        }
    }
}

pub struct SplashScreen<'a> {
    block: Option<Block<'a>>,
    style: Style,
    text: Text<'a>
}

impl<'a> SplashScreen<'a> {
    pub fn new<T: Into<Text<'a>>>(t: T) -> Self{
        SplashScreen {
            block: None,
            style: Style::default(),
            text: t.into()
        }
    }

    pub fn block(mut self, b: Block<'a>) -> Self {
        self.block = Some(b);
        self
    }
}

impl<'a> Widget for SplashScreen<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if text_area.height < 1 {
            return;
        }

        //Draw the text to the text_area

    }
}

pub fn setup_crossterm_input() -> ThreadStream<KeyEvent> {
    let (input_sender, thread_stream) = ThreadStream::channel();
    let tick_rate = Duration::from_millis(250);

    let inside_thread_stream = thread_stream.clone();

    thread::spawn(move || {
        let last_tick = Instant::now();
        loop {
            if inside_thread_stream.is_stopped() {
                break;
            }
            let timeout = tick_rate.checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                let read = event::read();
                if read.is_err() {
                    continue;
                }
                if let Event::Key(key) = read.unwrap() {
                    input_sender.send(key).unwrap();
                }
            }
        }
    });
    thread_stream
}