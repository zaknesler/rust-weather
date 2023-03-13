use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherResponse {
    #[serde(rename = "id")]
    pub city_id: Option<i64>,
    #[serde(rename = "name")]
    pub city_name: Option<String>,
    pub timezone: i64,
    #[serde(rename = "dt")]
    pub measured_at_unix: i64,
    pub cod: i64,
    pub visibility: i64,
    pub sys: Sys,
    pub coord: Coordinates,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub wind: Option<Wind>,
    pub rain: Option<RecentWeather>,
    pub snow: Option<RecentWeather>,
    pub clouds: Option<Clouds>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    #[serde(rename = "lon")]
    pub long: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

impl Weather {
    pub fn get_emoji(&self) -> Option<&str> {
        match self.icon.as_str() {
            "01n" | "01d" => Some("☀️"),
            "02n" | "02d" => Some("🌤️"),
            "03n" | "03d" => Some("☁️"),
            "04n" | "04d" => Some("☁️"),
            "09n" | "09d" => Some("🌧️"),
            "10n" | "10d" => Some("🌧️"),
            "11n" | "11d" => Some("⛈️"),
            "13n" | "13d" => Some("❄️"),
            "50n" | "50d" => Some("🌫️"),
            _ => None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Main {
    pub temp: Option<f64>,
    pub feels_like: Option<f64>,
    pub temp_min: Option<f64>,
    pub temp_max: Option<f64>,
    pub pressure: Option<i64>,
    pub humidity: Option<i64>,
    pub sea_level: Option<i64>,
    pub grnd_level: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub speed: Option<f64>,
    pub deg: Option<i64>,
    pub gust: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecentWeather {
    #[serde(rename = "1h")]
    pub past_1hr: Option<f64>,
    #[serde(rename = "3h")]
    pub past_3hr: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clouds {
    /// Cloudiness %
    pub all: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub id: Option<i64>,
    pub country: Option<String>,
    pub sunrise: i64,
    pub sunset: i64,
}
