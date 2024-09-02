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
        // TODO: use string builder
        match restaurant.opening_hours {
            Some(todays_opening_hours) => {
                if day_offset != 0 {
                    cprintln!("<bold>{}</> {}", restaurant.name, todays_opening_hours);
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
                            "<strong>{}</> <dim>{}</>",
                            restaurant.name,
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
                            "<bold>{}</> <green>{}</> <dim>closes in {}</>",
                            restaurant.name,
                            todays_opening_hours,
                            closes_in_formatted
                        );
                    }
                }
            }
            None => {
                cprintln!("<bold>{}</>", restaurant.name);
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
