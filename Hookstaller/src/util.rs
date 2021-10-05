use std::error::Error;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use std::str::FromStr;

static DEFAULT_TIMEOUT_DURATION: Duration = Duration::from_secs(3);
static DEFAULT_ENDPOINT: &str = "https://hnxgs8zjjd.execute-api.us-east-1.amazonaws.com/test/stuffs";

// macro_rules! param_read_write {
//     (struct $name:ident {$($fname:ident : $ftype:ty),* }) => {
//         struct $name {
//             $($fname : $ftype),*
//         }
//         impl $name {
//             fn read_from_file<P: Path>(p: P) -> Result<$name, Box<dyn Error>> {
//                 let file = File::open(p)?;
//                 let mut reader = BufReader::new(file);
//             }
//         }
//     }
// }


pub struct Config {
    pub team_name: String,
    pub username: String,
    pub timeout: Duration,
    pub endpoint: String
}

pub static CONFIG_DIR_NAME: &str =  ".sai_git_hook_config";

//TODO: Maybe make T: FromStr typed
#[allow(dead_code)]
pub fn get_input<S: AsRef<str>>(prompt: S) -> Result<String, Box<dyn Error>> {
    print!("{} ", prompt.as_ref());
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}

pub fn get_required_input<S: AsRef<str>>(prompt: S) -> Result<String, Box<dyn Error>> {
    let mut input = get_input(prompt.as_ref())?;
    while input.is_empty() {
        input = get_input(prompt.as_ref())?;
    }
    Ok(input)
}

pub fn get_optional_input<S: AsRef<str>>(prompt: S) -> Result<Option<String>, Box<dyn Error>> {
    let input = get_input(prompt.as_ref())?;
    if input.is_empty() {
        return Ok(None);
    }
    Ok(Some(input))
}

#[allow(dead_code)]
pub fn check_y_n<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().to_lowercase() == "y"
}

impl Default for Config {
    fn default() -> Self {
        Config {
            username: String::new(),
            team_name: String::new(),
            timeout: DEFAULT_TIMEOUT_DURATION,
            endpoint: DEFAULT_ENDPOINT.to_string()
        }
    }
}

#[allow(dead_code)]
impl Config {

    pub fn read_input<S: AsRef<str>>(header: S) -> Result<Config, Box<dyn Error>> {
        let mut config = Config::default();
        config.team_name = get_required_input(format!("{}: What is your Team Name?", header.as_ref()))?;
        config.username = get_required_input(format!("{}: What is your Username?", header.as_ref()))?;
        let timeout = get_optional_input(format!("{}: Desired Timeout in milliseconds (Default {} ms)? ", header.as_ref(), config.timeout.as_millis()))?;
        if timeout.is_some() {
            config.timeout = Duration::from_millis(u64::from_str(&timeout.unwrap())?);
        }

        Ok(config)
    }

    pub fn delete_config(self) -> Result<(), Box<dyn Error>> {
        let config_dir_path = Config::config_dir_path();
        if !config_dir_path.exists() {
            Err("Config doesn't exist")?;
        }

        std::fs::remove_dir_all(config_dir_path)?;
        Ok(())
    }

    pub fn read_config() -> Result<Self, Box<dyn Error>> {
        let mut config_dir_path = Config::config_dir_path();
        if !config_dir_path.exists() {
            Err("Git Hook Config Folder doesn't exist")?;
        }
        config_dir_path.push("config");
        let file = File::open(config_dir_path.as_path())?;
        let mut reader = BufReader::new(file);
        let team_name = Config::value_from_config_reader(&mut reader)?;
        let username = Config::value_from_config_reader(&mut reader)?;
        let timeout = Config::value_from_config_reader(&mut reader)?;
        let timeout = Duration::from_millis(u64::from_str(&timeout)?);
        let endpoint = Config::value_from_config_reader(&mut reader)?;
        Ok(Config {
            team_name,
            username,
            timeout,
            endpoint
        })
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut config_dir_path = Config::config_dir_path();
        if !config_dir_path.exists() {
            std::fs::create_dir(&config_dir_path)?;
        }
        config_dir_path.push("config");
        let file = File::create(config_dir_path.as_path())?;

        let mut writer = BufWriter::new(file);

        let message = format!("team_name={}\nusername={}\ntimeout_milliseconds={}\nendpoint={}\n", self.team_name, self.username, self.timeout.as_millis(), self.endpoint);
        writer.write_all(message.as_bytes())?;

        Ok(())
    }

    pub fn config_dir_path() -> PathBuf {
        let mut config_file_path = dirs::home_dir().unwrap();
        config_file_path.push(CONFIG_DIR_NAME);
        config_file_path
    }

    fn value_from_config_reader(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut value = String::new();
        reader.read_line(&mut value)?;
        let value = value.trim().split('=').skip(1).next().unwrap().to_string();
        Ok(value)
    }
}
