use derive_setters::Setters;
use ratatui::{Frame, layout::Rect, widgets::Widget};
use tracing::info;

use crate::{
    components::component::Component,
    widgets::popup::{self, Popup},
};

#[derive(Default, Debug, Setters)]
pub struct ToDoPopupComponent {
    pub title: String,
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

        Popup::new("test body")
            .title(self.title.clone())
            .render(popup_area, frame.buffer_mut());
    }
}
