use serde::{Deserialize, Serialize};
use surrealdb::{opt::PatchOp, sql::Thing, Surreal};

const TODOS: &str = "todos";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Todo {
    id: Option<Thing>,
    task: String,
    done: bool,
}

/*
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
async fn db_from_ws() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db
}
*/

use surrealdb::engine::local::{Db, Mem};
async fn db_from_mem() -> Surreal<Db> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db
}

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = db_from_mem().await;
    //let db = db_from_ws().await;
    db.use_ns("patch_test").use_db("patch_test").await.unwrap();
    let _: Vec<Todo> = db.delete(TODOS).await?;
    let todo = Todo {
        id: None,
        task: "task".to_string(),
        done: false,
    };
    let todo_created_1: Todo = db.create(TODOS).content(todo.clone()).await?;
    let todo_created_2: Todo = db.create(TODOS).content(todo.clone()).await?;
    assert_eq!(
        &todo_created_1.task, &todo_created_2.task,
        "todo tasks should be same"
    );
    let mut todo_clone = todo_created_1.clone();
    todo_clone.done = true;
    let id_1 = todo_created_1.id.clone().unwrap();
    let id_2 = todo_created_2.id.clone().unwrap();

    let todo_updated_with_content: Option<Todo> = db
        .update((TODOS, id_1.clone()))
        .content(todo_clone.clone())
        .await
        .unwrap();
    assert_eq!(
        todo_updated_with_content.as_ref().unwrap().done,
        todo_clone.done,
        "done should match after update using content"
    );
    let todo_updated_with_patch: Option<Todo> = db
        .update((TODOS, id_2.clone()))
        .patch(PatchOp::replace("/done", true))
        .await
        .unwrap();
    assert_eq!(
        todo_updated_with_patch.as_ref().unwrap().done,
        todo_clone.done,
        "done should match after update using patch"
    );
    assert_eq!(
        todo_updated_with_patch, todo_updated_with_content,
        "updates with patch and content should match"
    );
    Ok(())
}
