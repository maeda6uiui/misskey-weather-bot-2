use std::{
    error::{self, Error},
    path::Path,
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
}

pub fn load_emoji_csv(csv_path: &Path) -> Result<DataFrame, Box<dyn Error>> {
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

pub fn get_condition_emoji(df: &DataFrame, condition_code: i32) -> Result<String, Box<dyn Error>> {
    let df_result = df
        .clone()
        .lazy()
        .filter(col("code").eq(lit(condition_code)))
        .collect()?;
    let emoji_series = df_result.column("emoji")?;
    let emoji = emoji_series
        .str()?
        .get(0)
        .ok_or(EmojiConverterError::NoMatchingEmojiFound)?;
    Ok(emoji.to_string())
}
