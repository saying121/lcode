use ratatui::{prelude::*, style::Style, widgets::*, Frame};

use crate::mytui::app::App;

pub fn draw_keymaps(f: &mut Frame, app: &mut App, area: Rect) {
    let list = List::new(app.tab3.keymaps_items.clone())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.tab3.keymaps_state);
}
