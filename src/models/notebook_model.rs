use crate::models::note_model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct NotebookModel {
    pub notebook_id: String,
    pub name: String,
    pub notes: Vec<note_model::NoteModel>,
}
