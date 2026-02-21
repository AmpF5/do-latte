use ratatui::{
    layout::Rect,
    prelude::Buffer,
    widgets::{Block, Widget},
};

#[derive(Default)]
pub struct CategoriesWidget {}

impl CategoriesWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for CategoriesWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().title("[1] Categories");

        block.render(area, buf);
    }
}
