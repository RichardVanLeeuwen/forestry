use ratatui::style::{Color, Modifier, Style};

pub const TITLE_STYLE: Style = Style::new().fg(Color::LightGreen);
pub const LIST_ITEM_STYLE: Style = Style::new().fg(Color::Yellow);
pub const LIST_ITEM_SELECTED_STYLE: Style = LIST_ITEM_STYLE.add_modifier(Modifier::BOLD);
