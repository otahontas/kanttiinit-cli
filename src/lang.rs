use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug)]
pub enum Lang {
    Fi,
    En,
}

impl Lang {
    fn from_str(s: &str) -> Result<Lang, anyhow::Error> {
        match s {
            "fi" => Ok(Lang::Fi),
            "en" => Ok(Lang::En),
            _ => Err(anyhow::anyhow!("Invalid language value: {}", s)),
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::Fi => write!(f, "fi"),
            Lang::En => write!(f, "en"),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    lang: String,
}

const CONFIG_FOLDER_PREFIX: &str = env!("CARGO_PKG_NAME");
const CONFIG_FILE_NAME: &str = "config.toml";

fn get_config_file_path() -> Result<PathBuf, xdg::BaseDirectoriesError> {
    Ok(BaseDirectories::with_prefix(CONFIG_FOLDER_PREFIX)?.get_config_file(CONFIG_FILE_NAME))
}

fn get_config_from_file_or_return_default_config() -> Result<Config, anyhow::Error> {
    let config_path = get_config_file_path().context("Could not get config file path")?;
    match config_path.exists() {
        true => {
            let read_to_string =
                std::fs::read_to_string(config_path).context("Could not read config file")?;
            Ok(toml::from_str(&read_to_string).context("Could not parse config file as TOML")?)
        }
        false => Ok(Config {
            lang: "en".to_string(),
        }),
    }
}

fn create_config_directories_and_get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
        .context("Could not get base directories from system")?
        .place_config_file(CONFIG_FILE_NAME)
        .context("Could not place config file")
}

pub fn get_lang() -> Result<String, anyhow::Error> {
    Ok(Lang::from_str(
        &get_config_from_file_or_return_default_config()
            .context("Could not get config")?
            .lang,
    )
    .context("Could not parse language value")?
    .to_string())
}

pub fn set_lang(lang_from_user: &str) -> Result<(), anyhow::Error> {
    let lang = Lang::from_str(lang_from_user)?;
    let config = Config {
        lang: lang.to_string(),
    };
    let toml = toml::to_string(&config).context("Could not serialize config to TOML")?;
    let config_path = create_config_directories_and_get_config_file_path()
        .context("Could not get config file")?;
    let mut config_file = File::create(config_path).context("Could not create config file")?;
    write!(config_file, "{}", toml).context("Could not write to config file")?;
    Ok(())
}
