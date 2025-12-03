use rusty_todo::{
    AppAction, AppController, AppState, InMemoryTodoRepository, display::TodoDisplay,
};

fn main() {
    // Create the repo
    let repo = InMemoryTodoRepository::new();
    // Creater the controller
    let mut controller = AppController::new(repo);
    // Create the initial state
    let mut state = AppState::new();

    // Test by adding todos
    println!("Adding todos\n");
    controller.dispatch(&mut state, AppAction::AddTodo("Acheter du pain".into()));
    controller.dispatch(&mut state, AppAction::AddTodo("Faire le ménage".into()));
    controller.dispatch(&mut state, AppAction::AddTodo("Coder en Rust".into()));

    // List the todo
    TodoDisplay::print_todos(&state);
}
