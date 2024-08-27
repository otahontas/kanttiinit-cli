use crate::args::Args;
use crate::lang::{get_lang, set_lang};

use crate::search::search_by_query;

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

    // Check that either query or geo is provided
    if args.query.is_none() && args.geo.is_none() {
        println!("Use either the -q or -g option to query restaurants. Display help with --help.");
        return;
    }

    // Check if both query and geo are provided
    if args.query.is_some() && args.geo.is_some() {
        println!("Cannot use both -q and -g options at the same time. Display help with --help.");
        return;
    }

    if let Some(query) = args.query {
        match search_by_query(&query, &lang, args.day) {
            Ok(_) => (),
            Err(e) => println!("Error searching by query: {}", e),
        }
    }
    //
    //if let Some(geo) = args.geo {
    //    println!("Searching restaurants by location: {}", geo);
    //}

    //
    //if let Some(filter) = args.filter {
    //    println!("Filtering courses by keyword: {}", filter);
    //}
    //
    //if let Some(number) = args.number {
    //    println!("Showing only {} restaurants", number);
    //}
    //
    //if args.address {
    //    println!("Showing restaurant address");
    //}
    //
    //if args.url {
    //    println!("Showing restaurant URL");
    //}
    //
    //if args.hide_closed {
    //    println!("Hiding closed restaurants");
    //}
}
