#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoTitle(String);

#[derive(Debug, thiserror::Error)]
pub enum TodoTitleError {
    #[error("Title can't be empty!")]
    Empty,
    #[error("Title mustn't be longer than 100 characters!")]
    TooLong,
}

impl TodoTitle {
    const MAX_LENGTH: usize = 100;

    pub fn new(value: String) -> Result<Self, TodoTitleError> {
        if value.trim().is_empty() {
            return Err(TodoTitleError::TooLong);
        } else if value.len() > Self::MAX_LENGTH {
            return Err(TodoTitleError::TooLong);
        } else {
            Ok(Self(value))
        }
    }

    // Getter
    pub fn value(&self) -> &str {
        return &self.0;
    }
}
