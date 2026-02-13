use anyhow::Context;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

// Single source of truth for available languages
pub const AVAILABLE_LANGS: &[&str] = &["fi", "en"];

#[derive(Debug)]
pub enum Lang {
    Fi,
    En,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fi" => Ok(Lang::Fi),
            "en" => Ok(Lang::En),
            _ => Err(anyhow::anyhow!(
                "Invalid language value: {}. Available languages: {}",
                s,
                AVAILABLE_LANGS.join(", ")
            )),
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

const CONFIG_FOLDER_PREFIX: &str = env!("CARGO_PKG_NAME");
const CONFIG_FILE_NAME: &str = "config.toml";
const DEFAULT_LANG: &str = "en";

/// Parse `lang = "value"` from config file contents.
fn parse_lang_from_config(contents: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("lang") {
            let rest = rest.trim_start();
            if let Some(rest) = rest.strip_prefix('=') {
                let rest = rest.trim();
                if let Some(rest) = rest.strip_prefix('"') {
                    if let Some(value) = rest.strip_suffix('"') {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn get_config_dir() -> Result<PathBuf, anyhow::Error> {
    #[cfg(unix)]
    {
        let base = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME environment variable not set");
            format!("{}/.config", home)
        });
        Ok(PathBuf::from(base).join(CONFIG_FOLDER_PREFIX))
    }
    #[cfg(windows)]
    {
        let appdata = std::env::var("APPDATA").context("APPDATA environment variable not set")?;
        Ok(PathBuf::from(appdata).join(CONFIG_FOLDER_PREFIX))
    }
}

fn get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    Ok(get_config_dir()?.join(CONFIG_FILE_NAME))
}

fn get_lang_from_config_file() -> Result<String, anyhow::Error> {
    let config_path = get_config_file_path().context("Could not get config file path")?;
    if config_path.exists() {
        let contents =
            std::fs::read_to_string(config_path).context("Could not read config file")?;
        Ok(parse_lang_from_config(&contents).unwrap_or_else(|| DEFAULT_LANG.to_string()))
    } else {
        Ok(DEFAULT_LANG.to_string())
    }
}

fn create_config_directories_and_get_config_file_path() -> Result<PathBuf, anyhow::Error> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(&config_dir).context("Could not create config directory")?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
}

pub fn get_lang() -> Result<String, anyhow::Error> {
    let lang_str = get_lang_from_config_file().context("Could not get config")?;
    Ok(Lang::from_str(&lang_str)
        .context("Could not parse language value")?
        .to_string())
}

pub fn set_lang(lang_from_user: &str) -> Result<(), anyhow::Error> {
    let lang = Lang::from_str(lang_from_user)?;
    let config_contents = format!("lang = \"{}\"\n", lang);
    let config_path = create_config_directories_and_get_config_file_path()
        .context("Could not get config file")?;
    let mut config_file = File::create(config_path).context("Could not create config file")?;
    write!(config_file, "{}", config_contents).context("Could not write to config file")?;
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
    fn test_parse_lang_from_config_valid() {
        assert_eq!(
            parse_lang_from_config("lang = \"fi\"\n"),
            Some("fi".to_string())
        );
        assert_eq!(
            parse_lang_from_config("lang = \"en\"\n"),
            Some("en".to_string())
        );
    }

    #[test]
    fn test_parse_lang_from_config_with_whitespace() {
        assert_eq!(
            parse_lang_from_config("  lang  =  \"fi\"  \n"),
            Some("fi".to_string())
        );
    }

    #[test]
    fn test_parse_lang_from_config_missing() {
        assert_eq!(parse_lang_from_config(""), None);
        assert_eq!(parse_lang_from_config("other = \"value\""), None);
    }

    #[test]
    fn test_parse_lang_from_config_malformed() {
        assert_eq!(parse_lang_from_config("lang = fi"), None);
        assert_eq!(parse_lang_from_config("lang fi"), None);
    }

    #[test]
    fn test_available_langs_const() {
        assert_eq!(AVAILABLE_LANGS, &["fi", "en"]);
    }

    #[test]
    fn test_lang_from_str_invalid_shows_available() {
        let result = Lang::from_str("fr");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Available languages"));
        assert!(err.to_string().contains("fi, en"));
    }
}
