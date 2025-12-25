use crate::args::Args;
use crate::lang::{get_lang, set_lang};

use crate::output::print_menus;
use crate::search::{
    filter_menus_and_format_to_restaurants_with_menus, get_menus_by_restaurants,
    get_restaurants_by_query_filtered_by_closed_status_and_ordered_alphabetically, Restaurant,
};

fn limit_restaurants(restaurants: &[Restaurant], limit: Option<u16>) -> &[Restaurant] {
    limit.map_or(restaurants, |n| {
        let n = usize::from(n);
        &restaurants[..n.min(restaurants.len())]
    })
}

pub fn handle_arg(args: Args) {
    // Set language if provided
    if let Some(lang_from_user) = args.set_lang {
        match set_lang(&lang_from_user) {
            Ok(_) => println!("Language set to: {}", lang_from_user),
            Err(e) => println!("Error setting language: {}", e),
        }
        return;
    }

    // Get language to use
    let lang = match get_lang() {
        Ok(l) => l,
        Err(e) => {
            println!("Error getting language: {}", e);
            return;
        }
    };

    // Check that query is provided
    if args.query.is_none() {
        println!("Use the -q option to query restaurants. Display help with --help.");
        return;
    }

    // Check if both day and hide_closed are provided
    if args.day != 0 && args.hide_closed {
        println!("Cannot use both -d and -h options at the same time. Hiding closed restaurants works only when searching for todays menus. Display help with --help.");
        return;
    }

    // Handle query-based search
    if let Some(query) = args.query {
        match get_restaurants_by_query_filtered_by_closed_status_and_ordered_alphabetically(
            &query,
            &lang,
            args.hide_closed,
        ) {
            Ok(restaurants) => {
                let limited_restaurants = limit_restaurants(&restaurants, args.number);
                match get_menus_by_restaurants(limited_restaurants, &lang, args.day) {
                    Ok(menus) => {
                        let formatted_restaurants =
                            filter_menus_and_format_to_restaurants_with_menus(
                                limited_restaurants,
                                &menus,
                                &args.filter,
                            );
                        print_menus(formatted_restaurants, args.day, args.address, args.url);
                    }
                    Err(e) => println!("Error getting menus: {}", e),
                }
            }
            Err(e) => println!("Error searching by query: {}", e),
        }
    }
}
