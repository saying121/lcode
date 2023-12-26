use std::fs;

use miette::IntoDiagnostic;
use ratatui::{
    prelude::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{self, Title},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};

use crate::{config::global::glob_config_path, mytui::app::App};

pub fn draw_config(f: &mut Frame, _app: &mut App, area: Rect) {
    let content = fs::read_to_string(glob_config_path())
        .into_diagnostic()
        .unwrap_or_default();
    let text: Vec<Line<'_>> = content
        .split('\n')
        .map(Line::from)
        .collect();
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(
                    Title::from("config".bold().blue())
                        .alignment(Alignment::Center)
                        .position(block::Position::Top),
                ),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
        // .scroll((
        //     app.tab1
        //         .vertical_scroll
        //         .try_into()
        //         .unwrap_or_default(),
        //     0,
        // ));
    f.render_widget(paragraph, area);
}
