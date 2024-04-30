use std::convert::Into;

use lcode_config::config::global::G_USER_CONFIG;
use leetcode_api::render::Render;
use ratatui::{
    prelude::{style::palette::tailwind, *},
    widgets::*,
};

use crate::{
    app::inner::App,
    mytui::{
        helper::{self, centered_rect_percent},
        my_widget::botton::{Button, Theme},
        TuiMode,
    },
};

/// show question's detail
pub fn draw_qs_content(f: &mut Frame, app: &mut App, area: Rect) {
    let title = if G_USER_CONFIG.config.translate {
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

    let text = app.cur_qs.to_tui_vec();

    app.edit.vertical_row_len = text.len();
    app.edit.content_vert_scroll_state = app
        .edit
        .content_vert_scroll_state
        .content_length(text.len());

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.clone().bold().blue())
                .title_alignment(Alignment::Center)
                .title_position(block::Position::Top),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((app.edit.content_vert_scroll as u16, 0));

    f.render_widget(paragraph, area);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.edit.content_vert_scroll_state,
    );
}

/// for edit code
pub fn draw_code_block(f: &mut Frame, app: &mut App, area: Rect) {
    let title = match app.edit.code_block_mode {
        TuiMode::Normal => "Normal, Press q to exit edit, vim like keybind, ctrl + s save code",
        TuiMode::Insert => "Insert, emacs like keybind",
        TuiMode::OutEdit => "OutEdit, Press e to start edit",
        TuiMode::Visual => todo!(),
    };
    let blk = if matches!(app.edit.code_block_mode, TuiMode::OutEdit) {
        Block::default()
    }
    else {
        Block::default().fg(Color::Green)
    }
    .title(title)
    .borders(Borders::ALL);
    app.edit.code_block.set_block(blk);
    app.edit.code_block.set_cursor_style(
        Style::default()
            .fg(Color::Reset)
            .add_modifier(Modifier::REVERSED),
    );

    f.render_widget(app.edit.code_block.widget(), area);
}

pub fn draw_pop_buttons(f: &mut Frame, app: &App, area: Rect) {
    let pat = helper::centered_rect_percent(20, 10, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(pat);

    assert!(layout.len() > 1);

    f.render_widget(Clear, pat);
    f.render_widget(
        Button::new("Test Code")
            .theme(Theme::test_color())
            .state(app.edit.button_state.states[0]),
        layout[0],
    );
    f.render_widget(
        Button::new("Submit Code")
            .theme(Theme::blue())
            .state(app.edit.button_state.states[1]),
        layout[1],
    );
    // f.render_widget(
    //     Button::new("Blue")
    //         .theme(BLUE)
    //         .state(app.edit.button_state[2]),
    //     layout[2],
    // );
}

pub fn draw_pop_submit(f: &mut Frame, app: &mut App, area: Rect) {
    let res = &app.edit.submit_res;

    let status_msg = res.start_tui_text();

    app.edit.submit_row_len = status_msg.len();

    let para = Paragraph::new(status_msg).scroll((0, app.edit.submit_hori_scroll as u16));

    let area = centered_rect_percent(60, 60, area);
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(Line::from(vec![
            "<esc> exit, j/k up/down ".into(),
            "Submit".bold(),
        ]))
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let layout = helper::nest_rect(area, 1, 1, 1, 1);

    let layout_nest = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .split(layout);
    assert!(layout_nest.len() > 4);

    f.render_widget(para, layout_nest[0]);

    #[cfg(not(debug_assertions))]
    let ratio = res.runtime_percentile();
    #[cfg(debug_assertions)]
    let ratio = 1.0;
    let gauge_fast = Gauge::default()
        .label(format!(
            "Runtime {}.Fast than {}%",
            res.status_runtime,
            ratio * 100.0
        ))
        .ratio(ratio)
        .gauge_style(tailwind::PURPLE.c800);
    f.render_widget(gauge_fast, helper::nest_rect(layout_nest[1], 2, 2, 0, 0));

    #[cfg(not(debug_assertions))]
    let ratio = res.memory_percentile();
    #[cfg(debug_assertions)]
    let ratio = 0.7;
    let gauge_mem = Gauge::default()
        .ratio(ratio)
        .label(format!(
            "Memory {}. Low than {}%",
            res.status_memory,
            ratio * 100.0
        ))
        .gauge_style(tailwind::CYAN.c800);
    f.render_widget(gauge_mem, helper::nest_rect(layout_nest[2], 2, 2, 0, 0));

    #[cfg(not(debug_assertions))]
    let ratio = res.total_correct() as f64 / res.total_testcases().max(1) as f64;
    #[cfg(debug_assertions)]
    let ratio = 0.3;
    let gauge_test_case = Gauge::default()
        .label(format!(
            "Correct Test Case {}/{}",
            res.total_correct(),
            res.total_testcases()
        ))
        .ratio(ratio)
        .gauge_style(tailwind::SKY.c800);
    f.render_widget(
        gauge_test_case,
        helper::nest_rect(layout_nest[3], 2, 2, 0, 0),
    );

    let other_msg = res.end_tui_text();

    let para = Paragraph::new(other_msg).scroll((
        app.edit.submit_vert_scroll as u16,
        app.edit.submit_hori_scroll as u16,
    ));
    f.render_widget(para, layout_nest[4]);
}

pub fn draw_pop_test(f: &mut Frame, app: &mut App, area: Rect) {
    let text = app.edit.test_res.to_tui_vec();
    app.edit.test_row_len = text.len();
    let para = Paragraph::new(text)
        .block(
            helper::title_block(Line::from(vec![
                "<esc> exit, j/k up/down ".into(),
                "Test".bold(),
            ]))
            .border_style(Style::default().fg(Color::Cyan)),
        )
        .scroll((
            app.edit.test_vert_scroll as u16,
            app.edit.test_hori_scroll as u16,
        ));

    let area = centered_rect_percent(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

pub fn draw_save_state(f: &mut Frame, _app: &App, area: Rect) {
    let area = centered_rect_percent(30, 20, area);

    let para = Paragraph::new("save code done").block(
        Block::default()
            .borders(Borders::ALL)
            .title("default press `esc` close")
            .title_alignment(Alignment::Center),
    );

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}
