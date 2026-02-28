use crossterm::event::KeyEvent;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::{
    action::Action,
    components::{
        categories_component::CategoriesComponent, component::ComponentEntry,
        todo_list_component::ToDoListComponent,
    },
    tui::Tui,
};

pub struct App {
    should_exit: bool,
    components: Vec<ComponentEntry>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

impl App {
    pub fn new() -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        info!("initializing app");

        App {
            should_exit: false,
            components: vec![
                ComponentEntry::new(
                    Box::new(CategoriesComponent::new()),
                    Constraint::Percentage(30),
                    Some('1'),
                ),
                ComponentEntry::new(
                    Box::new(ToDoListComponent::new()),
                    Constraint::Percentage(70),
                    Some('2'),
                ),
            ],
            action_tx,
            action_rx,
        }
    }

    pub fn get_components_keys(&self) -> Vec<char> {
        self.components.iter().filter_map(|f| f.focus_key).collect()
    }

    pub fn get_focused_component(&mut self) -> Option<&mut ComponentEntry> {
        self.components.iter_mut().find(|f| f.is_focused)
    }

    pub fn get_component_by_focus_key(&self, focus_key: char) -> Option<usize> {
        self.components
            .iter()
            .position(|f| f.focus_key == Some(focus_key))
    }

    pub fn set_focus(&mut self, focus_key: &char) {
        for entry in self.components.iter_mut() {
            entry.is_focused = entry.focus_key == Some(*focus_key);
        }
    }

    // main app loop
    pub async fn run(&mut self) -> color_eyre::Result<()> {
        info!("starting main loop");
        let mut tui = Tui::new()?;

        tui.enter()?;

        for component_entry in self.components.iter_mut() {
            component_entry
                .component
                .register_focus_key(component_entry.focus_key);
        }

        // for component_entry in self.components.iter_mut() {
        //     component_entry
        //         .component
        //         .register_action_handler(self.action_tx.clone());
        // }
        //
        // let action_tx = self.action_tx.clone();

        loop {
            self.handle_event(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_exit {
                tui.exit()?;
                break;
            }
        }

        Ok(())
    }

    async fn handle_event(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };

        let action_tx = self.action_tx.clone();

        match event {
            crate::tui::Event::Quit => {
                debug!("received quit event");
                action_tx.send(Action::Quit)?;
            }
            crate::tui::Event::Render => action_tx.send(Action::Render)?,
            crate::tui::Event::Key(key_event) => {
                debug!(key = ?key_event, "received key event");
                self.handle_key_event(key_event)?;
            }
            _ => {}
        };

        // for component_entry in self.components.iter_mut() {
        //     if let Some(_action) = component_entry
        //         .component
        //         .handle_events(Some(event.clone()))?
        //     {
        //         // TODO: handle action from component
        //     }
        // }

        Ok(())
    }

    /// handles global event
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        let action_tx = self.action_tx.clone();

        // TODO: add reading from keybinds and produce Action
        match key.code {
            KeyCode::Char('q') => action_tx.send(Action::Quit)?,
            KeyCode::Char(ch) => {
                if self.get_components_keys().contains(&ch) {
                    self.set_focus(&ch);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Render => self.render(tui)?,
                Action::Quit => {
                    info!("quitting application");
                    self.should_exit = true;
                }
                Action::Error(ref err) => {
                    warn!(error = %err, "action error received");
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
        tui.terminal.draw(|f| {
            let constraints = self
                .components
                .iter()
                .map(|f| f.constraint)
                .collect::<Vec<Constraint>>();

            let layout = Layout::horizontal(constraints).split(f.area());

            for (component_entry, area) in self.components.iter_mut().zip(layout.iter()) {
                component_entry
                    .component
                    .draw(f, *area, component_entry.is_focused);
            }
        })?;

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
