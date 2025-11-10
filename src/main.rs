use std::mem;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, prelude::Backend};
mod app;
mod error;
mod git;
mod styles;
mod ui;

use crate::{app::CurrentlyCreating, git::create_worktree, ui::ui};
use crate::{
    app::{App, CurrentScreen, TreeList},
    error::Result,
    git::remove_worktree,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    set_panic_hook();

    let mut app = App::new()?;
    let _res = run_app(&mut terminal, &mut app);

    ratatui::restore();

    app.logging.iter().for_each(|log| println!("{}", log));

    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = ratatui::restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
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
                    KeyCode::Char('d') => {
                        let tree = app
                            .tree_list
                            .items
                            .get(app.tree_list.state.selected().unwrap());
                        remove_worktree(app, tree.unwrap().name.clone())?;
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
