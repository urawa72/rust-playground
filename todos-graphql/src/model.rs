use super::schema::todos;
use juniper::GraphQLInputObject;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
}

#[juniper::object]
impl Todo {
    #[graphql(name = "id")]
    fn id(&self) -> i32 {
        self.id
    }

    #[graphql(name = "task")]
    pub fn task(&self) -> &str {
        self.task.as_str()
    }

    #[graphql(name = "done")]
    fn done(&self) -> bool {
        self.done
    }
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub done: &'a bool,
}

#[derive(GraphQLInputObject)]
pub struct CreateTodoInput {
    pub task: String,
    pub done: Option<bool>,
}
