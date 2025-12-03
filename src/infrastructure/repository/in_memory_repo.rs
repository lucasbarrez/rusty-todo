use crate::domain::{Todo, TodoRepository};
use uuid::Uuid;

pub struct InMemoryTodoRepository {
    pub todos: Vec<Todo>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }
}

impl TodoRepository for InMemoryTodoRepository {
    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn delete(&mut self, id: Uuid) {
        self.todos.retain(|t| t.id != id);
    }

    fn update(&mut self, todo: Todo) {
        if let Some(t) = self.todos.iter_mut().find(|t| t.id == todo.id) {
            *t = todo;
        }
    }

    fn all(&self) -> Vec<Todo> {
        self.todos.clone()
    }
}
