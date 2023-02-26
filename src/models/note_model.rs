use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct NoteModel {
    pub note_id: String,
    pub tags: Vec<String>,
    pub content: String,
}
