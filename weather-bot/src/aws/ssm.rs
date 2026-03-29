use aws_config::SdkConfig;
use aws_sdk_ssm::{Client, error::SdkError, operation::get_parameter::GetParameterError};
use thiserror::Error;

use crate::aws::config::create_sdk_config;

pub struct SsmClient{
    aws_client:Client
}

#[derive(Debug,Error)]
pub enum SsmClientError{
    #[error("sdk error: {0}")]
    SdkError(String),
    #[error("no parameter found: {0}")]
    NoParameterFoundError(String),
    #[error("no value found: {0}")]
    NoValueFoundError(String),
}

impl<T> From<SdkError<T>> for SsmClientError{
    fn from(from:SdkError<T>)->SsmClientError{
        SsmClientError::SdkError(from.to_string())
    }
}

impl SsmClient{
    pub async fn new(aws_region:&str)->Self{
        let sdk_config=create_sdk_config(aws_region).await;
        let aws_client=Client::new(&sdk_config);
        SsmClient{
            aws_client
        }
    }

    pub async fn get_parameter(&self,name:&str)->Result<String,SsmClientError>{
        let resp=self.aws_client
            .get_parameter()
            .name(name)
            .with_decryption(true)
            .send()
            .await?;
        let parameter=match resp.parameter(){
            Some(v)=>Ok(v.value()),
            None=>Err(SsmClientError::NoParameterFoundError(name.to_string())),
        }?;
        let value=match parameter{
            Some(v)=>Ok(v),
            None=>Err(SsmClientError::NoValueFoundError(name.to_string())),
        }?;
        Ok(value.to_string())
    }
}
