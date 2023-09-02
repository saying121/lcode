use ratatui::{
    prelude::{Backend, Constraint, Layout, Rect, *},
    style::{Style, Stylize},
    widgets::{block::Title, *},
    Frame,
};
use rayon::prelude::*;

use crate::{config::global::glob_user_config, entities::index, render::Render};

use super::{
    app::{App, InputMode},
    helper::*,
};

pub(super) fn start_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let constraints = [Constraint::Length(2), Constraint::Min(1)];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(f.size());

    draw_tab(f, app, chunks[0]);

    match app.tab_index {
        0 => {
            let constraints = [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ];
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints.as_ref())
                .split(chunks[1]);

            draw_msg(f, app, chunks[0]);
            draw_input_line(f, app, chunks[1]);

            draw_table(f, app, chunks[2]);

            if app.questions.len() == 0 {
                draw_pop_msg(f, f.size());
            }
        }
        1 => {
            let area = chunks[1];
            let chunks1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                )
                .split(area);

            draw_qs_content(f, app, chunks1[0]);
            draw_code_block(f, app, chunks1[1]);

            if app.pop_submit_test {
                draw_pop_menu(f, app, f.size());
            }

            if app.show_submit_res {
                draw_pop_submit(f, app, f.size());
            }
            if app.show_test_res {
                draw_pop_test(f, app, f.size());
            }
        }
        2 => {
            let area = chunks[1];
            draw_keymaps(f, app, area);
        }
        _ => unreachable!(),
    };

    if app.sync_state {
        draw_sync_state(f, app, f.size());
    }

    if app.pop_temp {
        draw_pop_temp(f, app, f.size());
    }

    if app.save_code {
        draw_pop_state(f, app, f.size());
    }
}

fn draw_keymaps<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let list = List::new(app.l_items.to_owned())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("List"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.l_state);
}

fn draw_pop_menu<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
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

    let style = match app.submiting {
        true => Style::default().fg(Color::Blue),
        false => Style::default(),
    };

    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(style);

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

fn draw_pop_submit<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let str = app.submit_res.to_tui_vec();

    let text: Vec<Line> = str
        .par_iter()
        .map(|v| Line::from(Span::raw(v)))
        .collect();

    let para = Paragraph::new(text).block(
        Block::default()
            .title("q exit")
            .borders(Borders::ALL),
    );

    let area = centered_rect(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}
fn draw_pop_test<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let str = app.test_res.to_tui_vec();

    let text: Vec<Line> = str
        .par_iter()
        .map(|v| Line::from(Span::raw(v)))
        .collect();

    let para = Paragraph::new(text).block(
        Block::default()
            .title("q exit")
            .borders(Borders::ALL),
    );

    let area = centered_rect(60, 60, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

fn draw_pop_state<B: Backend>(f: &mut Frame<B>, _app: &mut App, area: Rect) {
    let area = centered_rect(60, 20, area);

    let para =
        Paragraph::new("save code ……").block(Block::default().borders(Borders::ALL));

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

fn draw_pop_temp<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let para = Paragraph::new(Line::from(app.temp_str.clone()))
        .block(Block::default().borders(Borders::ALL));
    let area = centered_rect(50, 50, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

/// for edit code
fn draw_code_block<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    app.code_block
        .set_style(match app.edit_code {
            false => Style::default(),
            true => Style::default().fg(Color::Yellow),
        });

    let (title, color) = if app.edit_code {
        match app.code_block_mode {
            InputMode::Normal => (
                "Normal, Press q to exit edit, vim like keybind, ctrl + s save code",
                Style::default()
                    .fg(Color::Reset)
                    .add_modifier(Modifier::REVERSED),
            ),
            InputMode::Insert => (
                "Insert, emacs like keybind",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::REVERSED),
            ),
        }
    } else {
        (
            "Normal, Press e to start edit",
            Style::default()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED),
        )
    };

    app.code_block.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(title),
    );
    app.code_block
        .set_cursor_style(color);

    f.render_widget(app.code_block.widget(), area);
}

/// some info
fn draw_pop_msg<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let para = Paragraph::new(Line::from(vec![
        "Press ".italic(),
        "S".bold(),
        " to sync database.".italic(),
    ]))
    .block(Block::default().borders(Borders::ALL));

    let area = centered_rect(60, 20, area);

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(para, area);
}

/// progress bar
fn draw_sync_state<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let perc = app.cur_index_num as f64 / app.total_index_num as f64 * 100.0;

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(format!("waiting sync {} ……", app.sync_title))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(perc as u16);

    // let area = centered_rect(60, 20, area);
    let area = bottom_rect(60, area);

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(gauge, area);
}

/// show question's detail
fn draw_qs_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // If want to add effects, it is very troublesome to deal with
    // let Rect {
    //     x: _,
    //     y: _,
    //     width,
    //     height: _height,
    // } = area;
    // let qs_str = qs.to_tui_mdvec((width - 2) as usize);

    let qs = &app.cur_qs;
    let qs_str = qs.to_tui_vec();

    let text: Vec<Line> = qs_str
        .par_iter()
        .map(|v| Line::from(Span::raw(v)))
        .collect();

    app.vertical_row_len = text.len();
    app.vertical_scroll_state = app
        .vertical_scroll_state
        .content_length(text.len() as u16);

    let title = match glob_user_config().translate {
        true => qs
            .translated_title
            .as_ref()
            .unwrap_or(
                qs.question_title
                    .as_ref()
                    .unwrap_or(&qs.title),
            ),
        false => qs
            .question_title
            .as_ref()
            .unwrap_or(&qs.title),
    }
    .trim_matches('"');

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(
                    Title::from(title.bold().blue())
                        .alignment(Alignment::Center)
                        .position(block::Position::Top),
                ),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, area);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.vertical_scroll_state,
    );
}

/// tab bar
fn draw_tab<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.yellow(), rest.green()])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default()),
        )
        .dim()
        .hidden()
        .select(app.tab_index)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .dim(),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, area);
}

/// soem info
fn draw_msg<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let (msg, style) = match app.input_line_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("C-q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::DIM),
        ),
        InputMode::Insert => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to reset the message"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    f.render_widget(help_message, area);
}

/// input to filter question
fn draw_input_line<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    app.text_line
        .set_style(match app.input_line_mode {
            InputMode::Normal => Style::default(),
            InputMode::Insert => Style::default().fg(Color::Yellow),
        });
    app.text_line.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input to filter"),
    );

    f.render_widget(app.text_line.widget(), area);
}

/// list questions
fn draw_table<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    use crate::fuzzy_search::filter;
    let line = &app.text_line.lines()[0];

    match app.input_line_mode {
        InputMode::Normal => {}
        InputMode::Insert => {
            app.questions_filtered = app
                .questions
                .clone()
                .into_par_iter()
                .filter(|v| filter(line, &"", &v.to_string(), 1))
                .collect::<Vec<index::Model>>();
        }
    };

    let items = app
        .questions_filtered
        .par_iter()
        .map(|v| {
            let cells = vec![
                Cell::from(format!("{:07}", v.question_id)),
                Cell::from(format!("{:07}", v.frontend_question_id)),
                Cell::from(v.category.to_owned()),
                Cell::from(v.question_title.to_owned()),
                Cell::from(
                    v.pass_rate
                        .unwrap_or_default()
                        .to_string(),
                ),
                Cell::from(v.paid_only.to_string()),
                match v.difficulty {
                    1 => Cell::from("⛳Easy").style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    2 => Cell::from("🕎Medium").style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    3 => Cell::from("💀Hard").style(
                        Style::default()
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD),
                    ),
                    _ => Cell::from(" Unknown").style(
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    ),
                },
            ];

            Row::new(cells)
                .height(1)
                .bottom_margin(0)
        })
        .collect::<Vec<Row>>();

    // let items = items.collect::<Vec<Row>>();
    app.questions_len = items.len();

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);

    let header_cells = [
        "ID",
        "Frontend ID",
        "Category",
        "Title",
        "Passing Rate",
        "Paid Only",
        "Difficulty",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let items = Table::new(items)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Sum: {}", app.questions_len)),
        )
        .highlight_style(selected_style)
        .highlight_symbol("")
        .widths(&[
            Constraint::Max(7),
            Constraint::Max(12),
            Constraint::Max(11),
            Constraint::Max(65),
            Constraint::Max(12),
            Constraint::Max(9),
            Constraint::Max(10),
        ]);

    f.render_stateful_widget(items, area, &mut app.state)
}
