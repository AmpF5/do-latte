use ratatui::{Frame, layout::Rect, symbols::border, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use crate::{action::Action, widgets::component::Component};

#[derive(Default, Debug)]
pub struct ToDoListWidget {
    focus_key: char,
    command_tx: Option<UnboundedSender<Action>>,
}

impl ToDoListWidget {
    pub fn new() -> Self {
        ToDoListWidget::default()
    }
}

impl Component for ToDoListWidget {
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
            let bottom_title = "test bottom title".to_string();
            border = border.title_bottom(bottom_title);
        }

        frame.render_widget(border, area);
    }
}
