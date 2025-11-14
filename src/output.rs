use chrono::Local;
use color_print::cprintln;

use crate::search::RestaurantWithMenu;

// TODO: i18n based on lang selection
pub fn print_menus(
    restaurants_with_menus: Vec<RestaurantWithMenu>,
    day_offset: i32,
    print_address: bool,
    print_url: bool,
) {
    let date_offset =
        (Local::now() + chrono::Duration::days(i64::from(day_offset))).format("%A %-d. of %B %Y");
    cprintln!("{}", date_offset);
    cprintln!("");
    // check if all the restaurants menus are None
    if restaurants_with_menus
        .iter()
        .all(|r| r.formatted_menu_items.is_none())
    {
        cprintln!("<red>No restaurants matched your query.</>");
        return;
    }
    for restaurant in restaurants_with_menus {
        let distance_str = match restaurant.distance {
            Some(d) => {
                if d >= 1000 {
                    format!(" <dim>{:.1}km</>", d as f64 / 1000.0)
                } else {
                    format!(" <dim>{}m</>", d)
                }
            }
            None => String::new(),
        };

        match restaurant.opening_hours {
            Some(todays_opening_hours) => {
                if day_offset != 0 {
                    cprintln!(
                        "<bold>{}</>{} {}",
                        restaurant.name,
                        distance_str,
                        todays_opening_hours
                    );
                } else {
                    let opening_hours_split = todays_opening_hours
                        .split('-')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>();
                    let end_time = chrono::NaiveTime::parse_from_str(
                        opening_hours_split.last().unwrap(),
                        "%H:%M",
                    )
                    .unwrap();
                    let current_time = Local::now().time();
                    if current_time > end_time {
                        cprintln!(
                            "<strong>{}</>{} <dim>{}</>",
                            restaurant.name,
                            distance_str,
                            todays_opening_hours
                        );
                    } else {
                        let closes_in = end_time.signed_duration_since(current_time);
                        let closes_in_formatted = format!(
                            "{}h {}m",
                            closes_in.num_hours(),
                            closes_in.num_minutes() % 60
                        );
                        cprintln!(
                            "<bold>{}</>{} <green>{}</> <dim>closes in {}</>",
                            restaurant.name,
                            distance_str,
                            todays_opening_hours,
                            closes_in_formatted
                        );
                    }
                }
            }
            None => {
                cprintln!("<bold>{}</>{}", restaurant.name, distance_str);
                continue;
            }
        }
        if print_address {
            cprintln!("<dim>{}</>", restaurant.address);
        }
        if print_url {
            cprintln!("<dim>{}</>", restaurant.url);
        }
        match restaurant.formatted_menu_items {
            Some(menu_items) => {
                for menu_item in menu_items {
                    cprintln!("◦ {} <dim>{}</>", menu_item.title, menu_item.properties);
                }
            }
            None => {
                cprintln!("No menu.");
            }
        }
        cprintln!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_restaurant_with_menu(
        name: &str,
        distance: Option<u32>,
        opening_hours: Option<String>,
        menu_count: usize,
    ) -> RestaurantWithMenu {
        let menu_items = if menu_count > 0 {
            Some(
                (0..menu_count)
                    .map(|i| crate::search::FormattedMenuItem {
                        title: format!("Menu Item {}", i + 1),
                        properties: format!("property{}", i + 1),
                    })
                    .collect(),
            )
        } else {
            None
        };

        RestaurantWithMenu {
            name: name.to_string(),
            opening_hours,
            address: "Test Address".to_string(),
            url: "https://example.com".to_string(),
            distance,
            formatted_menu_items: menu_items,
        }
    }

    #[test]
    fn test_restaurant_with_menu_structure() {
        let restaurant = create_test_restaurant_with_menu("Test", Some(500), None, 2);
        assert_eq!(restaurant.name, "Test");
        assert_eq!(restaurant.distance, Some(500));
        assert_eq!(restaurant.formatted_menu_items.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_restaurant_without_menu() {
        let restaurant = create_test_restaurant_with_menu("No Menu", None, None, 0);
        assert_eq!(restaurant.name, "No Menu");
        assert!(restaurant.formatted_menu_items.is_none());
    }

    #[test]
    fn test_restaurant_with_distance() {
        let restaurant = create_test_restaurant_with_menu("Close", Some(100), None, 1);
        assert_eq!(restaurant.distance, Some(100));

        let far_restaurant = create_test_restaurant_with_menu("Far", Some(5000), None, 1);
        assert_eq!(far_restaurant.distance, Some(5000));
    }

    #[test]
    fn test_restaurant_with_opening_hours() {
        let restaurant =
            create_test_restaurant_with_menu("Open", None, Some("10:30-14:00".to_string()), 1);
        assert_eq!(restaurant.opening_hours, Some("10:30-14:00".to_string()));
    }

    #[test]
    fn test_multiple_menu_items() {
        let restaurant = create_test_restaurant_with_menu("Multi", None, None, 5);
        assert_eq!(restaurant.formatted_menu_items.as_ref().unwrap().len(), 5);

        for (i, item) in restaurant
            .formatted_menu_items
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
        {
            assert_eq!(item.title, format!("Menu Item {}", i + 1));
            assert_eq!(item.properties, format!("property{}", i + 1));
        }
    }

    // Note: Testing print_menus function directly is challenging because it uses
    // cprintln! which outputs to stdout. The tests above verify the data structures
    // that are used by print_menus. For full integration testing, consider using
    // a mocking library for output capture or refactoring print_menus to accept
    // a writer parameter for testability.
}
