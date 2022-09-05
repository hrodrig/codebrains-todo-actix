use crate::entity::{prelude::*, todos};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct TodosRepository {
    pub db_conn: DatabaseConnection,
}

impl TodosRepository {
    pub async fn get_todos(&self) -> Vec<todos::Model> {
        Todos::find()
            .all(&self.db_conn)
            .await
            .expect("Error while fetching all todos")
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Option<todos::Model> {
        Todos::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Error while fetching todo by id")
    }

    pub async fn create_todo(
        &self,
        title: String,
        completed: bool,
    ) -> todos::Model {
        let todo = todos::ActiveModel {
            id: NotSet,
            title: Set(title),
            completed: Set(completed),
        };

        todo.insert(&self.db_conn)
            .await
            .expect("Error while creating todo")
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: String,
        completed: bool,
    ) -> Option<todos::Model> {
        let todo = todos::ActiveModel {
            id: Set(id),
            title: Set(title),
            completed: Set(completed),
        };

        todo.update(&self.db_conn)
            .await
            .expect("Error while updating todo")
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Option<todos::Model> {
        Todos::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Error while fetching todo by id")
            .delete(&self.db_conn)
            .await
            .expect("Error while deleting todo")
    }
}