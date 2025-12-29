//! Test fixtures for mocking API responses.
//!
//! This module provides sample data structures that mirror the Kanttiinit API responses.
//! Edit these fixtures to update test data as the API evolves.

use crate::search::{MenuItem, Restaurant};
use std::collections::HashMap;

/// Sample restaurants that would be returned from the API.
/// Opening hours format: "HH:MM-HH:MM" for each day (Mon-Sun).
/// None means closed on that day.
pub fn sample_restaurants() -> Vec<Restaurant> {
    vec![
        Restaurant::new(
            1,
            "Aalto Yliopiston ravintola".to_string(),
            "https://example.com/aalto".to_string(),
            "Otakaari 1, Espoo".to_string(),
            vec![
                Some("10:30-13:30".to_string()), // Mon
                Some("10:30-13:30".to_string()), // Tue
                Some("10:30-13:30".to_string()), // Wed
                Some("10:30-13:30".to_string()), // Thu
                Some("10:30-13:30".to_string()), // Fri
                None,                            // Sat
                None,                            // Sun
            ],
        ),
        Restaurant::new(
            2,
            "Unicafe Kaivopiha".to_string(),
            "https://example.com/unicafe".to_string(),
            "Kaivokatu 10, Helsinki".to_string(),
            vec![
                Some("08:00-15:00".to_string()), // Mon
                Some("08:00-15:00".to_string()), // Tue
                Some("08:00-15:00".to_string()), // Wed
                Some("08:00-15:00".to_string()), // Thu
                Some("08:00-15:00".to_string()), // Fri
                Some("10:00-14:00".to_string()), // Sat
                None,                            // Sun
            ],
        ),
        Restaurant::new(
            3,
            "Ravintola Factory".to_string(),
            "https://example.com/factory".to_string(),
            "Betonimiehenkuja 5, Espoo".to_string(),
            vec![
                Some("10:00-14:00".to_string()), // Mon
                Some("10:00-14:00".to_string()), // Tue
                Some("10:00-14:00".to_string()), // Wed
                Some("10:00-14:00".to_string()), // Thu
                Some("10:00-14:00".to_string()), // Fri
                None,                            // Sat
                None,                            // Sun
            ],
        ),
    ]
}

/// Sample menus keyed by restaurant ID, then by date.
/// The outer key is the restaurant ID as a string.
/// The value is a list of menu items for the requested date.
pub fn sample_menus() -> HashMap<String, Vec<MenuItem>> {
    let mut menus = HashMap::new();

    menus.insert(
        "1".to_string(),
        vec![
            MenuItem::new(
                "Lohikeitto".to_string(),
                vec!["G".to_string(), "L".to_string()],
            ),
            MenuItem::new(
                "Kasvislaatikko".to_string(),
                vec!["VEG".to_string(), "G".to_string()],
            ),
            MenuItem::new(
                "Chicken salad".to_string(),
                vec!["G".to_string(), "L".to_string()],
            ),
        ],
    );

    menus.insert(
        "2".to_string(),
        vec![
            MenuItem::new("Pasta Bolognese".to_string(), vec!["L".to_string()]),
            MenuItem::new(
                "Vegaaninen curry".to_string(),
                vec!["VEG".to_string(), "G".to_string()],
            ),
        ],
    );

    menus.insert(
        "3".to_string(),
        vec![MenuItem::new(
            "Grilled chicken with rice".to_string(),
            vec!["G".to_string(), "L".to_string()],
        )],
    );

    menus
}

/// Restaurant with no opening hours set (edge case).
pub fn restaurant_without_hours() -> Restaurant {
    Restaurant::new(
        99,
        "Mystery Restaurant".to_string(),
        "https://example.com/mystery".to_string(),
        "Unknown Address".to_string(),
        vec![],
    )
}

/// Restaurant with malformed opening hours (edge case).
pub fn restaurant_with_malformed_hours() -> Restaurant {
    Restaurant::new(
        98,
        "Broken Hours Restaurant".to_string(),
        "https://example.com/broken".to_string(),
        "Broken Street 1".to_string(),
        vec![
            Some("invalid".to_string()),     // Mon - invalid format
            Some("10:00".to_string()),       // Tue - missing end time
            Some("10:00-".to_string()),      // Wed - missing end time value
            Some("-14:00".to_string()),      // Thu - missing start time
            Some("25:00-26:00".to_string()), // Fri - invalid time values
            Some("10:00-14:00".to_string()), // Sat - valid
            None,                            // Sun
        ],
    )
}

/// Empty menu response (no menu available for the day).
pub fn empty_menus() -> HashMap<String, Vec<MenuItem>> {
    HashMap::new()
}
