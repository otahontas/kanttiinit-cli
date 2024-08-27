use std::env::ArgsOs;

use clap::Parser;

// TODO: Colors

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

Get restaurants close to a location:
kanttiinit -g Otakaari 8

Only list courses that match a certain keyword:
kanttiinit -q töölö -f salad

See menus for tomorrow:
kanttiinit -q alvari -d 1"
)]
pub struct Args {
    /// Search restaurants by restaurant or area name
    #[arg(short, long)]
    pub query: Option<String>,

    /// Search restaurants by location
    #[arg(short, long)]
    pub geo: Option<String>,

    /// Specify day
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    pub day: i32,

    /// Filter courses by keyword
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Show only n restaurants
    #[arg(short, long)]
    pub number: Option<u32>,

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

    /// Hide closed restaurants
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

// TODO: error handling
// TODO: tests + generic args for easier testing
pub fn parse(args: ArgsOs) -> Args {
    Args::parse_from(args)
}
