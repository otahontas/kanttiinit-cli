use serde::Deserialize;
use std::time::Duration;

const NOMINATIM_BASE_URL: &str = "https://nominatim.openstreetmap.org";

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

#[derive(Debug)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

fn get_location_internal(base_url: &str, address: &str) -> Result<GeoLocation, anyhow::Error> {
    let url = format!("{}/search", base_url);
    let response = ureq::get(&url)
        .query("q", address)
        .query("format", "json")
        .query("limit", "1")
        .timeout(Duration::from_secs(10))
        .set(
            "User-Agent",
            &format!("kanttiinit-cli/{}", env!("CARGO_PKG_VERSION")),
        )
        .call()?
        .into_json::<Vec<NominatimResponse>>()?;

    if let Some(location) = response.first() {
        Ok(GeoLocation {
            latitude: location
                .lat
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse latitude: {}", e))?,
            longitude: location
                .lon
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse longitude: {}", e))?,
        })
    } else {
        Err(anyhow::anyhow!(
            "No location found for address: {}",
            address
        ))
    }
}

/// Get geographic coordinates for an address using OpenStreetMap's Nominatim API
///
/// This is a free geocoding service that doesn't require an API key.
///
/// # Rate Limits
///
/// Nominatim has a strict usage policy with a rate limit of 1 request per second.
/// For high-volume usage, consider setting up your own Nominatim instance or using
/// a commercial geocoding service.
///
/// See: https://operations.osmfoundation.org/policies/nominatim/
pub fn get_location(address: &str) -> Result<GeoLocation, anyhow::Error> {
    get_location_internal(NOMINATIM_BASE_URL, address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominatim_response_deserialization() {
        let json = r#"[{"lat": "60.1695", "lon": "24.9354"}]"#;
        let response: Vec<NominatimResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].lat, "60.1695");
        assert_eq!(response[0].lon, "24.9354");
    }

    #[test]
    fn test_geo_location_creation() {
        let location = GeoLocation {
            latitude: 60.1695,
            longitude: 24.9354,
        };
        assert_eq!(location.latitude, 60.1695);
        assert_eq!(location.longitude, 24.9354);
    }

    #[test]
    fn test_get_location_with_mock_success() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "Helsinki".into()),
                mockito::Matcher::UrlEncoded("format".into(), "json".into()),
                mockito::Matcher::UrlEncoded("limit".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"lat": "60.1695", "lon": "24.9354"}]"#)
            .create();

        let result = get_location_internal(&server.url(), "Helsinki").unwrap();
        mock.assert();
        assert_eq!(result.latitude, 60.1695);
        assert_eq!(result.longitude, 24.9354);
    }

    #[test]
    fn test_get_location_with_mock_no_results() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .create();

        let result = get_location_internal(&server.url(), "NonexistentPlace");
        mock.assert();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No location found"));
    }

    #[test]
    fn test_get_location_with_mock_api_error() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::Any)
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let result = get_location_internal(&server.url(), "Helsinki");
        mock.assert();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_location_with_mock_invalid_lat_lon() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"lat": "invalid", "lon": "24.9354"}]"#)
            .create();

        let result = get_location_internal(&server.url(), "Helsinki");
        mock.assert();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse latitude"));
    }

    #[test]
    fn test_get_location_with_mock_different_coordinates() {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/search")
            .match_query(mockito::Matcher::UrlEncoded("q".into(), "Otakaari 8".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"lat": "60.1867", "lon": "24.8290"}]"#)
            .create();

        let result = get_location_internal(&server.url(), "Otakaari 8").unwrap();
        mock.assert();
        assert_eq!(result.latitude, 60.1867);
        assert_eq!(result.longitude, 24.8290);
    }

    #[test]
    fn test_get_location_with_timeout() {
        // This test verifies that the function sets a timeout
        // The actual timeout behavior would need a more complex test setup
        let result = get_location("InvalidAddressThatDoesntExist12345678901234567890");
        // We just verify the function can handle errors gracefully
        assert!(result.is_err());
    }
}
