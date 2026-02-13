use chrono::{NaiveDate, NaiveTime};
use color_print::{cformat, cprintln};

use crate::search::RestaurantWithMenu;

fn format_opening_hours_line(name: &str, hours: &str, now: NaiveTime) -> String {
    let times = hours.split_once('-').and_then(|(start, end)| {
        let start_time = chrono::NaiveTime::parse_from_str(start.trim(), "%H:%M").ok()?;
        let end_time = chrono::NaiveTime::parse_from_str(end.trim(), "%H:%M").ok()?;
        Some((start_time, end_time))
    });

    match times {
        Some((start_time, end_time)) if now < start_time || now > end_time => {
            cformat!("<strong>{name}</> <dim>{hours}</>")
        }
        Some((_, end_time)) => {
            let closes_in = end_time.signed_duration_since(now);
            let closes_in_formatted = format!(
                "{}h {}m",
                closes_in.num_hours(),
                closes_in.num_minutes() % 60
            );
            cformat!("<bold>{name}</> <green>{hours}</> <dim>closes in {closes_in_formatted}</>")
        }
        None => {
            cformat!("<bold>{name}</> {hours}")
        }
    }
}

// Date formatting is English-only. The lang setting only affects API content (menu items,
// restaurant names), not the CLI's own output. Localizing date display is out of scope.
pub fn print_menus(
    restaurants_with_menus: Vec<RestaurantWithMenu>,
    target_date: NaiveDate,
    now: NaiveTime,
    is_today: bool,
    print_address: bool,
    print_url: bool,
) {
    cprintln!("{}", target_date.format("%A %-d. of %B %Y"));
    cprintln!("");
    if restaurants_with_menus.is_empty() {
        cprintln!("<red>No restaurants matched your query.</>");
        return;
    }
    for restaurant in restaurants_with_menus {
        match &restaurant.opening_hours {
            Some(hours) if is_today => {
                cprintln!(
                    "{}",
                    format_opening_hours_line(&restaurant.name, hours, now)
                );
            }
            Some(hours) => {
                cprintln!("<bold>{}</> {}", restaurant.name, hours);
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
    use super::*;
    use chrono::NaiveTime;

    // ANSI escape codes used by color_print's cformat! macro
    const ANSI_DIM: &str = "\x1b[2m";
    const ANSI_GREEN: &str = "\x1b[32m";
    const ANSI_BOLD: &str = "\x1b[1m";

    #[test]
    fn test_before_opening_shows_dim() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_DIM));
        assert!(!line.contains(ANSI_GREEN));
    }

    #[test]
    fn test_after_closing_shows_dim() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_DIM));
        assert!(!line.contains(ANSI_GREEN));
    }

    #[test]
    fn test_during_open_hours_shows_green_with_closes_in() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_GREEN));
        assert!(line.contains("closes in 2h 0m"));
    }

    #[test]
    fn test_exactly_at_opening_shows_green() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_GREEN));
        assert!(line.contains("closes in 4h 0m"));
    }

    #[test]
    fn test_exactly_at_closing_shows_green() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_GREEN));
        assert!(line.contains("closes in 0h 0m"));
    }

    #[test]
    fn test_unparsable_hours_shows_plain() {
        let line = format_opening_hours_line(
            "Cafe",
            "invalid",
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        assert!(line.contains(ANSI_BOLD));
        assert!(line.contains("Cafe"));
        assert!(line.contains("invalid"));
        assert!(!line.contains(ANSI_GREEN));
        assert!(!line.contains(ANSI_DIM));
    }
}
