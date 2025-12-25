use chrono::{Datelike, Local};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
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
            if !hide_closed {
                return true;
            }
            is_restaurant_open_now(&restaurant.opening_hours, current_date_index_in_week)
        })
        .collect::<Restaurants>();
    restaurants.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(restaurants)
}

pub fn get_menus_by_restaurants(
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
    restaurants: &[Restaurant],
    menus: &Menus,
    maybe_filter: &Option<String>,
) -> Vec<RestaurantWithMenu> {
    restaurants
        .iter()
        .map(|restaurant| -> RestaurantWithMenu {
            let menu_id = restaurant.id.to_string();
            let maybe_menu_items = menus.get(&menu_id);
            let menu_items = maybe_menu_items.map(|menu_items| {
                let filter = maybe_filter.as_deref().unwrap_or("");
                menu_items
                    .iter()
                    .filter(|menu_item| menu_item.title.contains(filter))
                    .map(|menu_item| FormattedMenuItem {
                        title: menu_item.title.clone(),
                        properties: menu_item.properties.join(", "),
                    })
                    .collect::<Vec<FormattedMenuItem>>()
            });
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
