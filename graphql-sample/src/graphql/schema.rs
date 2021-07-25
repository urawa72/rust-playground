use juniper::FieldResult;

#[derive(Clone, Debug)]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[juniper::object]
#[graphql(description = "A Project returns struct")]
impl Photo {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn url(&self) -> String {
        format!("http://example/{}", self.id)
    }
}

pub struct Query;
pub struct Mutation;

#[derive(Clone, Debug)]
pub struct Context {
    pub photos: Vec<Photo>,
}

impl juniper::Context for Context {}

#[juniper::object(Context = Context)]
impl Query {
    fn all_photos(&self, context: &Context) -> FieldResult<Vec<Photo>> {
        Ok(context.photos.clone())
    }
}

#[juniper::object(Context = Context)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_shema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
