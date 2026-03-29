use std::time::Duration;

use reqwest::{Client, Url, header::{self, HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue}};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::misskey_client::entity::{CreateNoteRequest, CreateNoteResponse, NoteVisibility};

pub struct MisskeyClient{
    server_url:String,
    access_token:String,
    http_client:Client,
}

#[derive(Debug,Error)]
pub enum MisskeyClientError{
    #[error("http client error: {0}")]
    HttpClientError(#[from] reqwest::Error),
    #[error("invalid header value: {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
    #[error("url parse error: {0}")]
    UrlParseError(String),
}

impl MisskeyClient{
    pub fn new(server_url:&str,access_token:&str)->Result<Self,MisskeyClientError>{
        let http_client=Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(MisskeyClient {
            server_url: server_url.to_string(), 
            access_token: access_token.to_string(), 
            http_client 
        })
    }

    pub async fn create_note(&self,text:&str,visibility:NoteVisibility)->Result<CreateNoteResponse,MisskeyClientError>{
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}",&self.access_token).as_str())?,
        );

        let str_visibility=match visibility{
            NoteVisibility::Public=>"public",
            NoteVisibility::Home=>"home",
            NoteVisibility::Followers=>"followers",
            NoteVisibility::Direct(_)=>"specified",
        };
        let mut visible_user_ids:Vec<String>=Vec::new();
        if let NoteVisibility::Direct(user_ids)=visibility{
            user_ids.iter().for_each(|v| visible_user_ids.push(v.to_string()));
        };

        let request=CreateNoteRequest{
            visibility:str_visibility.to_string(),
            visible_user_ids,
            text:text.to_string(),
        };

        let endpoint=format!("{}/notes/create",&self.server_url);
        let url=match Url::parse(&endpoint){
            Ok(v)=>Ok(v),
            Err(e)=>Err(MisskeyClientError::UrlParseError(e.to_string())),
        }?;

        let response=self.http_client
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?
            .json::<CreateNoteResponse>()
            .await?;
        Ok(response)

    }
}
