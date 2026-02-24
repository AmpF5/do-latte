use crate::app::App;
pub mod widgets;

pub mod action;
pub mod app;
pub mod todo_item;
pub mod tui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    let mut app = App::new();
    let result = app.run();
    ratatui::restore();
    result
}
