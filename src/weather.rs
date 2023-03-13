use crate::model::WeatherResponse;

#[derive(Default)]
pub struct WeatherClient {
    api_key: String,
    client: Option<reqwest::Client>,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

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
            lat, long, self.api_key
        );
        let res = reqwest::get(url).await?.text().await?;

        tracing::debug!("Weather response: {}", res);

        serde_json::from_str::<WeatherResponse>(res.as_str()).map_err(|err| anyhow::anyhow!(err))
    }
}
