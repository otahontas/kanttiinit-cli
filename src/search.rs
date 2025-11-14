use chrono::{Datelike, Local};
use serde::Deserialize;
use std::collections::HashMap;

const API_BASE_URL: &str = "https://kitchen.kanttiinit.fi";

#[derive(Debug, Deserialize)]
pub struct Restaurant {
    #[serde(rename = "openingHours")]
    opening_hours: Vec<Option<String>>,
    id: u8,
    name: String,
    url: String,
    address: String,
    #[serde(default)]
    distance: Option<u32>,
}

impl Restaurant {
    pub fn distance(&self) -> Option<u32> {
        self.distance
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MenuItem {
    title: String,
    properties: Vec<String>,
}

type Restaurants = Vec<Restaurant>;
type MenuItems = Vec<MenuItem>;
type MenuDate = String;
type DailyMenu = HashMap<MenuDate, MenuItems>;
type RestaurantIdAsHashMapKey = String;
type MenusFromApi = HashMap<RestaurantIdAsHashMapKey, DailyMenu>;
type Menus = HashMap<RestaurantIdAsHashMapKey, MenuItems>;

fn get_restaurants_by_query_internal(
    base_url: &str,
    query: &str,
    lang: &str,
    hide_closed: bool,
) -> Result<Restaurants, anyhow::Error> {
    let current_date_index_in_week = chrono::offset::Local::now()
        .date_naive()
        .weekday()
        .days_since(chrono::Weekday::Mon);
    let url = format!("{}/restaurants", base_url);
    let mut restaurants = ureq::get(&url)
        .query("query", query)
        .query("lang", lang)
        .call()?
        .into_json::<Restaurants>()?
        .into_iter()
        .filter(|restaurant| {
            if hide_closed {
                let possibly_todays_opening_hours = restaurant
                    .opening_hours
                    .get(usize::try_from(current_date_index_in_week).unwrap())
                    .unwrap(); // TODO: handle
                if let Some(todays_opening_hours) = possibly_todays_opening_hours {
                    // TODO: refactor
                    // opening hours in form of "10:30-14:00"
                    let opening_hours_split = todays_opening_hours
                        .split('-')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>();
                    let possibly_start_time = opening_hours_split.first();
                    let possibly_end_time = opening_hours_split.last();
                    let current_time = Local::now().time();
                    if let Some(start_time) = possibly_start_time {
                        if let Some(end_time) = possibly_end_time {
                            let start_time_split = start_time.split(':').collect::<Vec<&str>>();
                            let end_time_split = end_time.split(':').collect::<Vec<&str>>();
                            let start_hour =
                                start_time_split.first().unwrap().parse::<u32>().unwrap();
                            let start_minute =
                                start_time_split.last().unwrap().parse::<u32>().unwrap();
                            let end_hour = end_time_split.first().unwrap().parse::<u32>().unwrap();
                            let end_minute = end_time_split.last().unwrap().parse::<u32>().unwrap();
                            if let (Some(start_time), Some(end_time)) = (
                                chrono::NaiveTime::from_hms_opt(start_hour, start_minute, 0),
                                chrono::NaiveTime::from_hms_opt(end_hour, end_minute, 0),
                            ) {
                                start_time <= current_time && current_time <= end_time
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect::<Restaurants>();
    restaurants.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(restaurants)
}

pub fn get_restaurants_by_query_filtered_by_closed_status_and_ordered_alphabetically(
    query: &str,
    lang: &str,
    hide_closed: bool,
) -> Result<Restaurants, anyhow::Error> {
    get_restaurants_by_query_internal(API_BASE_URL, query, lang, hide_closed)
}

fn get_restaurants_by_location_internal(
    base_url: &str,
    latitude: f64,
    longitude: f64,
    lang: &str,
) -> Result<Restaurants, anyhow::Error> {
    let url = format!("{}/restaurants", base_url);
    let mut restaurants = ureq::get(&url)
        .query("lat", &latitude.to_string())
        .query("lon", &longitude.to_string())
        .query("lang", lang)
        .call()?
        .into_json::<Restaurants>()?;
    // Sort by distance (closest first)
    restaurants.sort_by(|a, b| match (a.distance, b.distance) {
        (Some(d_a), Some(d_b)) => d_a.cmp(&d_b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    Ok(restaurants)
}

pub fn get_restaurants_by_location(
    latitude: f64,
    longitude: f64,
    lang: &str,
) -> Result<Restaurants, anyhow::Error> {
    get_restaurants_by_location_internal(API_BASE_URL, latitude, longitude, lang)
}

fn get_menus_by_restaurants_internal(
    base_url: &str,
    restaurants: &Restaurants,
    lang: &str,
    day_offset: i32,
    maybe_limit: Option<u16>,
) -> Result<Menus, anyhow::Error> {
    let day_to_fetch_query =
        (Local::now() + chrono::Duration::days(i64::from(day_offset))).format("%Y-%m-%d");

    let url = format!("{}/menus", base_url);
    Ok(ureq::get(&url)
        .query(
            "restaurants",
            &(match maybe_limit {
                Some(limit_u16) => {
                    let limit = usize::from(limit_u16);
                    if limit > restaurants.len() {
                        restaurants
                    } else {
                        &restaurants[..limit]
                    }
                }
                None => restaurants,
            })
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

pub fn get_menus_by_restaurants(
    restaurants: &Restaurants,
    lang: &str,
    day_offset: i32,
    maybe_limit: Option<u16>,
) -> Result<Menus, anyhow::Error> {
    get_menus_by_restaurants_internal(API_BASE_URL, restaurants, lang, day_offset, maybe_limit)
}

pub struct FormattedMenuItem {
    pub title: String,
    pub properties: String,
}

pub struct RestaurantWithMenu {
    pub name: String,
    pub opening_hours: Option<String>,
    pub address: String,
    pub url: String,
    pub distance: Option<u32>,
    pub formatted_menu_items: Option<Vec<FormattedMenuItem>>,
}

// TODO: format opening_hours properly
pub fn filter_menus_and_format_to_restaurants_with_menus(
    restaurants: &Restaurants,
    menus: &Menus,
    maybe_filter: &Option<String>,
) -> Vec<RestaurantWithMenu> {
    restaurants
        .iter()
        .map(|restaurant| -> RestaurantWithMenu {
            let menu_id = restaurant.id.to_string();
            let maybe_menu_items = menus.get(&menu_id);
            let menu_items = match maybe_menu_items {
                Some(menu_items) => {
                    let filter = match maybe_filter {
                        Some(filter_string) => filter_string.clone(),
                        None => "".to_string(),
                    };
                    let menus_items_filtered_by_title = menu_items
                        .iter()
                        .filter(|menu_item| menu_item.title.contains(&filter))
                        .map(|menu_item| FormattedMenuItem {
                            title: menu_item.title.clone(),
                            properties: menu_item.properties.join(", "),
                        })
                        .collect::<Vec<FormattedMenuItem>>();
                    Some(menus_items_filtered_by_title)
                }
                None => None,
            };
            RestaurantWithMenu {
                name: restaurant.name.clone(),
                opening_hours: restaurant.opening_hours.first().unwrap_or(&None).clone(),
                address: restaurant.address.clone(),
                url: restaurant.url.clone(),
                distance: restaurant.distance,
                formatted_menu_items: menu_items,
            }
        })
        .collect::<Vec<RestaurantWithMenu>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restaurant_distance() {
        let restaurant = Restaurant {
            id: 1,
            name: "Test Restaurant".to_string(),
            url: "https://example.com".to_string(),
            address: "Test Address".to_string(),
            opening_hours: vec![],
            distance: Some(500),
        };
        assert_eq!(restaurant.distance(), Some(500));
    }

    #[test]
    fn test_menu_item_deserialization() {
        let json = r#"{"title": "Pizza", "properties": ["vegan", "spicy"]}"#;
        let item: MenuItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.title, "Pizza");
        assert_eq!(item.properties, vec!["vegan", "spicy"]);
    }

    #[test]
    fn test_restaurant_deserialization() {
        let json = r#"{
            "id": 1,
            "name": "Test Restaurant",
            "url": "https://example.com",
            "address": "Test Address",
            "openingHours": ["10:30-14:00", null],
            "distance": 500
        }"#;
        let restaurant: Restaurant = serde_json::from_str(json).unwrap();
        assert_eq!(restaurant.id, 1);
        assert_eq!(restaurant.name, "Test Restaurant");
        assert_eq!(restaurant.distance(), Some(500));
    }

    #[test]
    fn test_get_restaurants_by_query_with_mock() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/restaurants")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("query".into(), "otaniemi".into()),
                mockito::Matcher::UrlEncoded("lang".into(), "en".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[
                {
                    "id": 1,
                    "name": "Restaurant A",
                    "url": "https://example.com/a",
                    "address": "Address A",
                    "openingHours": ["10:30-14:00", "10:30-14:00", "10:30-14:00", "10:30-14:00", "10:30-14:00", null, null]
                },
                {
                    "id": 2,
                    "name": "Restaurant B",
                    "url": "https://example.com/b",
                    "address": "Address B",
                    "openingHours": ["11:00-15:00", "11:00-15:00", "11:00-15:00", "11:00-15:00", "11:00-15:00", null, null]
                }
            ]"#,
            )
            .create();

        let result =
            get_restaurants_by_query_internal(&server.url(), "otaniemi", "en", false).unwrap();
        mock.assert();
        assert_eq!(result.len(), 2);
        // Should be sorted alphabetically
        assert_eq!(result[0].name, "Restaurant A");
        assert_eq!(result[1].name, "Restaurant B");
    }

    #[test]
    fn test_get_restaurants_by_location_with_mock() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/restaurants")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("lat".into(), "60.1695".into()),
                mockito::Matcher::UrlEncoded("lon".into(), "24.9354".into()),
                mockito::Matcher::UrlEncoded("lang".into(), "en".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"[
                {
                    "id": 1,
                    "name": "Close Restaurant",
                    "url": "https://example.com/close",
                    "address": "Nearby",
                    "openingHours": ["10:30-14:00"],
                    "distance": 100
                },
                {
                    "id": 2,
                    "name": "Far Restaurant",
                    "url": "https://example.com/far",
                    "address": "Far away",
                    "openingHours": ["11:00-15:00"],
                    "distance": 1000
                }
            ]"#,
            )
            .create();

        let result =
            get_restaurants_by_location_internal(&server.url(), 60.1695, 24.9354, "en").unwrap();
        mock.assert();
        assert_eq!(result.len(), 2);
        // Should be sorted by distance (closest first)
        assert_eq!(result[0].distance(), Some(100));
        assert_eq!(result[1].distance(), Some(1000));
    }

    #[test]
    fn test_get_menus_by_restaurants_with_mock() {
        let mut server = mockito::Server::new();
        let today = Local::now().format("%Y-%m-%d").to_string();

        let restaurants = vec![
            Restaurant {
                id: 1,
                name: "Restaurant 1".to_string(),
                url: "https://example.com/1".to_string(),
                address: "Address 1".to_string(),
                opening_hours: vec![],
                distance: None,
            },
            Restaurant {
                id: 2,
                name: "Restaurant 2".to_string(),
                url: "https://example.com/2".to_string(),
                address: "Address 2".to_string(),
                opening_hours: vec![],
                distance: None,
            },
        ];

        let mock = server
            .mock("GET", "/menus")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("restaurants".into(), "1,2".into()),
                mockito::Matcher::UrlEncoded("days".into(), today.clone()),
                mockito::Matcher::UrlEncoded("lang".into(), "en".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(format!(
                r#"{{
                "1": {{
                    "{}": [
                        {{"title": "Pizza", "properties": ["vegan"]}},
                        {{"title": "Pasta", "properties": ["vegetarian"]}}
                    ]
                }},
                "2": {{
                    "{}": [
                        {{"title": "Burger", "properties": ["beef"]}}
                    ]
                }}
            }}"#,
                today, today
            ))
            .create();

        let result =
            get_menus_by_restaurants_internal(&server.url(), &restaurants, "en", 0, None).unwrap();
        mock.assert();
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("1").unwrap().len(), 2);
        assert_eq!(result.get("2").unwrap().len(), 1);
    }

    #[test]
    fn test_get_menus_with_limit() {
        let mut server = mockito::Server::new();
        let today = Local::now().format("%Y-%m-%d").to_string();

        let restaurants = vec![
            Restaurant {
                id: 1,
                name: "Restaurant 1".to_string(),
                url: "https://example.com/1".to_string(),
                address: "Address 1".to_string(),
                opening_hours: vec![],
                distance: None,
            },
            Restaurant {
                id: 2,
                name: "Restaurant 2".to_string(),
                url: "https://example.com/2".to_string(),
                address: "Address 2".to_string(),
                opening_hours: vec![],
                distance: None,
            },
        ];

        let mock = server
            .mock("GET", "/menus")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("restaurants".into(), "1".into()),
                mockito::Matcher::UrlEncoded("days".into(), today.clone()),
                mockito::Matcher::UrlEncoded("lang".into(), "en".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(format!(
                r#"{{
                "1": {{
                    "{}": [
                        {{"title": "Pizza", "properties": []}}
                    ]
                }}
            }}"#,
                today
            ))
            .create();

        let result = get_menus_by_restaurants_internal(
            &server.url(),
            &restaurants,
            "en",
            0,
            Some(1), // Limit to 1 restaurant
        )
        .unwrap();
        mock.assert();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_filter_menus_and_format() {
        let restaurants = vec![Restaurant {
            id: 1,
            name: "Test Restaurant".to_string(),
            url: "https://example.com".to_string(),
            address: "Test Address".to_string(),
            opening_hours: vec![Some("10:30-14:00".to_string())],
            distance: Some(500),
        }];

        let mut menus = HashMap::new();
        menus.insert(
            "1".to_string(),
            vec![
                MenuItem {
                    title: "Caesar Salad".to_string(),
                    properties: vec!["vegetarian".to_string()],
                },
                MenuItem {
                    title: "Greek Salad".to_string(),
                    properties: vec!["vegan".to_string()],
                },
                MenuItem {
                    title: "Burger".to_string(),
                    properties: vec!["beef".to_string()],
                },
            ],
        );

        // Test without filter
        let result = filter_menus_and_format_to_restaurants_with_menus(&restaurants, &menus, &None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].formatted_menu_items.as_ref().unwrap().len(), 3);

        // Test with filter
        let filter = Some("Salad".to_string());
        let result =
            filter_menus_and_format_to_restaurants_with_menus(&restaurants, &menus, &filter);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].formatted_menu_items.as_ref().unwrap().len(), 2);
        assert!(result[0].formatted_menu_items.as_ref().unwrap()[0]
            .title
            .contains("Salad"));
    }

    #[test]
    fn test_filter_menus_no_menu_available() {
        let restaurants = vec![Restaurant {
            id: 1,
            name: "Test Restaurant".to_string(),
            url: "https://example.com".to_string(),
            address: "Test Address".to_string(),
            opening_hours: vec![Some("10:30-14:00".to_string())],
            distance: None,
        }];

        let menus = HashMap::new(); // No menus available

        let result = filter_menus_and_format_to_restaurants_with_menus(&restaurants, &menus, &None);
        assert_eq!(result.len(), 1);
        assert!(result[0].formatted_menu_items.is_none());
    }

    #[test]
    fn test_get_restaurants_api_error() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/restaurants")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let result = get_restaurants_by_query_internal(&server.url(), "test", "en", false);
        mock.assert();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_menus_api_error() {
        let mut server = mockito::Server::new();
        let restaurants = vec![Restaurant {
            id: 1,
            name: "Test".to_string(),
            url: "https://example.com".to_string(),
            address: "Address".to_string(),
            opening_hours: vec![],
            distance: None,
        }];

        let mock = server
            .mock("GET", "/menus")
            .match_query(mockito::Matcher::Any)
            .with_status(404)
            .with_body("Not Found")
            .create();

        let result = get_menus_by_restaurants_internal(&server.url(), &restaurants, "en", 0, None);
        mock.assert();
        assert!(result.is_err());
    }
}
