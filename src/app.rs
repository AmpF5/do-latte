use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::Block,
};

use crate::todo_item::TodoItem;

#[derive(Default)]
pub struct App {
    should_exit: bool,
    todo_list: Vec<TodoItem>,
}

impl App {
    // main app loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_exit {
            terminal.draw(App::render)?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key_press(key);
            }
        }

        Ok(())
    }

    // handle key press events
    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            _ => {}
        }
    }

    fn render(frame: &mut Frame) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(frame.area());

        frame.render_widget(Block::bordered().title("Categories"), left);
        frame.render_widget(Block::bordered().title("List"), right);
    }
}
