use ratatui::crossterm::event::{self, KeyCode};
use tokio::sync::mpsc;

use crate::{
    action::Action,
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
    pub fn run(&mut self) -> color_eyre::Result<()> {
        let mut tui = Tui::new()?;

        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone());
        }

        loop {
            if let Some(key) = event::read()?.as_key_press_event() {
                self.render(&mut tui)?;
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

    fn render(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
        tui.terminal.draw(|f| {
            for component in self.components.iter_mut() {
                component.draw(f, f.area());
            }
        })?;

        Ok(())
    }
}
