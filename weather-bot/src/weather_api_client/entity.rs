use serde::Deserialize;

#[derive(Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct Condition {
    pub code: i32,
    pub text: String,
}

#[derive(Deserialize)]
pub struct Day {
    pub maxtemp_c: f32,
    pub mintemp_c: f32,
    pub avgtemp_c: f32,
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: String,
}

#[derive(Deserialize)]
pub struct HourItem {
    pub time: String,
    pub time_epoch: i64,
    pub temp_c: f32,
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct ForecastdayItem {
    pub date: String,
    pub date_epoch: i64,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<HourItem>,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastdayItem>,
}

#[derive(Deserialize)]
pub struct WeatherForecastResponse {
    pub location: Location,
    pub forecast: Forecast,
}
