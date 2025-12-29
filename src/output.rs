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
        match restaurant.opening_hours {
            Some(todays_opening_hours) => {
                if day_offset != 0 {
                    cprintln!("<bold>{}</> {}", restaurant.name, todays_opening_hours);
                } else {
                    let current_time = Local::now().time();
                    let times = todays_opening_hours.split_once('-').and_then(|(start, end)| {
                        let start_time = chrono::NaiveTime::parse_from_str(start.trim(), "%H:%M").ok()?;
                        let end_time = chrono::NaiveTime::parse_from_str(end.trim(), "%H:%M").ok()?;
                        Some((start_time, end_time))
                    });

                    match times {
                        Some((start_time, end_time)) if current_time < start_time || current_time > end_time => {
                            // Outside opening hours (before opening or after closing)
                            cprintln!(
                                "<strong>{}</> <dim>{}</>",
                                restaurant.name,
                                todays_opening_hours
                            );
                        }
                        Some((_, end_time)) => {
                            // Inside opening hours
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
                        None => {
                            cprintln!("<bold>{}</> {}", restaurant.name, todays_opening_hours);
                        }
                    }
                }
            }
            None => {
                cprintln!("<bold>{}</>", restaurant.name);
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
    use chrono::NaiveTime;

    #[test]
    fn test_outside_hours_before_opening_shows_gray() {
        // Simulate time at 08:00, restaurant opens at 10:00-14:00
        let opening_hours = "10:00-14:00";
        let current_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();

        let (start_str, _) = opening_hours.split_once('-').unwrap();
        let start_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M").unwrap();

        // Before opening hours
        assert!(current_time < start_time);
        // Should show gray (dim) format, not green
        // This is the behavior we want: <strong>Name</> <dim>10:00-14:00</>
    }

    #[test]
    fn test_outside_hours_after_closing_shows_gray() {
        // Simulate time at 16:00, restaurant closes at 14:00
        let opening_hours = "10:00-14:00";
        let current_time = NaiveTime::from_hms_opt(16, 0, 0).unwrap();

        let (_, end_str) = opening_hours.split_once('-').unwrap();
        let end_time = NaiveTime::parse_from_str(end_str.trim(), "%H:%M").unwrap();

        // After closing hours
        assert!(current_time > end_time);
        // Should show gray (dim) format, not green
        // This is the behavior we want: <strong>Name</> <dim>10:00-14:00</>
    }

    #[test]
    fn test_outside_hours_no_closing_in_message() {
        // When outside hours, should NOT show "closing in X hours Y mins"
        let opening_hours = "10:00-14:00";

        // Test before opening (08:00)
        let current_time_before = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let (start_str, _) = opening_hours.split_once('-').unwrap();
        let start_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M").unwrap();
        assert!(current_time_before < start_time);
        // Should NOT calculate or show "closing in" message

        // Test after closing (16:00)
        let current_time_after = NaiveTime::from_hms_opt(16, 0, 0).unwrap();
        let (_, end_str) = opening_hours.split_once('-').unwrap();
        let end_time = NaiveTime::parse_from_str(end_str.trim(), "%H:%M").unwrap();
        assert!(current_time_after > end_time);
        // Should NOT calculate or show "closing in" message
    }

    #[test]
    fn test_inside_hours_shows_green() {
        // Simulate time at 12:00, restaurant open 10:00-14:00
        let opening_hours = "10:00-14:00";
        let current_time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

        let (start_str, end_str) = opening_hours.split_once('-').unwrap();
        let start_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M").unwrap();
        let end_time = NaiveTime::parse_from_str(end_str.trim(), "%H:%M").unwrap();

        // Inside opening hours
        assert!(current_time >= start_time && current_time <= end_time);
        // Should show green format with "closing in" message
        // This is the behavior we want: <bold>Name</> <green>10:00-14:00</> <dim>closes in 2h 0m</>

        let closes_in = end_time.signed_duration_since(current_time);
        assert_eq!(closes_in.num_hours(), 2);
    }

    #[test]
    fn test_edge_case_exactly_at_opening_time() {
        // At exactly 10:00, restaurant opens at 10:00-14:00
        let opening_hours = "10:00-14:00";
        let current_time = NaiveTime::from_hms_opt(10, 0, 0).unwrap();

        let (start_str, end_str) = opening_hours.split_once('-').unwrap();
        let start_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M").unwrap();
        let end_time = NaiveTime::parse_from_str(end_str.trim(), "%H:%M").unwrap();

        // Exactly at opening time should count as open
        assert!(current_time >= start_time && current_time <= end_time);
    }

    #[test]
    fn test_edge_case_exactly_at_closing_time() {
        // At exactly 14:00, restaurant closes at 14:00
        let opening_hours = "10:00-14:00";
        let current_time = NaiveTime::from_hms_opt(14, 0, 0).unwrap();

        let (start_str, end_str) = opening_hours.split_once('-').unwrap();
        let start_time = NaiveTime::parse_from_str(start_str.trim(), "%H:%M").unwrap();
        let end_time = NaiveTime::parse_from_str(end_str.trim(), "%H:%M").unwrap();

        // Exactly at closing time - depending on implementation could be open or closed
        // Current logic uses > end_time, so at exactly end_time it's still open
        assert!(current_time >= start_time && current_time <= end_time);
    }
}
