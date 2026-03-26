use crate::app::App;

pub mod app;
pub mod event;
pub mod git;
pub mod keymapping;
pub mod styles;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
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
