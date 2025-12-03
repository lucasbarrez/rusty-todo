use super::todo::Todo;
use uuid::Uuid;

pub trait TodoRepository {
    fn add(&mut self, todo: Todo);
    fn delete(&mut self, id: Uuid);
    fn update(&mut self, todo: Todo);
    fn all(&self) -> Vec<Todo>;
}
