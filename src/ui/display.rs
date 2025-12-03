use crate::app::AppState;
use crate::domain::Todo;

pub struct TodoDisplay;

impl TodoDisplay {
    pub fn print_todos(state: &AppState) {
        println!("Liste des Todos:\n");

        if state.todos().is_empty() {
            println!("  Aucun todo pour le moment.");
            return;
        }

        for (index, todo) in state.todos().iter().enumerate() {
            Self::print_todo(index + 1, todo);
        }
    }

    fn print_todo(number: usize, todo: &Todo) {
        let status = if todo.done { "✅" } else { "⬜" };
        println!("  {}. {} {}", number, status, todo.title.value());
    }

    /// Affiche un todo détaillé
    pub fn print_todo_details(todo: &Todo, index: usize) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Todo #{}", index + 1);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Titre: {}", todo.title.value());
        println!(
            "  Statut: {}",
            if todo.done { "Complété" } else { "En cours" }
        );
        println!("  ID: {}", todo.id);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}
