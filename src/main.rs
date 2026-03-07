use crate::app::App;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{self, EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod components;
mod utils;
mod widgets;

pub mod action;
pub mod app;
pub mod models;
pub mod tui;

fn init_tracing() -> WorkerGuard {
    let file_appender = rolling::daily("logs", "do-latte.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    guard
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> color_eyre::Result<()> {
    let _guard = init_tracing();
    tracing::info!("application starting");

    let mut app = App::new();
    let result = app.run().await;
    ratatui::restore();
    result
}
