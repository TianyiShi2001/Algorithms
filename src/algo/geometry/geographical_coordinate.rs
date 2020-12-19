#[derive(Debug, Copy, Clone)]
pub struct GeographicalCoordinate {
    longitude: f64,
    latitude: f64,
}

impl GeographicalCoordinate {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
    /// See: <https://en.wikipedia.org/wiki/Haversine_formula>
    pub fn distance(self, other: Self) -> f64 {
        const EARTH_RADIUS: f64 = 6378.1; // kilometers

        let (lat1, lon1) = (self.latitude.to_radians(), self.longitude.to_radians());
        let (lat2, lon2) = (other.latitude.to_radians(), other.longitude.to_radians());

        let delta_lat = lat2 - lat1;
        let delta_lon = lon2 - lon1;
        let x = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        2.0 * EARTH_RADIUS * x.sqrt().atan()
    }
}
