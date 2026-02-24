use crate::app::App;
pub mod widgets;

pub mod action;
pub mod app;
pub mod todo_item;
pub mod tui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
