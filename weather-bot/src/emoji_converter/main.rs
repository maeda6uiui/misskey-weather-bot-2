use std::{
    error::{self, Error},
    path::Path,
    sync::Once,
};

use polars::{
    frame::DataFrame,
    io::SerReader,
    prelude::{CsvReadOptions, DataType, Field, IntoLazy, Schema, col, lit},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmojiConverterError {
    #[error("no matching emoji found")]
    NoMatchingEmojiFound,
    #[error("polars error: {0}")]
    Polars(#[from] polars::error::PolarsError),
}

pub fn load_emoji_csv(csv_path: &Path) -> Result<DataFrame, EmojiConverterError> {
    let schema = Schema::from_iter(vec![
        Field::new("code".into(), DataType::Int32),
        Field::new("day".into(), DataType::String),
        Field::new("night".into(), DataType::String),
        Field::new("icon".into(), DataType::Int32),
        Field::new("emoji".into(), DataType::String),
    ]);
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .with_schema(Some(schema.into()))
        .try_into_reader_with_file_path(Some(csv_path.into()))?
        .finish()?;

    Ok(df)
}

pub fn get_condition_emoji(
    df: &DataFrame,
    condition_code: i32,
) -> Result<String, EmojiConverterError> {
    let df_result = df
        .clone()
        .lazy()
        .filter(col("code").eq(lit(condition_code)))
        .collect()?;
    let emoji_series = df_result.column("emoji")?;
    if emoji_series.len()==0{
        return Err(EmojiConverterError::NoMatchingEmojiFound);
    }

    let emoji = emoji_series
        .str()?
        .get(0)
        .unwrap();
    Ok(emoji.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use polars::df;

    static DF: Lazy<DataFrame> = Lazy::new(|| {
        df![
            "code"=>&[100,200],
            "emoji"=>&["☀", "☂"],
        ]
        .unwrap()
    });

    #[test]
    fn get_condition_emoji_success() {
        let emoji_sunny = get_condition_emoji(&DF, 100).unwrap();
        assert_eq!(emoji_sunny, "☀");

        let emoji_rain = get_condition_emoji(&DF, 200).unwrap();
        assert_eq!(emoji_rain, "☂");
    }

    #[test]
    fn get_condition_emoji_error() {
        let result = get_condition_emoji(&DF, -1);
        assert!(matches!(
            result,
            Err(EmojiConverterError::NoMatchingEmojiFound)
        ))
    }
}
