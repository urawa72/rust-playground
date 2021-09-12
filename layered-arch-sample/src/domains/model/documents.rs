use crate::domains::value::id::Id;

pub type DocumentId = Id<Document>;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub body: String,
}

impl Document {
    pub fn create(title: String, body: String) -> Self {
        Self {
            id: Default::default(),
            title,
            body,
        }
    }
}
