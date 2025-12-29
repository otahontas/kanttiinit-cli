use crate::args::Args;
use crate::lang::{get_lang, set_lang};
use crate::output::print_menus;
use crate::search::{format_restaurants_with_menus, get_menus, get_restaurants, Restaurant};

fn limit_restaurants(restaurants: &[Restaurant], limit: Option<u16>) -> &[Restaurant] {
    limit.map_or(restaurants, |n| {
        let n = usize::from(n);
        &restaurants[..n.min(restaurants.len())]
    })
}

fn handle_query(query: &str, lang: &str, args: &Args) -> Result<(), anyhow::Error> {
    let restaurants = get_restaurants(query, lang, args.hide_closed)?;

    // If hide_no_menu is enabled, we need to fetch menus first, then filter
    if args.hide_no_menu {
        let menus = get_menus(&restaurants, lang, args.day)?;
        let filtered: Vec<_> = restaurants
            .into_iter()
            .filter(|r| menus.contains_key(&r.id.to_string()))
            .collect();
        let limited = limit_restaurants(&filtered, args.head);
        let formatted = format_restaurants_with_menus(limited, &menus, &args.filter);
        print_menus(formatted, args.day, args.address, args.url);
    } else {
        let limited = limit_restaurants(&restaurants, args.head);
        let menus = get_menus(limited, lang, args.day)?;
        let formatted = format_restaurants_with_menus(limited, &menus, &args.filter);
        print_menus(formatted, args.day, args.address, args.url);
    }

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
        eprintln!("Cannot use both -d and --hide-closed options at the same time. Hiding closed restaurants works only when searching for todays menus. Display help with --help.");
        return;
    }

    if args.day != 0 && args.hide_no_menu {
        eprintln!("Cannot use both -d and --hide-no-menu options at the same time. Hiding restaurants without menu works only when searching for todays menus. Display help with --help.");
        return;
    }

    if let Some(query) = &args.query {
        if let Err(e) = handle_query(query, &lang, &args) {
            eprintln!("Error: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures;

    #[test]
    fn test_limit_restaurants_with_none_returns_all() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, None);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_limit_restaurants_with_limit_smaller_than_len() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, Some(2));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_limit_restaurants_with_limit_equal_to_len() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, Some(3));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_limit_restaurants_with_limit_greater_than_len() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, Some(100));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_limit_restaurants_with_zero() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, Some(0));
        assert!(result.is_empty());
    }

    #[test]
    fn test_limit_restaurants_empty_slice() {
        let restaurants: Vec<Restaurant> = vec![];
        let result = limit_restaurants(&restaurants, Some(5));
        assert!(result.is_empty());
    }

    #[test]
    fn test_limit_restaurants_preserves_order() {
        let restaurants = test_fixtures::sample_restaurants();
        let result = limit_restaurants(&restaurants, Some(2));
        assert_eq!(result[0].name(), restaurants[0].name());
        assert_eq!(result[1].name(), restaurants[1].name());
    }
}
