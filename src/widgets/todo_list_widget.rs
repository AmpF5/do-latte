use ratatui::{Frame, layout::Rect, widgets::Block};

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

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("To do"), area);
    }
}
