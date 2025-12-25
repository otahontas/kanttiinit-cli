use crate::args::Args;
use crate::lang::{get_lang, set_lang};
use crate::output::print_menus;
use crate::search::{
    format_restaurants_with_menus, get_menus, get_restaurants, Restaurant,
};

fn limit_restaurants(restaurants: &[Restaurant], limit: Option<u16>) -> &[Restaurant] {
    limit.map_or(restaurants, |n| {
        let n = usize::from(n);
        &restaurants[..n.min(restaurants.len())]
    })
}

fn handle_query(
    query: &str,
    lang: &str,
    args: &Args,
) -> Result<(), anyhow::Error> {
    let restaurants = get_restaurants(query, lang, args.hide_closed)?;
    let limited = limit_restaurants(&restaurants, args.number);
    let menus = get_menus(limited, lang, args.day)?;
    let formatted = format_restaurants_with_menus(limited, &menus, &args.filter);
    print_menus(formatted, args.day, args.address, args.url);
    Ok(())
}

pub fn handle_arg(args: Args) {
    if let Some(lang_from_user) = args.set_lang {
        match set_lang(&lang_from_user) {
            Ok(_) => println!("Language set to: {}", lang_from_user),
            Err(e) => eprintln!("Error setting language: {}", e),
        }
        return;
    }

    let lang = match get_lang() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error getting language: {}", e);
            return;
        }
    };

    if args.query.is_none() {
        eprintln!("Use the -q option to query restaurants. Display help with --help.");
        return;
    }

    if args.day != 0 && args.hide_closed {
        eprintln!("Cannot use both -d and -h options at the same time. Hiding closed restaurants works only when searching for todays menus. Display help with --help.");
        return;
    }

    if let Some(query) = &args.query {
        if let Err(e) = handle_query(query, &lang, &args) {
            eprintln!("Error: {}", e);
        }
    }
}
