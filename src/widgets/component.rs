use ratatui::{
    Frame,
    crossterm::event::{Event, KeyEvent},
    layout::{Constraint, Rect},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

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

    fn handle_key_event(&mut self, key: KeyEvent) {}

    fn draw(&mut self, frame: &mut Frame, area: Rect) {}

    fn constraint(&self) -> Constraint {
        Constraint::Fill(1)
    }
}
