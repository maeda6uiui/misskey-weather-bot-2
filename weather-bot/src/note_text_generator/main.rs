use std::path::Path;

use polars::frame::DataFrame;
use thiserror::Error;

use crate::{
    emoji_converter::main::{EmojiConverterError, get_condition_emoji, load_emoji_csv},
    weather_api_client::entity::WeatherForecastResponse,
};

pub struct NoteTextGenerator {
    df_emoji: DataFrame,
}

#[derive(Debug, Error)]
pub enum NoteTextGeneratorError {
    #[error("emoji converter error: {0}")]
    EmojiConverterError(#[from] EmojiConverterError),
}

impl NoteTextGenerator {
    pub fn new(emoji_csv_path: &Path) -> Result<Self, NoteTextGeneratorError> {
        let df_emoji = load_emoji_csv(emoji_csv_path)?;
        Ok(NoteTextGenerator { df_emoji })
    }

    pub fn get_daily_forecast_text(
        &self,
        forecast: &WeatherForecastResponse,
    ) -> Result<String, NoteTextGeneratorError> {
        let location = &forecast.location.name;
        let date = &forecast.forecast.forecastday[0].date;

        let astro = &forecast.forecast.forecastday[0].astro;
        let sunrise = &astro.sunrise;
        let sunset = &astro.sunset;
        let moonrise = &astro.moonrise;
        let moonset = &astro.moonset;
        let moon_phase = &astro.moon_phase;

        let daily_forecast = &forecast.forecast.forecastday[0].day;
        let condition_code = daily_forecast.condition.code;
        let condition_text = &daily_forecast.condition.text;
        let condition_emoji = get_condition_emoji(&self.df_emoji, condition_code)?;
        let avgtemp_c = daily_forecast.avgtemp_c;
        let mintemp_c = daily_forecast.mintemp_c;
        let maxtemp_c = daily_forecast.maxtemp_c;

        let text = indoc::formatdoc! {
            r#"
            [{date}] Weather forcast in {location}
            {condition_emoji}{condition_text}
            {avgtemp_c} ℃ (avg) / {mintemp_c} ℃ (min) / {maxtemp_c} ℃ (max)
            ---
            🌄 {sunrise} - {sunset}
            🌕 {moonrise} - {moonset} / {moon_phase}
            "#
        };
        Ok(text.to_string())
    }

    pub fn get_hourly_forecast_text(
        &self,
        forecast: &WeatherForecastResponse,
    ) -> Result<String, NoteTextGeneratorError> {
        let location = &forecast.location.name;
        let date = &forecast.forecast.forecastday[0].date;
        let mut text = format!("[{date}] Hourly weather forecast in {location}\n\n");

        let hourly_forecast = &forecast.forecast.forecastday[0].hour;
        for v in hourly_forecast.iter() {
            let time = &v.time;
            let time_splits: Vec<&str> = time.split(" ").collect();
            let time = time_splits[1];

            let temp_c = v.temp_c;
            let condition_code = v.condition.code;
            let condition_text = &v.condition.text;
            let condition_emoji = get_condition_emoji(&self.df_emoji, condition_code)?;

            let line = format!("{time} / {temp_c} ℃ / {condition_emoji}{condition_text}\n");
            text.push_str(&line);
        }

        Ok(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather_api_client::entity::{
        Astro, Condition, Day, Forecast, ForecastdayItem, HourItem, Location,
    };
    use once_cell::sync::Lazy;
    use polars::prelude::streaming;

    // ----------------------------
    // Helpers
    // ----------------------------
    fn sample_location() -> Location {
        Location {
            name: "Tokyo".to_string(),
            region: "Tokyo".to_string(),
            country: "Japan".to_string(),
        }
    }

    fn sample_forecast(
        date: &str,
        day_condition_code: i32,
        day_condition_text: &str,
        avg: f32,
        min: f32,
        max: f32,
        sunrise: &str,
        sunset: &str,
        moonrise: &str,
        moonset: &str,
        moon_phase: &str,
        hours: Vec<HourItem>,
    ) -> WeatherForecastResponse {
        WeatherForecastResponse {
            location: sample_location(),
            forecast: Forecast {
                forecastday: vec![ForecastdayItem {
                    date: date.to_string(),
                    date_epoch: 0,
                    day: Day {
                        maxtemp_c: max,
                        mintemp_c: min,
                        avgtemp_c: avg,
                        condition: Condition {
                            code: day_condition_code,
                            text: day_condition_text.to_string(),
                        },
                    },
                    astro: Astro {
                        sunrise: sunrise.to_string(),
                        sunset: sunset.to_string(),
                        moonrise: moonrise.to_string(),
                        moonset: moonset.to_string(),
                        moon_phase: moon_phase.to_string(),
                    },
                    hour: hours,
                }],
            },
        }
    }

    fn sample_hour(datetime: &str, temp: f32, code: i32, text: &str) -> HourItem {
        HourItem {
            time: datetime.to_string(),
            time_epoch: 0,
            temp_c: temp,
            condition: Condition {
                code,
                text: text.to_string(),
            },
        }
    }

    // ----------------------------
    // Tests
    // ----------------------------
    static GENERATOR: Lazy<NoteTextGenerator> =
        Lazy::new(|| NoteTextGenerator::new(Path::new("./Data/weather_conditions.csv")).unwrap());

    #[test]
    fn get_daily_forecast_text() {
        let forecast = sample_forecast(
            "2026-03-29",
            1000,
            "Sunny",
            15.0,
            10.0,
            20.0,
            "05:33 AM",
            "06:00 PM",
            "02:08 PM",
            "03:26 AM",
            "Waxing Gibbous",
            vec![],
        );
        let emoji = get_condition_emoji(&GENERATOR.df_emoji, 1000).unwrap();
        let text = GENERATOR.get_daily_forecast_text(&forecast).unwrap();
        let expected = indoc::formatdoc! {
            r#"
            [2026-03-29] Weather forcast in Tokyo
            {emoji}Sunny
            15 ℃ (avg) / 10 ℃ (min) / 20 ℃ (max)
            ---
            🌄 05:33 AM - 06:00 PM
            🌕 02:08 PM - 03:26 AM / Waxing Gibbous
            "#
        };
        assert_eq!(text, expected);
    }

    #[test]
    fn get_hourly_forecast_text() {
        let forecast = sample_forecast(
            "2026-03-29",
            1000,
            "Sunny",
            0.0,
            0.0,
            0.0,
            "05:33 AM",
            "06:00 PM",
            "02:08 PM",
            "03:26 AM",
            "Waxing Gibbous",
            vec![
                sample_hour("2026-03-29 09:00", 12.0, 1000, "Sunny"),
                sample_hour("2026-03-29 12:00", 18.0, 1000, "Sunny"),
            ],
        );
        let emoji = get_condition_emoji(&GENERATOR.df_emoji, 1000).unwrap();
        let text = GENERATOR.get_hourly_forecast_text(&forecast).unwrap();
        let expected = format!(
            "[2026-03-29] Hourly weather forecast in Tokyo\n\n\
             09:00 / 12 ℃ / {emoji}Sunny\n\
             12:00 / 18 ℃ / {emoji}Sunny\n"
        );
        assert_eq!(text, expected);
    }
}
