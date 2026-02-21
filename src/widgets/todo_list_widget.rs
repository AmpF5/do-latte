use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Widget},
};

pub struct ToDoListWidget {}

impl ToDoListWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for ToDoListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec!["[a] - add new".into()]);

        let block = Block::bordered()
            .title("[1] List")
            .title_bottom(instructions);

        block.render(area, buf);
    }
}
