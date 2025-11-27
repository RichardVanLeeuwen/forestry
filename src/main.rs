use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, prelude::Backend};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
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

    let mut app = App::new()?;
    let _res = run_app(&mut terminal, &mut app);

    ratatui::restore();

    app.logging.iter().for_each(|log| println!("{}", log));

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        let event = event::read()?;
        // on key events
        if let Event::Key(key) = event {
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
                        app.branch_input = Input::default();
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
                    }
                    KeyCode::Enter => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => {
                                    let branch_name =
                                        if app.branch_list.state.selected().unwrap() == 0 {
                                            app.branch_input.value()
                                        } else {
                                            app.branch_list
                                                .items
                                                .get(app.branch_list.state.selected().unwrap())
                                                .unwrap()
                                        };
                                    app.worktree_location =
                                        Input::default().with_value(format!("../{}", branch_name));
                                    app.branch_name = branch_name.to_string();
                                    app.creating = Some(CurrentlyCreating::Location);
                                }
                                CurrentlyCreating::Location => {
                                    let branch_input = app.branch_input.value_and_reset();
                                    let worktree_location = app.worktree_location.value_and_reset();
                                    create_worktree(branch_input, worktree_location)?;
                                    app.tree_list = TreeList::new(&app.root)?;
                                    app.creating = None;
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Down => {
                        app.branch_list.state.select_next();
                    }
                    KeyCode::Up => {
                        app.branch_list.state.select_previous();
                    }
                    _ => {
                        if let Some(creating) = &app.creating {
                            match creating {
                                CurrentlyCreating::Branch => {
                                    app.branch_input.handle_event(&event);
                                }
                                CurrentlyCreating::Location => {
                                    app.worktree_location.handle_event(&event);
                                }
                            };
                        }
                    }
                },
                // add other screens
            }
        }
    }
}
