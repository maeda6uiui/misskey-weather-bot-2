use std::path::Path;

use lambda_runtime::{LambdaEvent, service_fn};
use serde_json::Value;

use crate::{
    config::main::Config,
    misskey_client::{entity::NoteVisibility, main::MisskeyClient},
    note_text_generator::main::NoteTextGenerator,
    weather_api_client::main::WeatherApiClient,
};

mod aws;
mod config;
mod emoji_converter;
mod misskey_client;
mod note_text_generator;
mod weather_api_client;

async fn function_handler(_: LambdaEvent<Value>) -> Result<(), lambda_runtime::Error> {
    //Set up logger
    env_logger::init();

    //Load config
    let config = Config::new(None).await?;

    //Get weather forecast
    let weather_api_client = WeatherApiClient::new(
        &config.weather_api_endpoint,
        &config.weather_api_access_token,
    )?;
    let weather_forecast = weather_api_client
        .get_weather_forecast(&config.weather_api_query, config.weather_api_days)
        .await?;

    //Get text from the weather forecast
    let note_text_generator = NoteTextGenerator::new(Path::new(&config.emoji_csv_filepath))?;
    let daily_forecast_text = note_text_generator.get_daily_forecast_text(&weather_forecast)?;
    let hourly_forecast_text = note_text_generator.get_hourly_forecast_text(&weather_forecast)?;

    //Create notes on Misskey
    let misskey_client =
        MisskeyClient::new(&config.misskey_server_url, &config.misskey_access_token)?;
    let misskey_resp = misskey_client
        .create_note(&daily_forecast_text, NoteVisibility::Direct(Vec::new()))
        .await?;
    log::info!(
        "Note ID of daily forecast: {}",
        &misskey_resp.created_note.id
    );

    let misskey_resp = misskey_client
        .create_note(&hourly_forecast_text, NoteVisibility::Direct(Vec::new()))
        .await?;
    log::info!(
        "Note ID of hourly forecast: {}",
        &misskey_resp.created_note.id
    );

    Ok(())
}

#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    function_handler(LambdaEvent::new(serde_json::json!({}), Default::default())).await
}

#[cfg(feature = "default")]
#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(function_handler)).await
}
