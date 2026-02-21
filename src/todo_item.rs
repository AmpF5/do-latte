#[derive(Debug, Clone)]
pub struct TodoItem {
    name: String,
    content: String,
    state: TodoState,
}

impl TodoItem {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            state: TodoState::Todo,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TodoState {
    Todo,
    // InProgress,  -- ???
    Complteted,
    Deleted,
}
