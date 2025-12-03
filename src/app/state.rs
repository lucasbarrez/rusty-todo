use crate::domain::Todo;

pub struct AppState {
    pub todos: Vec<Todo>,
    pub input: String,
    pub selected: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: vec![],
            input: String::new(),
            selected: 0,
        }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }
}
