use std::convert::Into;

use lcode_config::global::{G_THEME, G_USER_CONFIG};
use leetcode_api::render::Render;
use ratatui::{prelude::*, widgets::*};

use super::title_block;
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

    let text = app.cur_qs.to_para_vec();

    app.edit.qs_content.content_row_num = text.len();
    app.edit.qs_content.vert_scroll_state = app
        .edit
        .qs_content
        .vert_scroll_state
        .content_length(text.len());

    let paragraph = Paragraph::new(text)
        .block(title_block(
            title
                .as_str()
                .set_style(G_THEME.edit.content_title),
        ))
        .style(G_THEME.edit.content_border)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll((app.edit.qs_content.vert_scroll as u16, 0));

    f.render_widget(paragraph, area);
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    f.render_stateful_widget(scrollbar, area, &mut app.edit.qs_content.vert_scroll_state);
}

/// for edit code
pub fn draw_code_block(f: &mut Frame, app: &mut App, area: Rect) {
    let title = match app.edit.code_block.mode {
        TuiMode::Normal => "Normal, Press q exit edit, vim like keybind, ctrl-s save",
        TuiMode::Insert => "Insert, emacs like keybind",
        TuiMode::OutEdit => "OutEdit, Press e to start edit 🖊️",
        TuiMode::Visual => todo!(),
    };
    let blk = if matches!(app.edit.code_block.mode, TuiMode::OutEdit) {
        Block::default()
    }
    else {
        Block::default().fg(Color::Green)
    }
    .title(title)
    .borders(Borders::ALL);
    app.edit
        .code_block
        .code_block
        .set_block(blk);
    app.edit
        .code_block
        .code_block
        .set_cursor_style(G_THEME.edit.code_block_cursor);

    f.render_widget(app.edit.code_block.code_block.widget(), area);
}

pub fn draw_pop_buttons(f: &mut Frame, app: &App, area: Rect) {
    let pat = helper::centered_rect_percent(35, 10, area);

    let mid = 20;
    let [test, _, submit] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50 - mid / 2),
            Constraint::Percentage(mid),
            Constraint::Percentage(50 - mid / 2),
        ])
        .areas(pat);

    f.render_widget(Clear, test);
    f.render_widget(
        Button::new("Test Code 🍨")
            .theme(Theme::test_color())
            .state(app.edit.button.state.states[0]),
        test,
    );
    f.render_widget(Clear, submit);
    f.render_widget(
        Button::new("Submit Code 🚩")
            .theme(Theme::blue())
            .state(app.edit.button.state.states[1]),
        submit,
    );
}

pub fn draw_pop_submit(f: &mut Frame, app: &mut App, area: Rect) {
    let res = &app.edit.submit.content;

    let status_msg = res.start_tui_text();

    let para = Paragraph::new(status_msg).scroll((0, app.edit.submit.hori_scroll as u16));

    let area = centered_rect_percent(60, 60, area);
    f.render_widget(Clear, area);

    let mut title = vec!["Submit 🌊".set_style(G_THEME.edit.submit_title)];
    if app.edit.submit.need_add() {
        title.push("Can add last test case, default press `a` 🧪".red());
    }
    let block = title_block(Line::from(title));
    f.render_widget(block, area);

    let layout = helper::nested_rect(area, 1, 1, 1, 1);

    let [head, runtime, memory, test_case, other] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .areas(layout);

    f.render_widget(para, head);

    // #[cfg(debug_assertions)]
    // let ratio: f64 = res.runtime_percentile().max(80.0);
    // #[cfg(not(debug_assertions))]
    let ratio = res.runtime_percentile();
    let gauge_fast = Gauge::default()
        .label(
            format!(
                "⌚Runtime: {}. Faster than {}% of programmers.",
                res.status_runtime, ratio
            )
            .set_style(G_THEME.edit.gauge_time_label),
        )
        .ratio((ratio / 100.0).min(1.0))
        .gauge_style(G_THEME.edit.gauge_time);
    f.render_widget(gauge_fast, helper::nested_rect(runtime, 2, 2, 0, 0));

    // #[cfg(debug_assertions)]
    // let ratio: f64 = res.memory_percentile().max(60.0);
    // #[cfg(not(debug_assertions))]
    let ratio = res.memory_percentile();
    let gauge_mem = Gauge::default()
        .ratio((ratio / 100.0).min(1.0))
        .label(
            format!(
                "📝Use memory: {}. Lower than {}% of programmers.",
                res.status_memory, ratio
            )
            .set_style(G_THEME.edit.gauge_mem_label),
        )
        .gauge_style(G_THEME.edit.gauge_memory);
    f.render_widget(gauge_mem, helper::nested_rect(memory, 2, 2, 0, 0));

    let (t_corr, t_case) = (res.total_correct(), res.total_testcases());
    // #[cfg(debug_assertions)]
    // let ratio: f64 = (t_corr as u32 as f64 / t_case.max(1) as u32 as f64).max(0.3);
    // #[cfg(not(debug_assertions))]
    let ratio = t_corr as u32 as f64 / t_case.max(1) as u32 as f64;
    let gauge_test_case = Gauge::default()
        .label(
            format!("👉Correct Test Case {}/{}.", t_corr, t_case)
                .set_style(G_THEME.edit.gauge_tcase_label),
        )
        .ratio(ratio.min(1.0))
        .gauge_style(G_THEME.edit.gauge_tcase);
    f.render_widget(gauge_test_case, helper::nested_rect(test_case, 2, 2, 0, 0));

    let other_msg = res.end_tui_text();

    app.edit.submit.row_len = other_msg.len();

    let para = Paragraph::new(other_msg).scroll((
        app.edit.submit.vert_scroll as u16,
        app.edit.submit.hori_scroll as u16,
    ));
    f.render_widget(para, other);
}

pub fn draw_pop_test(f: &mut Frame, app: &mut App, area: Rect) {
    let text = app.edit.test.content.to_para_vec();
    app.edit.test.row_len = text.len();
    let para = Paragraph::new(text)
        .block(
            helper::title_block(Line::from(vec![
                "<esc> exit, j/k up/down ".into(),
                "Test 🌈".set_style(G_THEME.edit.test_title),
            ]))
            .border_style(G_THEME.edit.test_border),
        )
        .scroll((
            app.edit.test.vert_scroll as u16,
            app.edit.test.hori_scroll as u16,
        ));

    let area = centered_rect_percent(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

pub fn draw_save_state(f: &mut Frame, _app: &App, area: Rect) {
    let area = centered_rect_percent(30, 20, area);

    let para =
        Paragraph::new("save code done").block(helper::title_block("default press `esc` close"));

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}
