use crate::domains::value::id::Id;

pub type DocumentId = Id<Document>;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Document {
    pub fn create(title: String, body: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Default::default(),
            title,
            body,
            created_at: now,
            updated_at: now,
        }
    }
}
