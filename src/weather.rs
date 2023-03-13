use crate::model::WeatherResponse;

#[derive(Default)]
pub struct WeatherClient {
    client: Option<reqwest::Client>,
}

const API_KEY: &str = "[redacted]";

impl WeatherClient {
    pub fn init_client(&mut self) -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;
        self.client = Some(client);
        Ok(())
    }

    pub async fn by_lat_long(&self, lat: f64, long: f64) -> anyhow::Result<WeatherResponse> {
        if self.client.is_none() {
            return Err(anyhow::anyhow!("Client not initialized"));
        }

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            lat, long, API_KEY
        );
        let res = reqwest::get(url).await?.text().await?;

        serde_json::from_str::<WeatherResponse>(res.as_str()).map_err(|err| anyhow::anyhow!(err))
    }
}
