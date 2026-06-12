use ratatui::{
    Frame,
    layout::{Alignment, Margin, Rect},
    text::{Line, Span, Text},
    widgets::{HighlightSpacing, List, ListItem, Paragraph},
};

use crate::{
    app::App,
    styles::{LIST_ITEM_SELECTED_STYLE, LIST_ITEM_STYLE, TITLE_STYLE},
    ui::util::{get_path_from_tree, shorten_home},
};

pub fn render_root_location(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let main_tree_text = Paragraph::new(Text::styled(
        format!(
            "Main git tree location: {}",
            shorten_home(&app.root_location)
        ),
        TITLE_STYLE,
    ))
    .alignment(Alignment::Left);

    frame.render_widget(
        main_tree_text,
        chunk.inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
    );
}

pub fn render_branch_list(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let mut list_items = Vec::<ListItem>::new();
    for tree in &app.tree_list.items {
        list_items.push(ListItem::new(Line::from(Span::styled(
            shorten_home(&get_path_from_tree(tree)),
            LIST_ITEM_STYLE,
        ))));
    }
    let list = List::new(list_items)
        .highlight_style(LIST_ITEM_SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(
        list,
        chunk.inner(Margin {
            horizontal: 1,
            vertical: 0,
        }),
        &mut app.tree_list.state,
    );
}
