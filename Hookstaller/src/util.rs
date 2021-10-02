use std::error::Error;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;

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