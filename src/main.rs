use std::io;

use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};
mod app;
mod styles;
mod ui;

use crate::app::{App, CurrentScreen};
use crate::ui::ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Todo! log and print the steps taken so the user gets a recap.
    // if let Ok(do_print) = res {
    //     if do_print {
    //         app.print_json()?;
    //     }
    // } else if let Err(err) = res {
    //     println!("{err:?}");
    // }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // on key events
        if let Event::Key(key) = event::read()? {
            // that are not releases
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                // For Main
                CurrentScreen::Main => match key.code {
                    // makes no sens atm, but only setting up the code for now.
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.tree_list.state.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.tree_list.state.select_previous();
                    }
                    KeyCode::Enter => {}
                    _ => {}
                },
                // add other screens
            }
        }
    }
}
