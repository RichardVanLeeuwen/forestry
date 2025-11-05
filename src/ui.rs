use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph},
};

use crate::{
    app::App,
    styles::{LIST_ITEM_SELECTED_STYLE, LIST_ITEM_STYLE, TITLE_STYLE},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
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
    .alignment(ratatui::layout::Alignment::Center)
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // render the main block
    let mut list_items = Vec::<ListItem>::new();
    for tree in &app.tree_list.items {
        list_items.push(ListItem::new(Line::from(Span::styled(
            tree.location.clone(),
            LIST_ITEM_STYLE,
        ))));
    }
    let list = List::new(list_items)
        .highlight_style(LIST_ITEM_SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, chunks[1], &mut app.tree_list.state);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
