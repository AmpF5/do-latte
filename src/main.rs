use crate::app::App;
pub mod components;
mod utils;

pub mod action;
pub mod app;
pub mod todo_item;
pub mod tui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    let mut app = App::new();
    let result = app.run().await;
    ratatui::restore();
    result
}
