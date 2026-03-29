use std::{collections::HashMap, error::Error, string::ParseError, time::Duration};

use polars::frame::DataFrame;
use reqwest::{
    Client, Url,
    header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue},
};
use thiserror::Error;

use crate::weather_api_client::response::WeatherForecastResponse;

pub struct WeatherApiClient {
    api_endpoint: String,
    http_client: Client,
}

#[derive(Debug,Error)]
pub enum WeatherApiClientError{
    #[error("http client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    #[error("invalid header value: {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
    #[error("url parse error: {0}")]
    UrlParseError(String),
}

impl WeatherApiClient {
    pub fn new(
        api_endpoint: &str,
        timeout_seconds: u64,
    ) -> Result<Self, WeatherApiClientError> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let client_builder = Client::builder();
        let http_client = client_builder
            .timeout(Duration::from_secs(timeout_seconds))
            .build()?;
        Ok(WeatherApiClient {
            api_endpoint: api_endpoint.to_string(),
            http_client,
        })
    }

    pub async fn get_weather_forecast(
        &self,
        weather_api_key: &str,
        q: &str,
        days: i32,
    ) -> Result<WeatherForecastResponse, WeatherApiClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::try_from("key").unwrap(),
            HeaderValue::from_str(weather_api_key)?,
        );

        let params = HashMap::from([("q", q.to_string()), ("days", days.to_string())]);

        let url =match Url::parse_with_params(&self.api_endpoint, &params){
            Ok(v)=>Ok(v),
            Err(e)=>Err(WeatherApiClientError::UrlParseError(e.to_string())),
        }?;
        let request_builder = self.http_client.get(url);
        let response = request_builder
            .headers(headers)
            .send()
            .await?
            .json::<WeatherForecastResponse>()
            .await?;

        Ok(response)
    }
}
