use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Padding, Paragraph},
};

use crate::{app::App, keyhints::key_hints_for};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn render_keylist(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let hints = key_hints_for(app);
    let mut spans: Vec<Span<'static>> = Vec::with_capacity(hints.len() * 3);

    for (i, hint) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            hint.key,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            hint.description,
            Style::default().fg(Color::Gray),
        ));
    }

    let hint_block = Block::bordered().padding(Padding::horizontal(1));
    let hints = Paragraph::new(Line::from(spans)).block(hint_block);

    frame.render_widget(hints, chunk);
}
