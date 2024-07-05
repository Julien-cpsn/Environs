//#![feature(rustc_private)]

mod app;
mod app_state;
mod utils;
mod events;
mod tui;
mod business_logic;
mod models;

use std::io::stdout;
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();

    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    App::new()
        .chain_hook()
        .run(terminal).await?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
