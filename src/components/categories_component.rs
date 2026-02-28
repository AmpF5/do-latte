use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::components::component::Component;

#[derive(Default, Debug)]
pub struct CategoriesComponent {
    focus_key: char,
}

impl CategoriesComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for CategoriesComponent {
    fn register_focus_key(&mut self, focus_key: Option<char>) {
        self.focus_key = focus_key.expect("focus_key need to be set");
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let title_top = format!("[{}] Categories", self.focus_key);

        let mut border = Block::bordered().title(title_top);

        if is_focused {
            let bottom_title = "test bottom title".to_string();
            border = border.title_bottom(bottom_title);
        }
        frame.render_widget(border, area);
    }
}
