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
    // TODO: read this from variable / other module?
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
struct Args {
    /// Search restaurants by restaurant or area name
    #[arg(short, long)]
    query: Option<String>,

    /// Search restaurants by location
    #[arg(short, long)]
    geo: Option<String>,

    /// Specify day
    #[arg(short, long, default_value_t = 0)]
    day: u32,

    /// Filter courses by keyword
    #[arg(short, long)]
    filter: Option<String>,

    /// Show only n restaurants
    #[arg(short, long)]
    number: Option<u32>,

    /// Print version
    // Allows both -v and -V short flags to be used instead of just the clap default -V.
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (),

    /// Show restaurant address
    #[arg(short, long)]
    address: bool,

    /// Show restaurant URL
    #[arg(short, long)]
    url: bool,

    /// Hide closed restaurants
    #[arg(short, long = "hide-closed")]
    hide_closed: bool,

    /// Save the preferred language (fi or en)
    #[arg(long = "set-lang")]
    set_lang: Option<String>,

    /// Display help
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();

    // TODO: Implement actual functionality

    if let Some(query) = args.query {
        println!("Searching restaurants by name or area: {}", query);
    }

    if let Some(geo) = args.geo {
        println!("Searching restaurants by location: {}", geo);
    }

    if args.day != 0 {
        println!("Day specified: {}", args.day);
    }

    if let Some(filter) = args.filter {
        println!("Filtering courses by keyword: {}", filter);
    }

    if let Some(number) = args.number {
        println!("Showing only {} restaurants", number);
    }

    if args.address {
        println!("Showing restaurant address");
    }

    if args.url {
        println!("Showing restaurant URL");
    }

    if args.hide_closed {
        println!("Hiding closed restaurants");
    }

    if let Some(lang) = args.set_lang {
        println!("Setting preferred language to: {}", lang);
    }
}
