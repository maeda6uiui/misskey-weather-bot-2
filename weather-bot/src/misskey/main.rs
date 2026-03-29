use misskey::ClientExt;
use misskey::HttpClient;
use misskey::model::note::Note;
use thiserror::Error;

pub struct MisskeyClient{
    client:HttpClient
}

#[derive(Debug,Error)]
pub enum MisskeyClientError{
    #[error("sdk error: {0}")]
    SdkError(String)
}

impl MisskeyClient{
    pub fn new(server_url:&str,access_token:&str)->Result<Self,MisskeyClientError>{
        let client=match HttpClient::builder(server_url)
            .token(access_token)
            .build(){
            Ok(v)=>Ok(v),
            Err(e)=>Err(MisskeyClientError::SdkError(e.to_string())),
        }?;
        Ok(MisskeyClient { client })
    }

    pub async fn create_note(&self,text:&str)->Result<Note,MisskeyClientError>{
        let result=self.client.create_note(text).await;
        match result{
            Ok(v)=>Ok(v),
            Err(e)=>Err(MisskeyClientError::SdkError(e.to_string())),
        }
    }
}
