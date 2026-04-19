use ratatui::crossterm::event::Event as CrosstermEvent;
use ratatui::crossterm::event::KeyCode;

use crate::{
    app::{App, CurrentScreen, CurrentlyCreating},
    event::AppEvent,
};

pub fn mapkey(app: &mut App, key: KeyCode, event: CrosstermEvent) {
    match app.current_screen {
        // For Main
        CurrentScreen::Main => match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                // quit
                app.events.send(AppEvent::Quit);
            }
            KeyCode::Char('k') | KeyCode::Up => app.events.send(AppEvent::TreelistUp),
            KeyCode::Char('j') | KeyCode::Down => app.events.send(AppEvent::TreelistDown),
            KeyCode::Char('c') | KeyCode::Char('+') => {
                app.events.send(AppEvent::EnterCreating);
            }
            KeyCode::Char('d') => {
                app.events.send(AppEvent::EnterDeleting);
            }
            _ => {}
        },
        CurrentScreen::Creating => match key {
            KeyCode::Esc => {
                app.events.send(AppEvent::ExitCreating);
            }
            KeyCode::Enter => {
                if let Some(creating) = &app.creating {
                    match creating {
                        CurrentlyCreating::Branch => {
                            app.events.send(AppEvent::SelectLocation);
                        }
                        CurrentlyCreating::Location => {
                            app.events.send(AppEvent::CreateTree);
                        }
                    }
                }
            }
            KeyCode::Up => {
                app.events.send(AppEvent::BranchListUp);
            }
            KeyCode::Down => {
                app.events.send(AppEvent::BranchListDown);
            }
            _ => app.events.send(AppEvent::TypeBranchName(event)),
        },
        CurrentScreen::Deleting => {
            if app.ask_force_delete {
                // forcing the delete only on input y
                match key {
                    KeyCode::Char('y') => {
                        app.events.send(AppEvent::ForceDeleteTree);
                    }
                    KeyCode::Enter | KeyCode::Esc | KeyCode::Char('n') => {
                        app.events.send(AppEvent::CancelDeleting);
                    }
                    _ => {}
                }
            } else {
                // not force deleting, accepting both enter and y to delete
                match key {
                    KeyCode::Enter | KeyCode::Char('y') => {
                        app.events.send(AppEvent::DeleteTree);
                    }
                    KeyCode::Esc | KeyCode::Char('n') => {
                        app.events.send(AppEvent::CancelDeleting);
                    }
                    _ => {}
                } // add other screens
            }
        }
    }
}
