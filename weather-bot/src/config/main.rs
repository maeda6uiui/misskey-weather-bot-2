use std::{env::{self, VarError}, num::ParseIntError};

use clap::Parser;
use thiserror::Error;

pub struct Config{
    pub weather_api_access_token:String,
    pub weather_api_query:String,
    pub weather_api_days:i32,
    pub misskey_server_url:String,
    pub misskey_access_token:String,
}

#[derive(Error,Debug)]
pub enum ConfigError{
    #[error("unknown runtime: {0}")]
    UnknownRuntimeError(String),
    #[error("missing environment variable: {0}")]
    MissingEnvVarError(#[from] VarError),
    #[error("cannot parse string into number: {0}")]
    NumberParseError(#[from] ParseIntError),
}

#[derive(Debug,Parser)]
struct LocalArgs{
    #[arg(short='q',long)]
    weather_api_query:String,
    #[arg(short='d',long)]
    weather_api_days:i32,
    #[arg(short='u',long)]
    misskey_server_url:String,
}

impl Config{
    /// Creates a new config.
    ///
    /// ## Test locally
    /// Set `WEATHER_BOT_RUNTIME = local`.
    /// Access tokens are loaded from environment variables.
    /// Other values are loaded from command line arguments.
    /// Specify `override_args` if you want to manually construct the `LocalArgs` struct,
    /// rather than depending on the actual parsing of environment variables.
    /// 
    /// ## Test on Lambda
    /// Set `WEATHER_BOT_RUNTIME = lambda`.
    /// Access tokens are loaded from AWS Parameter Store.
    /// Other values are loaded from the environment variables of the Lambda function.
    pub fn new(override_args:Option<LocalArgs>)->Result<Self,ConfigError>{
        let runtime=match env::var("WEATHER_BOT_RUNTIME"){
            Ok(v)=>v,
            Err(_)=>"local".to_string(),
        };

        match runtime.as_str(){
            "local"=>Self::load_locally(override_args),
            "lambda"=>Self::load_on_lambda(),
            other=>Err(ConfigError::UnknownRuntimeError(other.to_string())),
        }
    }

    fn load_locally(override_args:Option<LocalArgs>)->Result<Config,ConfigError>{
        //Load access tokens from environment variables
        let weather_api_access_token=env::var("WEATHER_API_ACCESS_TOKEN")?;
        let misskey_access_token=env::var("MISSKEY_ACCESS_TOKEN")?;

        //Load some variables from command line arguments
        let args=match override_args{
            Some(v)=>v,
            None=>LocalArgs::parse(),
        };
        Ok(Config{
            weather_api_access_token,
            weather_api_query:args.weather_api_query,
            weather_api_days:args.weather_api_days,
            misskey_server_url:args.misskey_server_url,
            misskey_access_token,
        })
    }

    fn load_on_lambda()->Result<Config,ConfigError>{
        //Todo: Load access tokens from parameter store

        //Load some variables from environment variables
        let weather_api_access_token=env::var("WEATHER_API_ACCESS_TOKEN")?;
        let weather_api_query=env::var("WEATHER_API_QUERY")?;
        let weather_api_days=env::var("WEATHER_API_DAYS")?;
        let misskey_server_url=env::var("MISSKEY_SERVER_URL")?;
        let misskey_access_token=env::var("MISSKEY_ACCESS_TOKEN")?;

        Ok(Config { 
            weather_api_access_token,
            weather_api_query, 
            weather_api_days:weather_api_days.parse()?, 
            misskey_server_url, 
            misskey_access_token 
        })
    }
}

#[cfg(test)]
mod tests{
    use serial_test::serial;
    use super::*;

    #[test]
    #[serial]
    fn invalid_runtime(){
        unsafe {
            env::set_var("WEATHER_BOT_RUNTIME", "invalid");
        }

        let result=Config::new(None);
        assert!(
            matches!(
                result,
                Err(ConfigError::UnknownRuntimeError(_)),
            )
        );

        unsafe{
            env::remove_var("WEATHER_BOT_RUNTIME");
        }
    }

    #[test]
    #[serial]
    fn load_locally_success(){
        unsafe {
            env::set_var("WEATHER_BOT_RUNTIME", "local");
            env::set_var("WEATHER_API_ACCESS_TOKEN", "access_token");
            env::set_var("MISSKEY_ACCESS_TOKEN", "access_token");
        }

        let args=LocalArgs::try_parse_from(vec![
            "--weather-api-query","Tokyo",
            "--weather-api-days","7",
            "--misskey-server-url","https://example.com",
        ]).unwrap();
        let config=Config::new(Some(args)).unwrap();

        assert_eq!(config.weather_api_access_token,"access_token");
        assert_eq!(config.weather_api_query,"Tokyo");
        assert_eq!(config.weather_api_days,7);
        assert_eq!(config.misskey_server_url,"https://example.com");
        assert_eq!(config.misskey_access_token,"access_token");

        unsafe{
            env::remove_var("WEATHER_BOT_RUNTIME");
            env::remove_var("WEATHER_API_ACCESS_TOKEN");
            env::remove_var("MISSKEY_ACCESS_TOKEN");
        }
    }

    #[test]
    #[serial]
    fn load_locally_missing_env_var(){
        unsafe {
            env::set_var("WEATHER_BOT_RUNTIME", "local");
            env::set_var("WEATHER_API_ACCESS_TOKEN", "access_token");
        }

        let args=LocalArgs::try_parse_from(vec![
            "--weather-api-query","Tokyo",
            "--weather-api-days","7",
            "--misskey-server-url","https://example.com",
        ]).unwrap();
        let result=Config::new(Some(args));
        assert!(
            matches!(
                result,
                Err(ConfigError::MissingEnvVarError(_))
            )
        );

        unsafe{
            env::remove_var("WEATHER_BOT_RUNTIME");
            env::remove_var("WEATHER_API_ACCESS_TOKEN");
        }
    }

    #[test]
    #[serial]
    fn load_locally_number_parse_error(){
        unsafe {
            env::set_var("WEATHER_BOT_RUNTIME", "local");
            env::set_var("WEATHER_API_ACCESS_TOKEN", "access_token");
            env::set_var("MISSKEY_ACCESS_TOKEN", "access_token");
        }

        let args=LocalArgs::try_parse_from(vec![
            "--weather-api-query","Tokyo",
            "--weather-api-days","test",
            "--misskey-server-url","https://example.com",
        ]).unwrap();
        let result=Config::new(Some(args));
        assert!(
            matches!(
                result,
                Err(ConfigError::NumberParseError(_))
            )
        );

        unsafe{
            env::remove_var("WEATHER_BOT_RUNTIME");
            env::remove_var("WEATHER_API_ACCESS_TOKEN");
            env::remove_var("MISSKEY_ACCESS_TOKEN");
        }
    }
}