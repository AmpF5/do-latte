use ratatui::{
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
};
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
                Box::new(CategoriesWidget::new()),
                Box::new(ToDoListWidget::new()),
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

        while !self.should_exit {
            self.render(&mut tui)?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_exit = true;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
        tui.terminal.draw(|f| {
            let constraints = self
                .components
                .iter()
                .map(|f| f.constraint())
                .collect::<Vec<Constraint>>();

            let layout = Layout::horizontal(constraints).split(f.area());

            for (component, area) in self.components.iter_mut().zip(layout.iter()) {
                component.draw(f, *area);
            }
        })?;

        Ok(())
    }
}
