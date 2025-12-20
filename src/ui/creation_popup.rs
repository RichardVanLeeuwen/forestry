use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, Paragraph},
};

use crate::{
    app::App,
    styles::{LIST_ITEM_SELECTED_STYLE, LIST_ITEM_STYLE},
    ui::util::centered_rect,
};

fn make_creation_popup(frame: &mut Frame, title: &str) -> Rect {
    let popup_block = Block::default()
        .title(title)
        .border_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    let area = centered_rect(70, 40, frame.area());
    let inner_rect = popup_block.inner(area);

    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);
    inner_rect
}

pub fn render_branch_input_popup(frame: &mut Frame, app: &mut App) {
    let content_text = app.branch_input.value();
    // render the list of all existing branches
    let mut list_items = Vec::<ListItem>::new();
    list_items.push(ListItem::new(Line::from(Span::styled(
        content_text,
        LIST_ITEM_STYLE,
    ))));
    app.branch_list
        .items
        .iter()
        .filter(|b| b.contains(content_text))
        .for_each(|branch| {
            list_items.push(ListItem::new(Line::from(Span::styled(
                branch.clone(),
                LIST_ITEM_STYLE,
            ))))
        });
    let list = List::new(list_items)
        .highlight_style(LIST_ITEM_SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let inner_rect = make_creation_popup(frame, "Select branch name");

    frame.render_stateful_widget(list, inner_rect, &mut app.branch_list.state);
    let x = app.branch_input.visual_cursor() + 1; // add 1 for the > list selector
    frame.set_cursor_position((inner_rect.x + x as u16, inner_rect.y));
}
pub fn render_location_input_popup(frame: &mut Frame, app: &mut App) {
    let content_text = app.worktree_location.value();
    let paragraph = Paragraph::new(content_text).style(LIST_ITEM_STYLE);

    let inner_rect = make_creation_popup(frame, "Enter worktree location");
    frame.render_widget(paragraph, inner_rect);
    let x = app.worktree_location.visual_cursor();
    frame.set_cursor_position((inner_rect.x + x as u16, inner_rect.y));
}
