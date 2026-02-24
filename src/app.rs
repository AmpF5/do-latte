use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::Block,
};
use tokio::sync::mpsc;

use crate::{
    action::Action,
    todo_item::TodoItem,
    tui::Tui,
    widgets::{
        categories_widget::CategoriesWidget, component::Component, todo_list_widget::ToDoListWidget,
    },
};

pub struct App {
    should_exit: bool,
    components: Vec<Box<dyn Component>>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        App {
            should_exit: false,
            components: vec![
                Box::new(ToDoListWidget::new()),
                Box::new(CategoriesWidget::new()),
            ],
            action_tx,
            action_rx,
        }
    }
    // main app loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let mut tui = Tui::new();

        tui.enter();

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone());
        }

        loop {
            // terminal.draw(self.render())?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.render(&mut tui);
                match key.code {
                    KeyCode::Char('q') => {
                        tui.exit();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) {
        tui.terminal.draw(|f| {
            for component in self.components.iter_mut() {
                component.draw(f, f.area());
            }
        });
    }
}
