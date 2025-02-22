use lcode_config::global::G_THEME;
use ratatui::{prelude::*, widgets::*};
use rayon::prelude::*;

use self::style::Styled;
use crate::{
    app::inner::App,
    mytui::{
        TuiMode,
        helper::{self, bottom_rect, centered_rect_percent},
    },
};

/// some info
pub fn draw_msg(f: &mut Frame, app: &mut App, area: Rect) {
    let (msg, style) = match app.select.inputline.mode {
        TuiMode::Insert => (
            vec![
                "Default press ".into(),
                "Esc".set_style(Style::default().add_modifier(Modifier::BOLD)),
                " to stop editing, ".into(),
                "Enter".set_style(Style::default().add_modifier(Modifier::BOLD)),
                " to reset the message".into(),
            ],
            Style::default(),
        ),
        _ => (
            vec![
                "Default Press ".into(),
                "C-q".set_style(Style::default().add_modifier(Modifier::BOLD)),
                " to exit, ".into(),
                "e".set_style(Style::default().add_modifier(Modifier::BOLD)),
                " to start editing.".into(),
            ],
            Style::default().add_modifier(Modifier::DIM),
        ),
    };

    let text = Text::from(Line::from(msg));
    let text = text.patch_style(style);
    let help_message = Paragraph::new(text);

    f.render_widget(help_message, area);
}

/// input to filter question
pub fn draw_input_line(f: &mut Frame, app: &mut App, area: Rect) {
    let (title, sty) = match app.select.inputline.mode {
        TuiMode::Normal => todo!(),
        TuiMode::Insert => ("Input to filter", G_THEME.select.text_line_insert),
        TuiMode::Visual => todo!(),
        TuiMode::OutEdit => ("Input to filter", G_THEME.select.text_line_outedit),
    };

    app.select
        .inputline
        .text_line
        .set_block(
            helper::title_block(title)
                .set_style(sty)
                .title_alignment(Alignment::Left),
        );

    f.render_widget(&app.select.inputline.text_line, area);
}

/// list questions
pub fn draw_table(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<Row<'_>> = app
        .select
        .qs_state
        .filtered_qs
        .par_iter()
        .map(|v| -> Row<'_> {
            let cells = vec![
                Cell::from(format!("{:07}", v.question_id)),
                Cell::from(format!("{:07}", v.frontend_question_id)),
                Cell::from(v.category.clone()),
                Cell::from(v.question_title.clone()),
                Cell::from(
                    v.pass_rate
                        .unwrap_or_default()
                        .to_string(),
                ),
                Cell::from(if v.paid_only { "" } else { "" }),
                match v.difficulty {
                    1 => Cell::from("⛳Easy").style(G_THEME.select.easy),
                    2 => Cell::from("🕎Medium").style(G_THEME.select.medium),
                    3 => Cell::from("💀Hard").style(G_THEME.select.hard),
                    _ => Cell::from(" Unknown").style(G_THEME.select.unknown),
                },
                Cell::from(if v.status.is_some() { "👍" } else { "" }),
            ];

            Row::new(cells)
                .height(1)
                .bottom_margin(0)
        })
        .collect();

    let header_cells = [
        "ID",
        "Frontend ID",
        "Category",
        "Title",
        "Passing Rate",
        "Paid Only",
        "Difficulty",
        "Status",
    ]
    .iter()
    .map(|h| Cell::from(*h).black());

    let header = Row::new(header_cells)
        .style(G_THEME.select.header)
        .height(1)
        .bottom_margin(1);
    let width = [
        Constraint::Max(7),
        Constraint::Max(12),
        Constraint::Max(11),
        Constraint::Max(65),
        Constraint::Max(12),
        Constraint::Max(9),
        Constraint::Max(10),
        Constraint::Max(10),
    ];
    let items = Table::new(items, width)
        .header(header)
        .block(
            helper::title_block(format!("Sum: {}", app.select.qs_state.filtered_qs.len()))
                .title_alignment(Alignment::Left),
        )
        .row_highlight_style(G_THEME.select.highlight_style)
        .highlight_symbol("");

    f.render_stateful_widget(items, area, &mut app.select.qs_state.state);
}

/// progress bar, it will draw in `area` bottom
pub fn draw_sync_progress(f: &mut Frame, app: &mut App, area: Rect) {
    let label = Span::styled(
        format!("{:.2}%", app.select.sync_bar.percent * 100.0),
        G_THEME.select.label,
    );
    let gauge = Gauge::default()
        .block(helper::title_block("waiting sync ……"))
        .gauge_style(G_THEME.select.gauge)
        .label(label)
        .ratio(app.select.sync_bar.percent);

    // let area = centered_rect(60, 20, area);
    let area = bottom_rect(60, area);

    f.render_widget(Clear, area); // this clears out the background
    f.render_widget(gauge, area);
}

/// some info, it will draw in `area` center
pub fn draw_pop_msg(f: &mut Frame, area: Rect) {
    let para = Paragraph::new(Line::from(vec![
        "Default press ".italic(),
        "S".bold(),
        " to sync database.".italic(),
    ]))
    .block(Block::default().borders(Borders::ALL));

    let area = centered_rect_percent(60, 20, area);

    f.render_widget(Clear, area); // this clears out the background
    f.render_widget(para, area);
}
