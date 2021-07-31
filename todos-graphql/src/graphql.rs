use diesel::PgConnection;
use juniper::{FieldResult, RootNode};

use crate::{
    data::Todos,
    model::{CreateTodoInput, Todo},
};

use super::context::GraphQLContext;

pub struct Query;

#[juniper::object(Context = GraphQLContext)]
impl Query {
    #[graphql(name = "allTodos")]
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::all_todos(conn)
    }

    #[graphql(name = "doneTodos")]
    pub fn done_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::done_todos(conn)
    }

    #[graphql(name = "notDoneTodos")]
    pub fn done_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::not_done_todos(conn)
    }

    #[graphql(name = "getTodoById")]
    pub fn get_todo_by_id(context: &GraphQLContext, id: i32) -> FieldResult<Option<Todo>> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::get_todo_by_id(conn, id)
    }
}

pub struct Mutation;

#[juniper::object(Context = GraphQLContext)]
impl Mutation {
    #[graphql(name = "createTodo")]
    pub fn create_todo(context: &GraphQLContext, input: CreateTodoInput) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::create_todo(conn, input)
    }

    #[graphql(name = "markTodoAsDone")]
    pub fn mark_todo_as_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_done(conn, id)
    }

    #[graphql(name = "markTodoAsNotDone")]
    pub fn mark_todo_as_not_done(context: &GraphQLContext, id: i32) -> FieldResult<Todo> {
        let conn: &PgConnection = &context.pool.get().unwrap();

        Todos::mark_todo_as_not_done(conn, id)
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
