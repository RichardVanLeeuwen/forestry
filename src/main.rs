use std::{io, mem};

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
mod git;
mod styles;
mod ui;

use crate::app::{App, CurrentScreen, TreeList};
use crate::{app::CurrentlyCreating, git::create_worktree, ui::ui};

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool, git2::Error> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        // on key events
        if let Event::Key(key) = event::read().unwrap() {
            // that are not releases
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                // For Main
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        // quit
                        return Ok(true);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.tree_list.state.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.tree_list.state.select_previous();
                    }
                    KeyCode::Char('c') | KeyCode::Char('+') => {
                        app.creating = Some(CurrentlyCreating::Branch);
                        app.current_screen = CurrentScreen::Creating;
                    }
                    _ => {}
                },
                CurrentScreen::Creating => match key.code {
                    KeyCode::Esc => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => {
                                    app.creating = None;
                                    app.current_screen = CurrentScreen::Main;
                                }
                                CurrentlyCreating::Location => {
                                    app.creating = Some(CurrentlyCreating::Branch);
                                }
                            }
                        }
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Enter => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => {
                                    app.worktree_location = format!("../{}", app.branch_name);
                                    app.creating = Some(CurrentlyCreating::Location);
                                }
                                CurrentlyCreating::Location => {
                                    let branch_name =
                                        mem::replace(&mut app.branch_name, String::new());
                                    let worktree_location =
                                        mem::replace(&mut app.worktree_location, String::new());
                                    create_worktree(branch_name, worktree_location)?;
                                    app.tree_list = TreeList::new(&app.root)?;
                                    app.creating = None;
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => app.branch_name.pop(),
                                CurrentlyCreating::Location => app.worktree_location.pop(),
                            };
                        }
                    }
                    KeyCode::Char(char) => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => app.branch_name.push(char),
                                CurrentlyCreating::Location => app.worktree_location.push(char),
                            };
                        }
                    }
                    _ => {}
                },
                // add other screens
            }
        }
    }
}
