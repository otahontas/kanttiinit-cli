use clap::Parser;
use clap::builder::PossibleValuesParser;
use crate::lang::AVAILABLE_LANGS;

const AFTER_HELP: &str = "Get all restaurants in a specific area:
kanttiinit -q otaniemi

Get restaurants by restaurant name:
kanttiinit -q unicafe

Only list courses that match a certain keyword:
kanttiinit -q töölö -f salad

See menus for tomorrow:
kanttiinit -q alvari -d 1";

#[derive(Parser, Debug)]
#[command(
    about,
    version,
    long_about = None,
    arg_required_else_help = true,
    disable_version_flag = true, // replace with custom setup that allows both -v and -V
    after_help = AFTER_HELP
)]
pub struct Args {
    /// Search restaurants by restaurant or area name (e.g. otaniemi, kumpula, kaivopiha)
    #[arg(short, long)]
    pub query: Option<String>,

    /// Specify day (0=today, 1=tomorrow, -1=yesterday, etc.)
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    pub day: i32,

    /// Filter courses by keyword (case insensitive)
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Show first n restaurants
    #[arg(short = 'n', long)]
    pub head: Option<u16>,

    /// Print version
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (), // handled automatically, no need for pub here

    /// Show restaurant address in the output
    #[arg(short, long)]
    pub address: bool,

    /// Show restaurant URL in the output
    #[arg(short, long)]
    pub url: bool,

    /// Hide closed restaurants when searching for todays menus
    #[arg(long = "hide-closed")]
    pub hide_closed: bool,

    /// Hide restaurants without menu when searching for todays menus
    #[arg(long = "hide-no-menu")]
    pub hide_no_menu: bool,

    /// Save the preferred language
    #[arg(long = "set-lang", value_parser = PossibleValuesParser::new(AVAILABLE_LANGS))]
    pub set_lang: Option<String>,
}

pub fn parse<I, T>(args: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::parse_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    fn make_args(args: &[&str]) -> Vec<OsString> {
        args.iter().map(OsString::from).collect()
    }

    #[test]
    fn test_args_parsing_with_query() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi"]);
        let parsed = parse(args);
        assert_eq!(parsed.query, Some("otaniemi".to_string()));
    }

    #[test]
    fn test_args_parsing_with_day_offset() {
        let args = make_args(&["kanttiinit", "-q", "töölö", "-d", "1"]);
        let parsed = parse(args);
        assert_eq!(parsed.day, 1);
    }

    #[test]
    fn test_args_parsing_with_negative_day_offset() {
        let args = make_args(&["kanttiinit", "-q", "töölö", "-d", "-1"]);
        let parsed = parse(args);
        assert_eq!(parsed.day, -1);
    }

    #[test]
    fn test_args_parsing_with_filter() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "-f", "salad"]);
        let parsed = parse(args);
        assert_eq!(parsed.filter, Some("salad".to_string()));
    }

    #[test]
    fn test_args_parsing_with_head() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "-n", "5"]);
        let parsed = parse(args);
        assert_eq!(parsed.head, Some(5));
    }

    #[test]
    fn test_args_parsing_with_flags() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "-a", "-u", "--hide-closed"]);
        let parsed = parse(args);
        assert!(parsed.address);
        assert!(parsed.url);
        assert!(parsed.hide_closed);
    }

    #[test]
    fn test_args_parsing_with_hide_no_menu() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "--hide-no-menu"]);
        let parsed = parse(args);
        assert!(parsed.hide_no_menu);
    }

    #[test]
    fn test_args_parsing_set_lang() {
        let args = make_args(&["kanttiinit", "--set-lang", "fi"]);
        let parsed = parse(args);
        assert_eq!(parsed.set_lang, Some("fi".to_string()));
    }

    #[test]
    fn test_args_default_values() {
        let args = make_args(&["kanttiinit", "-q", "test"]);
        let parsed = parse(args);
        assert_eq!(parsed.day, 0);
        assert_eq!(parsed.filter, None);
        assert_eq!(parsed.head, None);
        assert!(!parsed.address);
        assert!(!parsed.url);
        assert!(!parsed.hide_closed);
        assert!(!parsed.hide_no_menu);
        assert_eq!(parsed.set_lang, None);
    }
}
