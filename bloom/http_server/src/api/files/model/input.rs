use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub namespace: String,
    pub file_id: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trash {
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFilesToTrash {
    pub files: Vec<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreFilesFromTrash {
    pub files: Vec<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyTrash {
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFiles {
    pub files:  Vec<Id>,
    pub destination: Id,
}
