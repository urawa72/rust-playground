use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;

use crate::domains::repository::documents::DocumentRepository;

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Faild to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn document_repository(&self) -> impl DocumentRepository {
        use crate::infrastructures::repository::documents::DocumentRepositoryImpl;

        DocumentRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
