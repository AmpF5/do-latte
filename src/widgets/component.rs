use ratatui::{
    Frame,
    crossterm::event::{Event, KeyEvent},
    layout::{Constraint, Rect},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

pub struct ComponentEntry {
    pub component: Box<dyn Component>,
    pub constraint: Constraint,
    pub focus_key: Option<char>,
    pub is_focused: bool,
}

impl ComponentEntry {
    pub fn new(
        component: Box<dyn Component>,
        constraint: Constraint,
        focus_key: Option<char>,
    ) -> Self {
        Self {
            component,
            constraint,
            focus_key,
            is_focused: false,
        }
    }
}

pub trait Component {
    fn init(&mut self) {}

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        _ = tx;
    }

    fn handle_events(&mut self, event: Option<Event>) {
        match event {
            Some(e) => match e {
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Key(key_event) => self.handle_key_event(key_event),
                Event::Mouse(mouse_event) => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(_, _) => todo!(),
            },
            None => todo!(),
        }
    }

    fn handle_key_event(&mut self, _key: KeyEvent) {}

    fn register_focus_key(&mut self, _focus_key: Option<char>) {}

    fn draw(&mut self, _frame: &mut Frame, _area: Rect, _in_focused: bool) {}
}
