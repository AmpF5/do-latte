use std::io::{Stdout, stdout};

use crossterm::{
    event::{EventStream, KeyEvent},
    terminal::LeaveAlternateScreen,
};

use futures::{FutureExt, StreamExt};
use ratatui::{
    crossterm::{self, cursor, terminal::EnterAlternateScreen},
    prelude::CrosstermBackend,
};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::interval,
};
use tracing::{debug, error, info};

#[derive(Clone)]
pub enum Event {
    Init,
    Quit,
    Render,
    Error,
    Key(KeyEvent),
}

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    task: JoinHandle<()>,
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    frame_rate: f64,
}

impl Tui {
    pub fn new() -> color_eyre::Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Ok(Tui {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
            task: tokio::spawn(async {}),
            event_tx,
            event_rx,
            frame_rate: 60_f64,
        })
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        info!("entering TUI mode");
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;

        self.start();

        Ok(())
    }

    pub fn start(&mut self) {
        debug!(frame_rate = self.frame_rate, "starting event loop");
        let event_loop = Self::event_loop(self.frame_rate, self.event_tx.clone());
        self.task = tokio::spawn(event_loop);
    }

    async fn event_loop(frame_rate: f64, event_tx: UnboundedSender<Event>) {
        let mut event_stream = EventStream::new();
        let mut render_interval = interval(std::time::Duration::from_secs_f64(1.0 / frame_rate));

        event_tx
            .send(Event::Init)
            .expect("failed to send init event");

        loop {
            let event = tokio::select! {
                _ = render_interval.tick() => crate::tui::Event::Render,
                crossterm_event = event_stream.next().fuse() => match crossterm_event {
                    Some(Ok(event)) => match event {
                        crossterm::event::Event::FocusGained => todo!(),
                        crossterm::event::Event::FocusLost => todo!(),
                        crossterm::event::Event::Key(key_event) => crate::tui::Event::Key(key_event),
                        crossterm::event::Event::Mouse(_) => todo!(),
                        crossterm::event::Event::Paste(_) => todo!(),
                        crossterm::event::Event::Resize(_, _) => todo!(),
                    }
                    Some(Err(ref e)) => {
                        error!(error = %e, "crossterm event stream error");
                        crate::tui::Event::Error
                    }
                    None => break,
                }
            };

            if event_tx.send(event).is_err() {
                break;
            }
        }
    }
    pub fn exit(&mut self) -> color_eyre::Result<()> {
        info!("exiting TUI mode");
        crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
