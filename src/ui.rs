use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, Paragraph},
};

use crate::{
    app::{App, CurrentlyCreating},
    styles::{LIST_ITEM_SELECTED_STYLE, LIST_ITEM_STYLE, TITLE_STYLE},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
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
    .alignment(Alignment::Center)
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let root_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());
    let main_tree_text = Paragraph::new(Text::styled(
        format!(
            "Main git tree location: {}",
            app.root
                .commondir()
                .parent()
                .expect("Root directory not found")
                .to_path_buf()
                .into_os_string()
                .into_string()
                .expect("Root location not found")
        ),
        TITLE_STYLE,
    ))
    .alignment(Alignment::Left)
    .block(root_block);

    frame.render_widget(
        main_tree_text,
        chunks[1].inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
    );

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

    frame.render_stateful_widget(
        list,
        chunks[2].inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
        &mut app.tree_list.state,
    );

    if let Some(creating) = &app.creating {
        let title = match creating {
            CurrentlyCreating::Branch => "Select branch name",
            CurrentlyCreating::Location => "Enter worktree location",
        };
        let popup_block = Block::default()
            .title(title)
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .style(Style::default());
        let area = centered_rect(60, 20, frame.area());
        let inner_area = popup_block.inner(area);
        frame.render_widget(popup_block, area);

        let content_text = match creating {
            CurrentlyCreating::Location => app.worktree_location.value(),
            CurrentlyCreating::Branch => app.branch_name.value(),
        };
        let content = Paragraph::new(content_text).style(Style::default().fg(Color::Yellow));
        frame.render_widget(content, inner_area);
        let x = match creating {
            CurrentlyCreating::Location => app.worktree_location.visual_cursor(),
            CurrentlyCreating::Branch => app.branch_name.visual_cursor(),
        } + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
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
