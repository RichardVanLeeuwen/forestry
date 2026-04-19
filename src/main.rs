use tracing_subscriber::{EnvFilter, fmt};

use crate::app::App;

pub mod app;
pub mod event;
pub mod git;
pub mod keymapping;
pub mod regex;
pub mod styles;
pub mod ui;

fn init_logging() {
    let file = std::fs::File::create("debug.log").unwrap();
    fmt()
        .with_writer(file)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));

    // Logging
    init_logging();
    // Error handling
    color_eyre::install()?;
    // Hide the normal terminal
    let terminal = ratatui::init();
    // Run the app
    let result = App::new().run(terminal).await;
    // Restore the terminal
    ratatui::restore();

    result
}
