use chrono::{Datelike, Local};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Restaurant {
    #[serde(rename = "openingHours")]
    opening_hours: Vec<Option<String>>,
    id: u8,
    name: String,
    url: String,
    address: String,
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

pub fn get_restaurants_by_query_filtered_by_closed_status_and_ordered_alphabetically(
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
                            let start_time =
                                chrono::NaiveTime::from_hms(start_hour, start_minute, 0);
                            let end_time = chrono::NaiveTime::from_hms(end_hour, end_minute, 0);
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
                true
            }
        })
        .collect::<Restaurants>();
    restaurants.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(restaurants)
}

pub fn get_menus_by_restaurants(
    restaurants: &Restaurants,
    lang: &str,
    day_offset: i32,
    maybe_limit: Option<u16>,
) -> Result<Menus, anyhow::Error> {
    let day_to_fetch_query =
        (Local::now() + chrono::Duration::days(i64::from(day_offset))).format("%Y-%m-%d");

    Ok(ureq::get("https://kitchen.kanttiinit.fi/menus")
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

pub struct FormattedMenuItem {
    pub title: String,
    pub properties: String,
}

pub struct RestaurantWithMenu {
    pub name: String,
    pub opening_hours: Option<String>,
    pub address: String,
    pub url: String,
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
                formatted_menu_items: menu_items,
            }
        })
        .collect::<Vec<RestaurantWithMenu>>()
}
