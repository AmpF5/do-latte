use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::Block,
};

use crate::widgets::component::Component;

#[derive(Default)]
pub struct ToDoListWidget {}

impl ToDoListWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for ToDoListWidget {
    fn init(&mut self) {}

    fn handle_key_event(&mut self, key: ratatui::crossterm::event::KeyEvent) {
        println!("{:?}", key);
    }

    fn constraint(&self) -> Constraint {
        Constraint::Percentage(70)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("[2] ToDos"), area);
    }
}
