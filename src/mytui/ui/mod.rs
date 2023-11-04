mod edit_ui;
mod select_ui;

use ratatui::{
    prelude::*,
    style::{Style, Stylize},
    widgets::{block::Title, *},
    Frame,
};
use rayon::prelude::*;

use crate::config::global::glob_user_config;

use super::{app::App, helper::*};

pub(super) fn start_ui(f: &mut Frame, app: &mut App) {
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

            select_ui::draw_msg(f, app, chunks[0]);
            select_ui::draw_input_line(f, app, chunks[1]);

            select_ui::draw_table(f, app, chunks[2]);

            if app.tab0.questions.is_empty() {
                draw_pop_msg(f, f.size());
            }
            if app.tab0.sync_state {
                draw_sync_progress(f, app, f.size());
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

            edit_ui::draw_qs_content(f, app, chunks1[0]);
            edit_ui::draw_code_block(f, app, chunks1[1]);

            if app.tab1.show_pop_menu {
                edit_ui::draw_pop_menu(f, app, f.size());
            }

            if app.tab1.show_submit_res {
                edit_ui::draw_pop_submit(f, app, f.size());
            }
            if app.tab1.show_test_res {
                edit_ui::draw_pop_test(f, app, f.size());
            }
        }
        2 => {
            let area = chunks[1];
            let chunks1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(30), Constraint::Min(0)].as_ref())
                .split(area);
            let chunks2 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Percentage(60), Constraint::Percentage(40)].as_ref(),
                )
                .split(chunks1[0]);
            draw_all_topic_tags(f, app, chunks2[0]);
            draw_user_topic(f, app, chunks2[1]);
            draw_topic_filtered_qs(f, app, chunks1[1]);

            if app.tab2.filter_index <= 2 && app.tab2.topic_tags.is_empty() {
                draw_pop_msg(f, f.size());
            }
            if app.tab2.sync_state {
                draw_sync_progress_new(f, app, f.size());
            }
        }
        3 => {
            let area = chunks[1];
            draw_keymaps(f, app, area);
        }
        _ => {}
    };

    if app.pop_temp {
        draw_pop_temp(f, app, f.size());
    }

    if app.save_code {
        draw_pop_state(f, app, f.size());
    }
}

fn draw_all_topic_tags(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .tab2
        .topic_tags
        .par_iter()
        .map(|v| {
            let name = if glob_user_config().translate {
                let mut name = v
                    .name_translated
                    .as_deref()
                    .unwrap_or_default();
                if name.is_empty() {
                    name = v.name.as_str();
                }
                name
            } else {
                v.name.as_str()
            };
            ListItem::new(name)
        })
        .collect();
    let style = if app.tab2.filter_index == 0 {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };
    let list = List::new(items)
        .block(
            Block::default()
                .border_style(style)
                .borders(Borders::ALL)
                .title(Title::from("All Topic Tag"))
                .title_alignment(Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.tab2.topic_tags_state);
}
fn draw_user_topic(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem<'_>> = if glob_user_config().translate {
        app.tab2
            .user_topic_tags_translated
            .par_iter()
            .map(|v| ListItem::new(v.as_str()))
            .collect()
    } else {
        app.tab2
            .user_topic_tags
            .par_iter()
            .map(|v| ListItem::new(v.as_str()))
            .collect()
    };

    let style = if app.tab2.filter_index == 1 {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };
    let list = List::new(items)
        .block(
            Block::default()
                .border_style(style)
                .borders(Borders::ALL)
                .title(Title::from("User Topic Tag"))
                .title_alignment(Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.tab2.user_topic_tags_state);
}

fn draw_topic_filtered_qs(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .tab2
        .filtered_topic_qs
        .par_iter()
        .map(|v| {
            let name = if glob_user_config().translate {
                let mut name = v
                    .title_cn
                    .as_deref()
                    .unwrap_or_default();
                if name.is_empty() {
                    name = v.title.as_str();
                }
                name
            } else {
                v.title.as_str()
            };
            ListItem::new(format!(
                "FID: {id},Title: {tit}",
                id = v.frontend_question_id,
                tit = name
            ))
        })
        .collect();
    let style = if app.tab2.filter_index == 2 {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };
    let count = items.len();
    let list = List::new(items)
        .block(
            Block::default()
                .title(Title::from(format!("Questions count: {}", count)))
                .title_alignment(Alignment::Center)
                .border_style(style)
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.tab2.filtered_topic_qs_state);
}

fn draw_keymaps(f: &mut Frame, app: &mut App, area: Rect) {
    let list = List::new(app.tab3.keymaps_items.to_owned())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.tab3.keymaps_state);
}

fn draw_pop_state(f: &mut Frame, _app: &mut App, area: Rect) {
    let area = centered_rect(60, 20, area);

    let para =
        Paragraph::new("save code ……").block(Block::default().borders(Borders::ALL));

    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

fn draw_pop_temp(f: &mut Frame, app: &mut App, area: Rect) {
    let para = Paragraph::new(Line::from(app.temp_str.clone()))
        .block(Block::default().borders(Borders::ALL));
    let area = centered_rect(50, 50, area);
    f.render_widget(Clear, area);
    f.render_widget(para, area);
}

/// some info, it will draw in `area` center
fn draw_pop_msg(f: &mut Frame, area: Rect) {
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

/// progress bar, it will draw in `area` bottom
fn draw_sync_progress(f: &mut Frame, app: &mut App, area: Rect) {
    let label = Span::styled(
        format!("{:.2}%", app.tab0.cur_perc * 100.0),
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(String::from("waiting sync ……"))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .label(label)
        .ratio(app.tab0.cur_perc);

    // let area = centered_rect(60, 20, area);
    let area = bottom_rect(60, area);

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(gauge, area);
}
/// progress bar, it will draw in `area` bottom
fn draw_sync_progress_new(f: &mut Frame, app: &mut App, area: Rect) {
    let label = Span::styled(
        format!("{:.2}%", app.tab2.cur_perc * 100.0),
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(String::from("waiting sync ……"))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .label(label)
        .ratio(app.tab2.cur_perc);

    // let area = centered_rect(60, 20, area);
    let area = bottom_rect(60, area);

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(gauge, area);
}

/// tab bar
fn draw_tab(f: &mut Frame, app: &mut App, area: Rect) {
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
            Style::default().add_modifier(Modifier::BOLD), // .bg(Color::Black),
        );
    f.render_widget(tabs, area);
}
