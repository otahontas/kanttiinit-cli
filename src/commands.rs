use crate::args::Args;
use crate::lang::{get_lang, set_lang};

pub fn handle_arg(args: Args) -> () {
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
    println!("Using language: {}", lang);

    //if let Some(query) = args.query {
    //    println!("Searching restaurants by name or area: {}", query);
    //}
    //
    //if let Some(geo) = args.geo {
    //    println!("Searching restaurants by location: {}", geo);
    //}
    //
    //if args.day != 0 {
    //    println!("Day specified: {}", args.day);
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
