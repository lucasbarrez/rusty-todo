pub mod app;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod ui;
pub mod utils;

// Make imports easier in the main.rs

// Domain
pub use domain::{Todo, TodoRepository, TodoService};

// Infrastructure
pub use infrastructure::repository::InMemoryTodoRepository;

// App
pub use app::{AppAction, AppController, AppState};

// UI
pub use ui::display;
