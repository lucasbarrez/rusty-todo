use crate::app::{AppAction, AppState};
use crate::domain::{Todo, TodoRepository};

pub struct AppController<R: TodoRepository> {
    pub repo: R,
}

impl<R: TodoRepository> AppController<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn dispatch(&mut self, state: &mut AppState, action: AppAction) {
        match action {
            AppAction::AddTodo(title) => {
                let todo = Todo::new(title);
                self.repo.add(todo.unwrap());
                state.todos = self.repo.all();
            }

            AppAction::RemoveSelected => {}

            AppAction::ToggleSelected => {}

            _ => {}
        }
    }
}
