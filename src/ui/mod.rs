use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{App, CurrentlyCreating},
    styles::TITLE_STYLE,
    ui::{
        creation_popup::{render_branch_input_popup, render_location_input_popup},
        main_screen::{render_branch_list, render_root_location},
    },
};
mod creation_popup;
mod main_screen;
mod util;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(2),
            Constraint::Length(5),
        ])
        .split(frame.area());

    // render the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Forestry, manage your git worktree forest",
        TITLE_STYLE,
    ))
    .alignment(Alignment::Center)
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    render_root_location(frame, app, chunks[1]);

    render_branch_list(frame, app, chunks[2]);

    if let Some(creating) = &app.creating {
        match creating {
            CurrentlyCreating::Branch => render_branch_input_popup(frame, app),
            CurrentlyCreating::Location => render_location_input_popup(frame, app),
        }
    }
}
