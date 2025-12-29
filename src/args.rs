use clap::Parser;

/// Kanttiinit.fi command-line interface
#[derive(Parser, Debug)]
#[command(
    about,
    version,
    long_about = None,
    arg_required_else_help = true,
    // Automatic help flag is disabled, since short -h flag is reserved
    // for hiding closed restaurants
    disable_help_flag = true,
    // Automatic version flag is disabled, since short -v flag needs
    // to be generated in Args struct
    disable_version_flag = true,
    // TODO: read this from variable / other crate?
    after_help = "Get all restaurants in a specific area:
kanttiinit -q otaniemi

Get restaurants by restaurant name:
kanttiinit -q unicafe

Only list courses that match a certain keyword:
kanttiinit -q töölö -f salad

See menus for tomorrow:
kanttiinit -q alvari -d 1"
)]
pub struct Args {
    /// Search restaurants by restaurant or area name
    #[arg(short, long)]
    pub query: Option<String>,

    /// Specify day
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    pub day: i32,

    /// Filter courses by keyword
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Show only n restaurants
    #[arg(short, long)]
    pub number: Option<u16>,

    /// Print version
    // Allows both -v and -V short flags to be used instead of just the clap default -V.
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (), // handled automatically, no need for pub

    /// Show restaurant address
    #[arg(short, long)]
    pub address: bool,

    /// Show restaurant URL
    #[arg(short, long)]
    pub url: bool,

    /// Hide closed restaurants when searching for todays menus
    #[arg(short, long = "hide-closed")]
    pub hide_closed: bool,

    /// Save the preferred language (fi or en)
    // TODO: get language options from lang crate
    #[arg(long = "set-lang")]
    pub set_lang: Option<String>,

    /// Display help
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>, // handled automatically, no need for pub
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
    fn test_args_parsing_with_number() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "-n", "5"]);
        let parsed = parse(args);
        assert_eq!(parsed.number, Some(5));
    }

    #[test]
    fn test_args_parsing_with_flags() {
        let args = make_args(&["kanttiinit", "-q", "otaniemi", "-a", "-u", "-h"]);
        let parsed = parse(args);
        assert!(parsed.address);
        assert!(parsed.url);
        assert!(parsed.hide_closed);
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
        assert_eq!(parsed.number, None);
        assert!(!parsed.address);
        assert!(!parsed.url);
        assert!(!parsed.hide_closed);
        assert_eq!(parsed.set_lang, None);
    }
}
