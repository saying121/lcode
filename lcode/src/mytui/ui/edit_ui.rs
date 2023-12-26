use ratatui::{
    prelude::*,
    style::{Style, Stylize},
    widgets::{block::Title, *},
    Frame,
};

use super::super::app::App;
use crate::{
    config::global::glob_config,
    mytui::{
        helper::{self, centered_rect},
        keymap::TuiMode,
        my_widget::*,
    },
    render::Render,
};

/// show question's detail
pub fn draw_qs_content(f: &mut Frame, app: &mut App, area: Rect) {
    // If want to add effects, it is very troublesome to deal with
    // let Rect {
    //     x: _,
    //     y: _,
    //     width,
    //     height: _height,
    // } = area;
    // let qs_str = qs.to_tui_mdvec((width - 2) as usize);

    let title = if glob_config().config.translate {
        app.cur_qs
            .translated_title
            .as_ref()
            .unwrap_or_else(|| {
                app.cur_qs
                    .question_title
                    .as_ref()
                    .unwrap_or(&app.cur_qs.title)
            })
    }
    else {
        app.cur_qs
            .question_title
            .as_ref()
            .unwrap_or(&app.cur_qs.title)
    };

    let paragraph = Paragraph::new(app.cur_qs.to_tui_vec())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(
                    Title::from(title.clone().bold().blue())
                        .alignment(Alignment::Center)
                        .position(block::Position::Top),
                ),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap {
            trim: true
        })
        .scroll((app.tab1.vertical_scroll as u16, 0));

    f.render_widget(paragraph, area);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.tab1.vertical_scroll_state,
    );
}

/// for edit code
pub fn draw_code_block(f: &mut Frame, app: &mut App, area: Rect) {
    let title = match app.tab1.code_block_mode {
        TuiMode::Normal => "Normal, Press q to exit edit, vim like keybind, ctrl + s save code",
        TuiMode::Insert => "Insert, emacs like keybind",
        TuiMode::OutEdit => "OutEdit, Press e to start edit",
        TuiMode::Select => todo!(),
    };
    app.tab1.code_block.set_block(
        Block::default()
            .title(title)
            .borders(Borders::ALL),
    );
    app.tab1
        .code_block
        .set_cursor_style(
            Style::default()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED),
        );

    f.render_widget(app.tab1.code_block.widget(), area);
}

pub fn draw_pop_menu(f: &mut Frame, app: &App, area: Rect) {
    let area = centered_rect(40, 20, area);

    let text = vec![
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("S", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Submit"),
        ]),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("T", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Test"),
        ]),
        Line::from(""),
        Line::from("Please wait a while after pressing S or T"),
    ];

    let style = if app.tab1.submitting {
        Style::default().fg(Color::Blue)
    }
    else {
        Style::default()
    };

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(style);

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

pub fn draw_pop_buttons(f: &mut Frame, _app: &App, area: Rect, states: &[State; 3]) {
    let pat = helper::centered_rect(40, 20, area);
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Min(0), // ignore remaining space
        ])
        .split(pat);
    f.render_widget(Clear, pat);
    f.render_widget(
        Button::new("Red")
            .theme(RED)
            .state(states[0]),
        layout[0],
    );
    f.render_widget(
        Button::new("Green")
            .theme(GREEN)
            .state(states[1]),
        layout[1],
    );
    f.render_widget(
        Button::new("Blue")
            .theme(BLUE)
            .state(states[2]),
        layout[2],
    );
}

pub fn draw_pop_submit(f: &mut Frame, app: &mut App, area: Rect) {
    let text = app.tab1.test_res.to_tui_vec();
    app.tab1.submit_row_len = text.len();

    let para = Paragraph::new(text)
        .block(
            Block::default()
                .border_style(Style::default().fg(Color::Cyan))
                .title(Title::from(Line::from(vec![
                    Span::styled("q exit, j/k up/down ", Style::default()),
                    Span::styled("Submit", Style::default().bold()),
                ])))
                .borders(Borders::ALL),
        )
        .scroll((
            app.tab1
                .submit_vert_scroll
                .try_into()
                .unwrap_or_default(),
            app.tab1
                .submit_hori_scroll
                .try_into()
                .unwrap_or_default(),
        ));

    let area = centered_rect(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.tab1.submit_vert_scroll_state,
    );
}

pub fn draw_pop_test(f: &mut Frame, app: &mut App, area: Rect) {
    let text = app.tab1.test_res.to_tui_vec();
    app.tab1.test_row_len = text.len();
    let para = Paragraph::new(text)
        .block(
            Block::default()
                .border_style(Style::default().fg(Color::Cyan))
                .title(Title::from(Line::from(vec![
                    Span::styled("q exit, j/k up/down ", Style::default()),
                    Span::styled("Test", Style::default().bold()),
                ])))
                .borders(Borders::ALL),
        )
        .scroll((
            app.tab1
                .test_vert_scroll
                .try_into()
                .unwrap_or_default(),
            app.tab1
                .test_hori_scroll
                .try_into()
                .unwrap_or_default(),
        ));

    let area = centered_rect(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            // .track_symbol(Some("░"))
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area.inner(&Margin {
            vertical:   0,
            horizontal: 1,
        }),
        &mut app.tab1.test_vert_scroll_state,
    );
}

pub fn draw_save_state(f: &mut Frame, _app: &App, area: Rect) {
    let area = centered_rect(60, 20, area);

    let para = Paragraph::new("save code ……").block(Block::default().borders(Borders::ALL));

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}
