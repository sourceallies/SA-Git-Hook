use std::error::Error;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::fs::File;

//TODO: Create timeout for config file that we can use as a duration for waiting to get success.
pub struct Config {
    pub team_name: String,
    pub source_allies_email: String
}

pub static CONFIG_FILE_NAME: &str =  "/.sai_git_hook_config";

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
    pub fn create_from_input() -> Result<Self, Box<dyn Error>> {
        let team_name = get_input("What is your Source Allies team name?")?;
        let source_allies_email = get_input("What is your Source Allies email?")?;

        Ok(Config {
            team_name,
            source_allies_email
        })
    }

    pub fn read_from_config() -> Result<Self, Box<dyn Error>> {
        let file = File::open(&Config::get_config_file_path())?;
        let mut reader = BufReader::new(file);

        Ok(Config {
            team_name: Config::get_value_from_config_reader(&mut reader)?,
            source_allies_email: Config::get_value_from_config_reader(&mut reader)?
        })
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&Config::get_config_file_path())?;

        let mut writer = BufWriter::new(file);

        let message = format!("team_name={}\nsource_allies_email={}\n", self.team_name, self.source_allies_email);
        print!("Config:\n{}", message);
        writer.write_all(message.as_bytes())?;

        Ok(())
    }

    fn get_config_file_path() -> String {
        dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/" + CONFIG_FILE_NAME
    }

    fn get_value_from_config_reader(reader: &mut BufReader<File>) -> Result<String, Box<dyn Error>> {
        let mut value = String::new();
        reader.read_line(&mut value)?;
        let value = value.trim().split('=').skip(1).next().unwrap().to_string();
        Ok(value)
    }
}