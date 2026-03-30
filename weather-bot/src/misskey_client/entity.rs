use serde::{Deserialize, Serialize};

pub enum NoteVisibility {
    Public,
    Home,
    Followers,
    Direct(Vec<String>),
}

#[derive(Deserialize)]
pub struct Note {
    pub id: String,
}

#[derive(Serialize)]
pub struct CreateNoteRequest {
    pub visibility: String,
    #[serde(rename = "visibleUserIds")]
    pub visible_user_ids: Vec<String>,
    pub text: String,
}

#[derive(Deserialize)]
pub struct CreateNoteResponse {
    #[serde(rename = "createdNote")]
    pub created_note: Note,
}
