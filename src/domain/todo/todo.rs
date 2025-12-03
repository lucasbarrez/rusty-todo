use uuid::Uuid;

use crate::domain::todo::{TodoTitle, title::TodoTitleError};

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: TodoTitle,
    pub done: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("Invalid title: {0}")]
    InvalidTitle(#[from] TodoTitleError),
}

impl Todo {
    pub fn new(title: String) -> Result<Self, TodoError> {
        Ok(Self {
            id: Uuid::new_v4(),
            title: TodoTitle::new(title)?,
            done: false,
        })
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
}
