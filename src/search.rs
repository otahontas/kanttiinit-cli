use chrono::Local;
use color_print::cprintln;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Restaurant {
    #[serde(rename = "openingHours")]
    opening_hours: Vec<Option<String>>,
    id: u8,
    name: String,
    url: String,
    address: String,
}

type Restaurants = Vec<Restaurant>;

#[derive(Debug, Deserialize)]
struct MenuItem {
    title: String,
    properties: Vec<String>,
}

type DailyMenu = HashMap<String, Vec<MenuItem>>;

type MenuId = String;

type Menus = HashMap<MenuId, DailyMenu>;

struct FormattedMenuItem {
    title: String,
    properties: String,
}

struct RestaurantWithMenu {
    name: String,
    opening_hours: Option<String>,
    address: String,
    url: String,
    formatted_menu_items: Vec<FormattedMenuItem>,
}

pub fn search_by_query(query: &str, lang: &str, day_offset: i32) -> Result<(), anyhow::Error> {
    let restaurants = ureq::get("https://kitchen.kanttiinit.fi/restaurants")
        .query("query", query)
        .query("lang", lang)
        .call()?
        .into_json::<Restaurants>()?;
    // TODO: filter the first n restaurants if number is provided
    // TODO: filter out closed restaurants
    let restaurant_ids_as_comma_separated_string = restaurants
        .iter()
        .map(|r| r.id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let local_time = Local::now();
    let day_offset_casted = i64::from(day_offset);
    let new_time = local_time + chrono::Duration::days(day_offset_casted);
    let formatted_new_time = new_time.format("%Y-%m-%d");
    let menu = ureq::get("https://kitchen.kanttiinit.fi/menus")
        .query("restaurants", &restaurant_ids_as_comma_separated_string)
        .query("days", &formatted_new_time.to_string())
        .query("lang", lang)
        .call()?
        .into_json::<Menus>()?;

    let restaurant_with_menus =
        restaurants
            .iter()
            .map(|restaurant| -> Option<RestaurantWithMenu> {
                let menu_id = restaurant.id.to_string();
                let daily_menu = menu.get(&menu_id); // TODO: error handling
                let parsed_menu = match daily_menu {
                    Some(m) => m,
                    None => return None,
                };
                let menu_items = parsed_menu.get(&formatted_new_time.to_string());
                let parsed_menu_items = match menu_items {
                    Some(m) => m,
                    None => return None,
                };

                Some(RestaurantWithMenu {
                    name: restaurant.name.clone(),
                    opening_hours: restaurant.opening_hours.first().unwrap().clone(), // TODO: error handling
                    address: restaurant.address.clone(),
                    url: restaurant.url.clone(),
                    formatted_menu_items: parsed_menu_items
                        .iter()
                        .map(|menu_item| FormattedMenuItem {
                            title: menu_item.title.clone(),
                            properties: menu_item.properties.join(", "),
                        })
                        .collect(),
                })
            });
    for restaurant_with_menu in restaurant_with_menus {
        let restaurant_with_menu = match restaurant_with_menu {
            Some(r) => r,
            None => continue,
        };
        cprintln!(
            "<bold>{}</> <dim>{}</>",
            restaurant_with_menu.name,
            restaurant_with_menu
                .opening_hours
                .unwrap_or("N/A".to_string())
        );
        cprintln!("<dim>{}</>", restaurant_with_menu.address);
        cprintln!("<dim>{}</>", restaurant_with_menu.url);
        for formatted_menu_item in restaurant_with_menu.formatted_menu_items {
            cprintln!(
                "- {} <dim>{}</>",
                formatted_menu_item.title,
                formatted_menu_item.properties
            )
        }
        println!();
    }
    Ok(())
}
