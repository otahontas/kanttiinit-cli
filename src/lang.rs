use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(unix)]
use xdg::BaseDirectories;

#[cfg(windows)]
use directories::ProjectDirs;

#[cfg(windows)]
use std::fs;

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

#[cfg(unix)]
fn get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    Ok(BaseDirectories::with_prefix(CONFIG_FOLDER_PREFIX)
        .context("Could not get base directories")?
        .get_config_file(CONFIG_FILE_NAME))
}

#[cfg(windows)]
fn get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    let proj_dirs = ProjectDirs::from("", "", CONFIG_FOLDER_PREFIX)
        .context("Could not determine project directories")?;
    Ok(proj_dirs.config_dir().join(CONFIG_FILE_NAME))
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

#[cfg(unix)]
fn create_config_directories_and_get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    BaseDirectories::with_prefix(CONFIG_FOLDER_PREFIX)
        .context("Could not get base directories")?
        .place_config_file(CONFIG_FILE_NAME)
        .context("Could not place config file")
}

#[cfg(windows)]
fn create_config_directories_and_get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    let proj_dirs = ProjectDirs::from("", "", CONFIG_FOLDER_PREFIX)
        .context("Could not determine project directories")?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir).context("Could not create config directory")?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_from_str_valid() {
        assert!(matches!(Lang::from_str("fi"), Ok(Lang::Fi)));
        assert!(matches!(Lang::from_str("en"), Ok(Lang::En)));
    }

    #[test]
    fn test_lang_from_str_invalid() {
        assert!(Lang::from_str("invalid").is_err());
        assert!(Lang::from_str("").is_err());
        assert!(Lang::from_str("fr").is_err());
    }

    #[test]
    fn test_lang_display() {
        assert_eq!(Lang::Fi.to_string(), "fi");
        assert_eq!(Lang::En.to_string(), "en");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            lang: "en".to_string(),
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("lang"));
        assert!(toml_str.contains("en"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = "lang = \"fi\"";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.lang, "fi");
    }
}
