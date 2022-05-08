use std::sync::Mutex;

use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Enum, InputObject, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;

static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| {
    Mutex::new(vec![
        User {
            login_id: "mHattrup".to_string(),
            name: "Mike Hattrup".to_string(),
            avatar: "".to_string(),
        },
        User {
            login_id: "gPlake".to_string(),
            name: "Glen Plake".to_string(),
            avatar: "".to_string(),
        },
        User {
            login_id: "sSchmidt".to_string(),
            name: "Scot Schmidt".to_string(),
            avatar: "".to_string(),
        },
    ])
});
static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| {
    Mutex::new(vec![
        Photo {
            id: 5,
            name: "Dropping the Heart Chute".to_string(),
            description: "The heart chute is one of my favorite chutes".to_string(),
            category: PhotoCategory::Action,
            user: "gPlake".to_string(),
        },
        Photo {
            id: 2,
            name: "Enjoying the sunshine".to_string(),
            description: "".to_string(),
            category: PhotoCategory::Selfie,
            user: "sSchmidt".to_string(),
        },
        Photo {
            id: 3,
            name: "Gunbarrel 25".to_string(),
            description: "25 laps on gunbarrel today".to_string(),
            category: PhotoCategory::Portait,
            user: "sSchmidt".to_string(),
        },
    ])
});

#[derive(Clone)]
struct User {
    login_id: String,
    name: String,
    avatar: String,
}

#[Object]
impl User {
    async fn login_id(&self) -> String {
        self.login_id.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn avatar(&self) -> String {
        self.avatar.clone()
    }

    async fn post_photos(&self) -> Vec<Photo> {
        PHOTOS
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.user == self.login_id)
            .cloned()
            .collect()
    }
}

#[derive(Clone)]
struct Photo {
    id: usize,
    name: String,
    description: String,
    category: PhotoCategory,
    user: String,
}

#[Object]
impl Photo {
    async fn id(&self) -> usize {
        self.id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> String {
        self.description.clone()
    }

    async fn category(&self) -> PhotoCategory {
        self.category
    }

    async fn posted_by(&self) -> User {
        USERS
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.login_id == self.user)
            .cloned()
            .unwrap()
    }
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
enum PhotoCategory {
    Selfie,
    Portait,
    Action,
}

impl Default for PhotoCategory {
    fn default() -> Self {
        Self::Portait
    }
}

struct Mutation;

#[derive(InputObject)]
struct PostPhotoInput {
    name: String,
    description: String,
    #[graphql(default_with = "PhotoCategory::default()")]
    category: PhotoCategory,
    user: String,
}

#[Object]
impl Mutation {
    async fn post_photo(&self, input: PostPhotoInput) -> Photo {
        let mut id = SEQUENCE_ID.lock().unwrap();
        *id += 1;
        let photo = Photo {
            id: *id,
            name: input.name,
            description: input.description,
            category: input.category,
            user: input.user,
        };
        PHOTOS.lock().unwrap().push(photo.clone());
        photo
    }
}

struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }

    async fn all_photos(&self) -> Vec<Photo> {
        PHOTOS.lock().unwrap().clone()
    }
}

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    println!("Playground: http://localhost:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
