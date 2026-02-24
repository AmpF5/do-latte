use std::io::{Stdout, stdout};

use crossterm::{
    event::{Event, EventStream},
    terminal::LeaveAlternateScreen,
};

use futures::StreamExt;
use ratatui::{
    crossterm::{self, cursor, terminal::EnterAlternateScreen},
    prelude::CrosstermBackend,
};

pub struct Tui {
    pub terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Tui {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;

        self.start();

        Ok(())
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
    pub fn exit(&mut self) -> color_eyre::Result<()> {
        crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }
}
