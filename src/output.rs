use chrono::{NaiveDate, NaiveTime};

use crate::search::RestaurantWithMenu;

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn format_opening_hours_line(name: &str, hours: &str, now: NaiveTime) -> String {
    let times = hours.split_once('-').and_then(|(start, end)| {
        let start_time = chrono::NaiveTime::parse_from_str(start.trim(), "%H:%M").ok()?;
        let end_time = chrono::NaiveTime::parse_from_str(end.trim(), "%H:%M").ok()?;
        Some((start_time, end_time))
    });

    match times {
        Some((start_time, end_time)) if now < start_time || now > end_time => {
            format!("{BOLD}{name}{RESET} {DIM}{hours}{RESET}")
        }
        Some((_, end_time)) => {
            let closes_in = end_time.signed_duration_since(now);
            let closes_in_formatted = format!(
                "{}h {}m",
                closes_in.num_hours(),
                closes_in.num_minutes() % 60
            );
            format!(
                "{BOLD}{name}{RESET} {GREEN}{hours}{RESET} {DIM}closes in {closes_in_formatted}{RESET}"
            )
        }
        None => {
            format!("{BOLD}{name}{RESET} {hours}")
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
    println!("{}", target_date.format("%A %-d. of %B %Y"));
    println!();
    if restaurants_with_menus.is_empty() {
        println!("{RED}No restaurants matched your query.{RESET}");
        return;
    }
    for restaurant in restaurants_with_menus {
        match &restaurant.opening_hours {
            Some(hours) if is_today => {
                println!(
                    "{}",
                    format_opening_hours_line(&restaurant.name, hours, now)
                );
            }
            Some(hours) => {
                println!("{BOLD}{}{RESET} {}", restaurant.name, hours);
            }
            None => {
                println!("{BOLD}{}{RESET}", restaurant.name);
            }
        }
        if print_address {
            println!("{DIM}{}{RESET}", restaurant.address);
        }
        if print_url {
            println!("{DIM}{}{RESET}", restaurant.url);
        }
        match restaurant.formatted_menu_items {
            Some(menu_items) => {
                for menu_item in menu_items {
                    println!("◦ {} {DIM}{}{RESET}", menu_item.title, menu_item.properties);
                }
            }
            None => {
                println!("No menu.");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_before_opening_shows_dim() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(line.contains(DIM));
        assert!(!line.contains(GREEN));
    }

    #[test]
    fn test_after_closing_shows_dim() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );
        assert!(line.contains(DIM));
        assert!(!line.contains(GREEN));
    }

    #[test]
    fn test_during_open_hours_shows_green_with_closes_in() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        assert!(line.contains(GREEN));
        assert!(line.contains("closes in 2h 0m"));
    }

    #[test]
    fn test_exactly_at_opening_shows_green() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        );
        assert!(line.contains(GREEN));
        assert!(line.contains("closes in 4h 0m"));
    }

    #[test]
    fn test_exactly_at_closing_shows_green() {
        let line = format_opening_hours_line(
            "Cafe",
            "10:00-14:00",
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        );
        assert!(line.contains(GREEN));
        assert!(line.contains("closes in 0h 0m"));
    }

    #[test]
    fn test_unparsable_hours_shows_plain() {
        let line = format_opening_hours_line(
            "Cafe",
            "invalid",
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        assert!(line.contains(BOLD));
        assert!(line.contains("Cafe"));
        assert!(line.contains("invalid"));
        assert!(!line.contains(GREEN));
        assert!(!line.contains(DIM));
    }
}
