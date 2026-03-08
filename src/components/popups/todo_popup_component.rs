use derive_setters::Setters;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph, Widget},
};
use tracing::info;

use crate::{
    components::component::Component,
    widgets::{popup::Popup, popup_layout::PopupLayout},
};

#[derive(Default, Debug, Setters)]
pub struct ToDoPopupComponent {
    pub title: String,
    pub bottom_title: String,
}

impl ToDoPopupComponent {
    pub fn new() -> Self {
        ToDoPopupComponent::default()
    }
}

impl Component for ToDoPopupComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect, _is_focused: bool) {
        info!("drawing popup");
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: area.height / 3,
        };

        let layout = PopupLayout::new(popup_area.width, popup_area.height)
            .row(
                Constraint::Percentage(20),
                Paragraph::new("test 1").block(Block::bordered().title("Name")),
            )
            .row(
                Constraint::Percentage(80),
                Paragraph::new("test 2").block(Block::bordered().title("Content")),
            );

        Popup::new(layout)
            .title(self.title.clone())
            .bottom_title(self.bottom_title.clone())
            .render(popup_area, frame.buffer_mut());
    }
}
