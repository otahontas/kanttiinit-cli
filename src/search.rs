use chrono::{Datelike, Local};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Restaurant {
    #[serde(rename = "openingHours")]
    opening_hours: Vec<Option<String>>,
    pub id: u8,
    name: String,
    url: String,
    address: String,
}

impl Restaurant {
    #[cfg(test)]
    pub fn new(
        id: u8,
        name: String,
        url: String,
        address: String,
        opening_hours: Vec<Option<String>>,
    ) -> Self {
        Self {
            id,
            name,
            url,
            address,
            opening_hours,
        }
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(test)]
    pub fn opening_hours(&self) -> &[Option<String>] {
        &self.opening_hours
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MenuItem {
    title: String,
    properties: Vec<String>,
}

impl MenuItem {
    #[cfg(test)]
    pub fn new(title: String, properties: Vec<String>) -> Self {
        Self { title, properties }
    }
}

type Restaurants = Vec<Restaurant>;
type MenuItems = Vec<MenuItem>;
type DailyMenu = HashMap<String, MenuItems>;
type MenusFromApi = HashMap<String, DailyMenu>;
type Menus = HashMap<String, MenuItems>;

fn is_restaurant_open_now(opening_hours: &[Option<String>], weekday_index: u32) -> bool {
    let hours = match opening_hours.get(weekday_index as usize) {
        Some(Some(h)) => h,
        _ => return false,
    };
    let (start_str, end_str) = match hours.split_once('-') {
        Some((s, e)) => (s.trim(), e.trim()),
        None => return false,
    };
    let start_time = match chrono::NaiveTime::parse_from_str(start_str, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };
    let end_time = match chrono::NaiveTime::parse_from_str(end_str, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };
    let now = chrono::Local::now().time();
    start_time <= now && now <= end_time
}

pub fn get_restaurants(
    query: &str,
    lang: &str,
    hide_closed: bool,
) -> Result<Restaurants, anyhow::Error> {
    let current_date_index_in_week = chrono::offset::Local::now()
        .date_naive()
        .weekday()
        .days_since(chrono::Weekday::Mon);
    let mut restaurants = ureq::get("https://kitchen.kanttiinit.fi/restaurants")
        .query("query", query)
        .query("lang", lang)
        .call()?
        .into_json::<Restaurants>()?
        .into_iter()
        .filter(|restaurant| {
            if !hide_closed {
                return true;
            }
            is_restaurant_open_now(&restaurant.opening_hours, current_date_index_in_week)
        })
        .collect::<Restaurants>();
    restaurants.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(restaurants)
}

pub fn get_menus(
    restaurants: &[Restaurant],
    lang: &str,
    day_offset: i32,
) -> Result<Menus, anyhow::Error> {
    let day_to_fetch_query =
        (Local::now() + chrono::Duration::days(i64::from(day_offset))).format("%Y-%m-%d");

    Ok(ureq::get("https://kitchen.kanttiinit.fi/menus")
        .query(
            "restaurants",
            &restaurants
                .iter()
                .map(|r| r.id.to_string())
                .collect::<Vec<String>>()
                .join(","),
        )
        .query("days", &day_to_fetch_query.to_string())
        .query("lang", lang)
        .call()?
        .into_json::<MenusFromApi>()?
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc, (restaurant_id, menu_for_this_day_map)| {
                if let Some(menu_for_this_day) =
                    menu_for_this_day_map.get(&day_to_fetch_query.to_string())
                {
                    acc.insert(restaurant_id, menu_for_this_day.clone());
                }
                acc
            },
        ))
}

#[derive(PartialEq, Debug)]
pub struct FormattedMenuItem {
    pub title: String,
    pub properties: String,
}

#[derive(PartialEq, Debug)]
pub struct RestaurantWithMenu {
    pub name: String,
    pub opening_hours: Option<String>,
    pub address: String,
    pub url: String,
    pub formatted_menu_items: Option<Vec<FormattedMenuItem>>,
}

// TODO: format opening_hours properly
pub fn format_restaurants_with_menus(
    restaurants: &[Restaurant],
    menus: &Menus,
    maybe_filter: &Option<String>,
) -> Vec<RestaurantWithMenu> {
    let filter = maybe_filter.as_deref().unwrap_or_default().to_lowercase();
    restaurants
        .iter()
        .map(|restaurant| {
            let menu_id = restaurant.id.to_string();
            let formatted_menu_items = menus.get(&menu_id).map(|items| {
                items
                    .iter()
                    .filter(|item| item.title.to_lowercase().contains(&filter))
                    .map(|item| FormattedMenuItem {
                        title: item.title.clone(),
                        properties: item.properties.join(", "),
                    })
                    .collect()
            });
            RestaurantWithMenu {
                name: restaurant.name.clone(),
                opening_hours: restaurant.opening_hours.first().cloned().flatten(),
                address: restaurant.address.clone(),
                url: restaurant.url.clone(),
                formatted_menu_items,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_fixtures;

    // Tests for is_restaurant_open_now

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_empty_hours() {
        let hours: Vec<Option<String>> = vec![];
        assert!(!is_restaurant_open_now(&hours, 0));
    }

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_none_on_weekday() {
        let hours = vec![None, Some("10:00-14:00".to_string())];
        assert!(!is_restaurant_open_now(&hours, 0)); // Monday is None
    }

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_weekday_out_of_bounds() {
        let hours = vec![Some("10:00-14:00".to_string())];
        assert!(!is_restaurant_open_now(&hours, 7)); // Only Monday exists
    }

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_malformed_hours_no_dash() {
        let hours = vec![Some("1000".to_string())];
        assert!(!is_restaurant_open_now(&hours, 0));
    }

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_invalid_start_time() {
        let hours = vec![Some("invalid-14:00".to_string())];
        assert!(!is_restaurant_open_now(&hours, 0));
    }

    #[test]
    fn test_is_restaurant_open_now_returns_false_for_invalid_end_time() {
        let hours = vec![Some("10:00-invalid".to_string())];
        assert!(!is_restaurant_open_now(&hours, 0));
    }

    // Tests for format_restaurants_with_menus

    #[test]
    fn test_format_restaurants_with_menus_basic() {
        let restaurants = test_fixtures::sample_restaurants();
        let menus = test_fixtures::sample_menus();

        let result = format_restaurants_with_menus(&restaurants, &menus, &None);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "Aalto Yliopiston ravintola");
        assert_eq!(result[0].address, "Otakaari 1, Espoo");
        assert_eq!(result[0].url, "https://example.com/aalto");
        assert_eq!(result[0].opening_hours, Some("10:30-13:30".to_string()));

        let menu_items = result[0].formatted_menu_items.as_ref().unwrap();
        assert_eq!(menu_items.len(), 3);
        assert_eq!(menu_items[0].title, "Lohikeitto");
        assert_eq!(menu_items[0].properties, "G, L");
    }

    #[test]
    fn test_format_restaurants_with_menus_with_filter() {
        let restaurants = test_fixtures::sample_restaurants();
        let menus = test_fixtures::sample_menus();

        let result =
            format_restaurants_with_menus(&restaurants, &menus, &Some("salad".to_string()));

        // First restaurant should only have "Chicken salad"
        let menu_items = result[0].formatted_menu_items.as_ref().unwrap();
        assert_eq!(menu_items.len(), 1);
        assert_eq!(menu_items[0].title, "Chicken salad");
    }

    #[test]
    fn test_format_restaurants_with_menus_filter_matches_nothing() {
        let restaurants = test_fixtures::sample_restaurants();
        let menus = test_fixtures::sample_menus();

        let result = format_restaurants_with_menus(
            &restaurants,
            &menus,
            &Some("nonexistent_food".to_string()),
        );

        // All restaurants should have empty menu items after filtering
        for r in &result {
            if let Some(items) = &r.formatted_menu_items {
                assert!(items.is_empty());
            }
        }
    }

    #[test]
    fn test_format_restaurants_with_menus_filter_case_insensitive() {
        let restaurants = test_fixtures::sample_restaurants();
        let menus = test_fixtures::sample_menus();

        // Test with lowercase filter
        let result_lower = format_restaurants_with_menus(
            &restaurants,
            &menus,
            &Some("salad".to_string()),
        );

        // Test with uppercase filter
        let result_upper = format_restaurants_with_menus(
            &restaurants,
            &menus,
            &Some("SALAD".to_string()),
        );

        // Test with mixed case filter
        let result_mixed = format_restaurants_with_menus(
            &restaurants,
            &menus,
            &Some("SaLaD".to_string()),
        );

        // All three should match the same item "Chicken salad"
        assert_eq!(result_lower, result_upper);
        assert_eq!(result_lower, result_mixed);

        let menu_items = result_lower[0].formatted_menu_items.as_ref().unwrap();
        assert_eq!(menu_items.len(), 1);
        assert_eq!(menu_items[0].title, "Chicken salad");
    }

    #[test]
    fn test_format_restaurants_with_menus_no_menu_for_restaurant() {
        let restaurants = vec![test_fixtures::restaurant_without_hours()];
        let menus = test_fixtures::empty_menus();

        let result = format_restaurants_with_menus(&restaurants, &menus, &None);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Mystery Restaurant");
        assert!(result[0].formatted_menu_items.is_none());
    }

    #[test]
    fn test_format_restaurants_with_menus_empty_opening_hours() {
        let restaurants = vec![test_fixtures::restaurant_without_hours()];
        let menus = test_fixtures::sample_menus();

        let result = format_restaurants_with_menus(&restaurants, &menus, &None);

        assert_eq!(result.len(), 1);
        assert!(result[0].opening_hours.is_none());
    }

    #[test]
    fn test_format_restaurants_with_menus_properties_joined() {
        let restaurants = test_fixtures::sample_restaurants();
        let menus = test_fixtures::sample_menus();

        let result = format_restaurants_with_menus(&restaurants, &menus, &None);

        // Check that properties are properly joined with ", "
        let menu_items = result[0].formatted_menu_items.as_ref().unwrap();
        assert!(menu_items[0].properties.contains(", "));
    }

    #[test]
    fn test_format_restaurants_with_menus_empty_restaurants() {
        let restaurants: Vec<Restaurant> = vec![];
        let menus = test_fixtures::sample_menus();

        let result = format_restaurants_with_menus(&restaurants, &menus, &None);

        assert!(result.is_empty());
    }

    #[test]
    fn test_is_restaurant_open_now_handles_malformed_hours_gracefully() {
        let restaurant = test_fixtures::restaurant_with_malformed_hours();
        let hours = restaurant.opening_hours();

        // Monday: "invalid" - should return false
        assert!(!is_restaurant_open_now(hours, 0));
        // Tuesday: "10:00" - missing end time, should return false
        assert!(!is_restaurant_open_now(hours, 1));
        // Wednesday: "10:00-" - empty end time, should return false
        assert!(!is_restaurant_open_now(hours, 2));
        // Thursday: "-14:00" - empty start time, should return false
        assert!(!is_restaurant_open_now(hours, 3));
        // Friday: "25:00-26:00" - invalid times, should return false
        assert!(!is_restaurant_open_now(hours, 4));
    }

    // Integration tests with real API fixtures
    // These verify that our deserializers can handle actual API responses

    #[test]
    fn test_deserialize_otaniemi_restaurants_from_api() {
        let json = include_str!("test_fixtures/otaniemi_restaurants.json");
        let restaurants: Restaurants = serde_json::from_str(json)
            .expect("Failed to deserialize otaniemi restaurants from real API response");

        // Should have multiple restaurants
        assert!(!restaurants.is_empty(), "Should have at least one restaurant");

        // Verify all restaurants have required fields matching our Restaurant struct
        for restaurant in &restaurants {
            assert!(!restaurant.name.is_empty(), "Restaurant name should not be empty");
            assert!(!restaurant.url.is_empty(), "Restaurant URL should not be empty");
            assert!(!restaurant.address.is_empty(), "Restaurant address should not be empty");
            assert_eq!(
                restaurant.opening_hours.len(),
                7,
                "Should have 7 days of opening hours"
            );
        }
    }

    #[test]
    fn test_deserialize_otaniemi_menus_from_api() {
        let json = include_str!("test_fixtures/otaniemi_menus.json");
        let menus: MenusFromApi = serde_json::from_str(json)
            .expect("Failed to deserialize otaniemi menus from real API response");

        // Verify structure matches our MenusFromApi type
        assert!(!menus.is_empty(), "Menus should not be empty");

        // Check that we can access nested data
        for (restaurant_id, daily_menus) in &menus {
            assert!(!restaurant_id.is_empty(), "Restaurant ID should not be empty");

            for (date, items) in daily_menus {
                // Date should be in YYYY-MM-DD format
                assert!(date.contains('-'), "Date should be in YYYY-MM-DD format");

                // Verify menu items match our MenuItem struct
                for item in items {
                    assert!(!item.title.is_empty(), "Menu item title should not be empty");
                    // Properties can be empty (it's a Vec, not Option)
                }
            }
        }
    }

    #[test]
    fn test_deserialize_keskusta_restaurants_from_api() {
        let json = include_str!("test_fixtures/keskusta_restaurants.json");
        let restaurants: Restaurants = serde_json::from_str(json)
            .expect("Failed to deserialize keskusta restaurants from real API response");

        // Should have multiple restaurants
        assert!(!restaurants.is_empty(), "Should have at least one restaurant");

        // Verify all restaurants have required fields
        for restaurant in &restaurants {
            assert!(!restaurant.name.is_empty(), "Restaurant name should not be empty");
            assert!(!restaurant.url.is_empty(), "Restaurant URL should not be empty");
            assert!(!restaurant.address.is_empty(), "Restaurant address should not be empty");
            assert_eq!(
                restaurant.opening_hours.len(),
                7,
                "Should have 7 days of opening hours"
            );
        }
    }

    #[test]
    fn test_deserialize_keskusta_menus_from_api() {
        let json = include_str!("test_fixtures/keskusta_menus.json");
        let menus: MenusFromApi = serde_json::from_str(json)
            .expect("Failed to deserialize keskusta menus from real API response");

        // Verify structure is correct
        assert!(!menus.is_empty(), "Menus should not be empty");

        // Check nested structure
        for (restaurant_id, daily_menus) in &menus {
            assert!(!restaurant_id.is_empty(), "Restaurant ID should not be empty");

            for (date, items) in daily_menus {
                assert!(date.contains('-'), "Date should be in YYYY-MM-DD format");

                for item in items {
                    assert!(!item.title.is_empty(), "Menu item title should not be empty");
                }
            }
        }
    }

    #[test]
    fn test_api_responses_handle_empty_menus() {
        let json = include_str!("test_fixtures/otaniemi_menus.json");
        let menus: MenusFromApi = serde_json::from_str(json)
            .expect("Failed to deserialize otaniemi menus");

        // Some restaurants should have empty menus (closed during holidays)
        let has_empty = menus
            .values()
            .any(|daily_menus| daily_menus.values().any(|items| items.is_empty()));

        assert!(
            has_empty,
            "Real API data should include restaurants with empty menus"
        );
    }

    #[test]
    fn test_api_responses_handle_closure_messages() {
        let json = include_str!("test_fixtures/otaniemi_menus.json");
        let menus: MenusFromApi = serde_json::from_str(json)
            .expect("Failed to deserialize otaniemi menus");

        // Some restaurants have closure messages as menu items
        let has_closure_message = menus.values().any(|daily_menus| {
            daily_menus.values().any(|items| {
                items
                    .iter()
                    .any(|item| item.title.contains("closed") || item.title.contains("Christmas"))
            })
        });

        assert!(
            has_closure_message,
            "Real API data should include closure messages"
        );
    }
}
