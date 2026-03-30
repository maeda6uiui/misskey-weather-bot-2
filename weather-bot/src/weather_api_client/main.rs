use std::{collections::HashMap, time::Duration};

use reqwest::{
    Client, Url,
    header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue},
};
use thiserror::Error;

use crate::weather_api_client::entity::WeatherForecastResponse;

pub struct WeatherApiClient {
    api_endpoint: String,
    access_token: String,
    http_client: Client,
}

#[derive(Debug, Error)]
pub enum WeatherApiClientError {
    #[error("http client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    #[error("invalid header value: {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
    #[error("url parse error: {0}")]
    UrlParseError(String),
}

impl WeatherApiClient {
    pub fn new(api_endpoint: &str, access_token: &str) -> Result<Self, WeatherApiClientError> {
        let http_client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(WeatherApiClient {
            api_endpoint: api_endpoint.to_string(),
            access_token: access_token.to_string(),
            http_client,
        })
    }

    pub async fn get_weather_forecast(
        &self,
        q: &str,
        days: i32,
    ) -> Result<WeatherForecastResponse, WeatherApiClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            HeaderName::try_from("key").unwrap(),
            HeaderValue::from_str(&self.access_token)?,
        );

        let params = HashMap::from([("q", q.to_string()), ("days", days.to_string())]);

        let url = match Url::parse_with_params(&self.api_endpoint, &params) {
            Ok(v) => Ok(v),
            Err(e) => Err(WeatherApiClientError::UrlParseError(e.to_string())),
        }?;
        let response = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json::<WeatherForecastResponse>()
            .await?;

        Ok(response)
    }
}
