use crate::{
    action::Action,
    components::{
        component::{Component, ComponentEntry},
        popups::todo_popup_component::ToDoPopupComponent,
        todo_list_component::ToDoListComponent,
    },
    tui::Tui,
};
use crossterm::event::KeyEvent;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

pub struct App {
    should_exit: bool,
    components: Vec<ComponentEntry>,
    active_popup: Option<Box<dyn Component>>,
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
                // ComponentEntry::new(
                //     Box::new(CategoriesComponent::new()),
                //     Constraint::Percentage(30),
                //     Some('1'),
                // ),
                ComponentEntry::new(
                    Box::new(ToDoListComponent::new()),
                    Constraint::Percentage(90),
                    Some('2'),
                ),
            ],
            active_popup: None,
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

    pub fn handle_component_key(&mut self, key_event: KeyEvent) -> Action {
        match self.get_focused_component() {
            Some(focused_component_entry) => focused_component_entry
                .component
                .handle_key_event(key_event),
            None => Action::None,
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

        Ok(())
    }

    /// handles global event
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
        let action_tx = self.action_tx.clone();

        match key.code {
            KeyCode::Char('q') => action_tx.send(Action::Quit)?,
            KeyCode::Char(ch) => {
                if self.get_components_keys().contains(&ch) {
                    self.set_focus(&ch);
                }

                action_tx.send(self.handle_component_key(key))?
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
                    self.should_exit = true;
                }
                Action::Error(ref err) => {
                    warn!(error = %err, "action error received");
                }
                Action::RenderToDoPopup => {
                    info!("adding popup");
                    self.active_popup = Some(Box::new(
                        ToDoPopupComponent::new()
                            .title("test".to_string())
                            .bottom_title("test bottom_title".to_string()),
                    ));
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

            let layout = Layout::vertical(constraints).split(f.area());

            for (component_entry, area) in self.components.iter_mut().zip(layout.iter()) {
                component_entry
                    .component
                    .draw(f, *area, component_entry.is_focused);
            }

            if let Some(popup_to_render) = &mut self.active_popup {
                info!("inside popup_render");
                popup_to_render.draw(f, f.area(), true);
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
