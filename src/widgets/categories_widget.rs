use ratatui::{
    layout::Rect,
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

impl Component for CategoriesWidget {}

impl Widget for CategoriesWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("[1] Categories");

        block.render(area, buf);
    }
}
