
// $ setup-sai-hooks

use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

static CONFIG_FILE_NAME: &str = ".sai_git_hook_config";

struct Config {
    team_name: String,
    source_allies_email: String,
    should_apply_globally: bool
}

fn get_input<S: AsRef<str>>(prompt: S) -> Result<String, Box<dyn Error>> {
    print!("{} ", prompt.as_ref());
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}

fn check_y_n<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().to_lowercase() == "y"
}

impl Config {
    fn create_from_input() -> Result<Config, Box<dyn Error>> {
        let team_name = get_input("What is your Source Allies team name?")?;
        let source_allies_email = get_input("What is your Source Allies email?")?;
        let should_apply_globally = check_y_n(get_input("Should this hook run globally? (Y|N)")?);


        Ok(Config {
            team_name,
            source_allies_email,
            should_apply_globally
        })
    }

    fn save_to_file<S: AsRef<str>>(&self, file_name: S) -> Result<(), Box<dyn Error>> {
        let file = File::create(file_name.as_ref())?;

        let mut writer = BufWriter::new(file);

        let message = format!("team_name={}\nsource_allies_email={}\nshould_apply_globally={}\n", self.team_name, self.source_allies_email, self.should_apply_globally);
        print!("config {}", message);
        writer.write_all(message.as_bytes())?;

        Ok(())
    }
}

fn main() {
    let cfg = match Config::create_from_input() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    let file_loc = home_dir + "/" + CONFIG_FILE_NAME;
    // println!("{}", file_loc);
    match cfg.save_to_file(file_loc) {
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
}
