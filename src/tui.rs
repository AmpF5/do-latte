use std::io::{Stdout, stdout};

use crossterm::{
    event::{Event, EventStream},
    terminal::LeaveAlternateScreen,
};

use futures::StreamExt;
use ratatui::{
    crossterm::{self, cursor, terminal::EnterAlternateScreen},
    prelude::{Backend, CrosstermBackend},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> Self {
        Tui {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))
                .expect("error initializing terminal"),
        }
    }

    pub fn enter(&mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

        self.start();
    }

    pub fn start(&mut self) {}

    async fn event_loop(&mut self) {
        let mut event_stream = EventStream::new();

        loop {
            let event = event_stream
                .next()
                .await
                .expect("error while reading from stream");

            match event {
                Ok(e) => match e {
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Key(key_event) => todo!(),
                    Event::Mouse(mouse_event) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                },
                Err(_) => todo!(),
            }
        }
    }

    pub fn exit(&mut self) {
        crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show);
        crossterm::terminal::disable_raw_mode();
    }
}
