use ratatui::{
    Frame,
    layout::{Alignment, Margin},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{app::App, styles::POPUP_TEXT_STYLE, ui::util::centered_rect};

pub fn render_delete_popup(frame: &mut Frame, app: &mut App) {
    let popup_block = Block::default()
        .title("Deleting")
        .border_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));
    let area = centered_rect(60, 30, frame.area());
    let inner_rect = popup_block.inner(area).inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    let firstline = if app.ask_force_delete {
        "Deletion failed, likely due to changes in the tree. Do you want to force remove the tree, destroying uncommitted changes?"
    } else {
        "Are you sure you want to delete this tree?"
    };
    let options = if app.ask_force_delete {
        "[y/N]"
    } else {
        "[Y/n]"
    };
    let content_text = vec![
        Line::from(firstline),
        Line::from(""),
        Line::from(options).alignment(Alignment::Center),
    ];
    let paragraph = Paragraph::new(content_text)
        .style(POPUP_TEXT_STYLE)
        .wrap(Wrap { trim: true });
    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);
    frame.render_widget(paragraph, inner_rect);
}
