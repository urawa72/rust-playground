use crate::domains::model::documents::{Document, DocumentId};
use failure::Error;

pub trait DocumentRepository {
    fn find_by_id(&self, document_id: DocumentId) -> Result<Document, Error>;
    fn list(&self) -> Result<Vec<Document>, Error>;
    fn insert(&self, document: &Document) -> Result<(), Error>;
    fn update(&self, document: &Document) -> Result<(), Error>;
    fn delete(&self, document: &Document) -> Result<(), Error>;
}
