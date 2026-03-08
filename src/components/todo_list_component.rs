use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action, components::component::Component, models::todo_item::TodoItem,
    utils::liner_builder::LineBuilder,
};

#[derive(Default, Debug)]
pub struct ToDoListComponent {
    todos: Vec<TodoItem>,
    focus_key: char,
    command_tx: Option<UnboundedSender<Action>>,
}

impl ToDoListComponent {
    pub fn new() -> Self {
        ToDoListComponent::default()
    }
}

impl Component for ToDoListComponent {
    fn register_focus_key(&mut self, focus_key: Option<char>) {
        self.focus_key = focus_key.expect("focus_key need to be set");
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        self.command_tx = Some(tx);
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let title_top = format!("[{}] ToDos", self.focus_key);

        let mut border = Block::bordered().title(title_top);

        if is_focused {
            let add_line = LineBuilder::new("Add")
                .bold_first_char()
                .surround_first_letter_with_brackets()
                .build();

            let delete_line = LineBuilder::new("Delete")
                .bold_first_char()
                .surround_first_letter_with_brackets()
                .build();

            border = border.title_bottom(add_line).title_bottom(delete_line);
        }

        frame.render_widget(border, area);
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Action {
        match _key.code {
            KeyCode::Char('a') => Action::RenderToDoPopup,
            _ => Action::None,
        }
    }
}
