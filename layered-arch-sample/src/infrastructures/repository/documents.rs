use super::super::database::schema::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[table_name = "documents"]
pub struct NewDocumentEntity {
    pub title: String,
    pub body: String,
}
