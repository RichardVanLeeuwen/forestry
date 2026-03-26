use crossterm::event::KeyCode;

use crate::{
    app::{App, CurrentScreen},
    event::AppEvent,
};

pub fn mapkey(app: &mut App, key: KeyCode) {
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
                app.events.send(AppEvent::CreateTree);
            }
            KeyCode::Char('d') => {
                app.events.send(AppEvent::CreateTree);
            }
            _ => {}
        },
        CurrentScreen::Creating => match key {
            KeyCode::Esc => {
                // if let Some(creating) = &app.creating {
                //     match creating {
                //         CurrentlyCreating::Branch => {
                //             app.creating = None;
                //             app.current_screen = CurrentScreen::Main;
                //         }
                //         CurrentlyCreating::Location => {
                //             app.creating = Some(CurrentlyCreating::Branch);
                //         }
                //     }
                // }
            }
            KeyCode::Enter => {
                // if let Some(creating) = &app.creating {
                //     match creating {
                //         CurrentlyCreating::Branch => {
                //             let branch_name = if app.branch_list.state.selected().unwrap() == 0 {
                //                 app.branch_input.value()
                //             } else {
                //                 app.branch_list
                //                     .items
                //                     .iter()
                //                     .filter(|b| b.contains(app.branch_input.value()))
                //                     .take(app.branch_list.state.selected().unwrap())
                //                     .last()
                //                     .unwrap()
                //             };
                //             app.worktree_location =
                //                 Input::default().with_value(format!("../{}", branch_name));
                //             app.branch_name = branch_name.to_string();
                //             app.creating = Some(CurrentlyCreating::Location);
                //         }
                //         CurrentlyCreating::Location => {
                //             let branch_input = app.branch_input.value_and_reset();
                //             let worktree_location = app.worktree_location.value_and_reset();
                //             create_worktree(branch_input, worktree_location)?;
                //             app.tree_list = TreeList::new(&app.root)?;
                //             app.creating = None;
                //             app.current_screen = CurrentScreen::Main;
                //         }
                //     }
                // }
            }
            KeyCode::Down => {
                // app.branch_list.state.select_next();
            }
            KeyCode::Up => {
                // app.branch_list.state.select_previous();
            }
            _ => {
                // if let Some(creating) = &app.creating {
                //     match creating {
                //         CurrentlyCreating::Branch => {
                //             app.branch_input.handle_event(&event);
                //         }
                //         CurrentlyCreating::Location => {
                //             app.worktree_location.handle_event(&event);
                //         }
                //     };
                // }
            }
        },
        CurrentScreen::Deleting => {
            // if app.ask_force_delete {
            //     // forcing the delete only on input y
            //     match key {
            //         KeyCode::Char('y') => {
            //             let tree = app
            //                 .tree_list
            //                 .items
            //                 .get(app.tree_list.state.selected().unwrap());
            //             remove_worktree(app, tree.unwrap().name.clone(), true)?;
            //             fetch_origin(&app.root)?;
            //             app.refresh_branchlist()?;
            //             app.current_screen = CurrentScreen::Main;
            //             app.ask_force_delete = false;
            //         }
            //         KeyCode::Enter | KeyCode::Esc | KeyCode::Char('n') => {
            //             app.current_screen = CurrentScreen::Main;
            //             app.ask_force_delete = false;
            //         }
            //         _ => {}
            //     }
            // } else {
            //     // not force deleting, accepting both enter and y to delete
            //     match key {
            //         KeyCode::Enter | KeyCode::Char('y') => {
            //             let tree = app
            //                 .tree_list
            //                 .items
            //                 .get(app.tree_list.state.selected().unwrap());
            //             let remove_result = remove_worktree(app, tree.unwrap().name.clone(), false);
            //             if remove_result.is_err() {
            //                 app.ask_force_delete = true;
            //             } else {
            //                 fetch_origin(&app.root)?;
            //                 app.refresh_branchlist()?;
            //                 app.current_screen = CurrentScreen::Main;
            //             }
            //         }
            //         KeyCode::Esc | KeyCode::Char('n') => {
            //             app.current_screen = CurrentScreen::Main;
            //             app.ask_force_delete = false;
            //         }
            //         _ => {}
            //     } // add other screens
            // }
        }
    }
}
