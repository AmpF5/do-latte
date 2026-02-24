use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    prelude::Buffer,
    widgets::{Block, Widget},
};

use crate::widgets::component::Component;

#[derive(Default)]
pub struct CategoriesWidget {}

impl CategoriesWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for CategoriesWidget {
    fn constraint(&self) -> Constraint {
        Constraint::Percentage(30)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::bordered().title("[1] Categories"), area);
    }
}
