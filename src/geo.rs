use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoLocation {
    /// Calculate distance in meters between two geographic coordinates using Haversine formula
    pub fn distance_to(&self, other_lat: f64, other_lon: f64) -> u32 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1 = self.latitude.to_radians();
        let lat2 = other_lat.to_radians();
        let delta_lat = (other_lat - self.latitude).to_radians();
        let delta_lon = (other_lon - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        let distance_km = EARTH_RADIUS_KM * c;
        (distance_km * 1000.0).round() as u32
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
    let response = ureq::get("https://nominatim.openstreetmap.org/search")
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
}
