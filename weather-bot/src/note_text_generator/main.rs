use std::path::Path;

use polars::frame::DataFrame;
use thiserror::Error;

use crate::{emoji_converter::main::{EmojiConverterError, get_condition_emoji, load_emoji_csv}, weather_api_client::entity::WeatherForecastResponse};

pub struct NoteTextGenerator{
    df_emoji:DataFrame
}

#[derive(Debug,Error)]
pub enum NoteTextGeneratorError{
    #[error("emoji converter error: {0}")]
    EmojiConverterError(#[from] EmojiConverterError)
}

impl NoteTextGenerator{
    pub fn new(emoji_csv_path:&Path)->Result<Self,NoteTextGeneratorError>{
        let df_emoji=load_emoji_csv(emoji_csv_path)?;
        Ok(NoteTextGenerator { df_emoji })
    }

    pub fn get_daily_forecast_text(&self,forecast:&WeatherForecastResponse)->Result<String,NoteTextGeneratorError>{
        let location=forecast.location.name.to_string();
        
        let date=forecast.forecast.forecastday[0].date.to_string();
        let daily_forecast=&forecast.forecast.forecastday[0].day;
        let condition_code=daily_forecast.condition.code;
        let condition_text=daily_forecast.condition.text.to_string();
        let condition_emoji=get_condition_emoji(&self.df_emoji, condition_code)?;
        let avgtemp_c=daily_forecast.avgtemp_c;
        let mintemp_c=daily_forecast.mintemp_c;
        let maxtemp_c=daily_forecast.maxtemp_c;

        let text=format!(
            r#"
            [{date}] Weather forcast in {location}
            {condition_emoji}{condition_text}
            {avgtemp_c} ℃ (avg) / {mintemp_c} ℃ (min) / {maxtemp_c} ℃ (max)
            "#
        );
        Ok(text.to_string())
    }
}