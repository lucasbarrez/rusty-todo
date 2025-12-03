use crate::domain::Todo;

pub struct TodoService;

impl TodoService {
    pub fn filter_done(todos: &[Todo]) -> Vec<Todo> {
        todos.iter().filter(|t| t.done).cloned().collect()
    }
}
