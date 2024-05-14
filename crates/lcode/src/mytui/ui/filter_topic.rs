use lcode_config::global::{G_THEME, G_USER_CONFIG};
use ratatui::{
    prelude::{style::palette::tailwind, *},
    widgets::*,
};
use rayon::prelude::*;

use crate::{
    app::{inner::App, Tab2Panel},
    mytui::{
        helper::{self, bottom_rect},
        TuiMode,
    },
};

pub fn draw_difficults(f: &mut Frame, app: &mut App, area: Rect) {
    let items = app
        .topic
        .difficultys
        .iter()
        .map(|v| ListItem::new(v.as_str()));

    let style = if app.topic.index == Tab2Panel::Difficulty {
        G_THEME.topic.active_border
    }
    else {
        G_THEME.topic.inactive_border
    };

    let list = List::new(items)
        .block(
            Block::default()
                .border_style(style)
                .borders(Borders::ALL)
                .title(
                    if app.topic.user_diff.is_empty() {
                        "Difficulty"
                    }
                    else {
                        &app.topic.user_diff
                    },
                )
                .title_alignment(Alignment::Center),
        )
        .highlight_style(G_THEME.topic.list_highlight);
    f.render_stateful_widget(list, area, &mut app.topic.difficultys_state);
}
// pub fn draw_chart(f: &mut Frame, app: &App, area: Rect) {
//     unimplemented!()
// }
pub fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let status = &app.topic.ac_status;

    for (index, wid) in app
        .topic
        .ac_status
        .iter()
        .map(|(diff, pass, total)| {
            Gauge::default()
                .label(format!("{}/{}", pass, total))
                .ratio((*pass as f64 / *total as f64).min(1.0))
                .block(helper::title_block(diff.as_str()))
                .gauge_style(tailwind::SKY.c800)
        })
        .enumerate()
    {
        f.render_widget(wid, chunk[index]);
    }

    let pass_total = status.iter().map(|v| v.1).sum::<u32>();
    let total = status.iter().map(|v| v.2).sum::<u32>();
    let total = Gauge::default()
        .label(format!("{}/{}", pass_total, total))
        .ratio((pass_total as f64 / total as f64).min(1.0))
        .block(helper::title_block("TOTAL"))
        .gauge_style(tailwind::SKY.c700);
    f.render_widget(total, chunk[3]);
}
pub fn draw_all_topic_tags(f: &mut Frame, app: &mut App, area: Rect) {
    let items = app.topic.topic_tags.iter().map(|v| {
        let name = if G_USER_CONFIG.config.translate {
            let mut name = v
                .name_translated
                .as_deref()
                .unwrap_or_default();
            if name.is_empty() {
                name = v.name.as_str();
            }
            name
        }
        else {
            v.name.as_str()
        };
        ListItem::new(name)
    });
    let style = if app.topic.index == Tab2Panel::AllTopics {
        G_THEME.topic.active_border
    }
    else {
        G_THEME.topic.inactive_border
    };
    let list = List::new(items)
        .block(
            Block::default()
                .border_style(style)
                .borders(Borders::ALL)
                .title("All Topic Tag")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(G_THEME.topic.list_highlight);
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.topic.topic_tags_state);
}

pub fn draw_user_topic(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Box<dyn Iterator<Item = ListItem<'_>>> = if G_USER_CONFIG.config.translate {
        Box::new(
            app.topic
                .user_topic_tags_translated
                .iter()
                .map(|v| ListItem::new(v.as_str())),
        )
    }
    else {
        Box::new(
            app.topic
                .user_topic_tags
                .iter()
                .map(|v| ListItem::new(v.as_str())),
        )
    };

    let style = if app.topic.index == Tab2Panel::UserTopics {
        G_THEME.topic.active_border
    }
    else {
        G_THEME.topic.inactive_border
    };
    let list = List::new(items)
        .block(
            Block::default()
                .border_style(style)
                .borders(Borders::ALL)
                .title("User Topic Tag")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(G_THEME.topic.list_highlight);
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.topic.user_topic_tags_state);
}

pub fn draw_filtered_qs(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .topic
        .filtered_qs
        .par_iter()
        .map(|v| ListItem::new(v.to_string()))
        .collect();

    let style = if app.topic.index == Tab2Panel::Questions {
        G_THEME.topic.active_border
    }
    else {
        G_THEME.topic.inactive_border
    };
    let count = items.len();
    let list = List::new(items)
        .block(
            Block::default()
                .title(format!("Questions count: {}", count))
                .title_alignment(Alignment::Center)
                .border_style(style)
                .borders(Borders::ALL),
        )
        .highlight_style(G_THEME.topic.list_highlight);
    // .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut app.topic.filtered_topic_qs_state);
}

/// progress bar, it will draw in `area` bottom
pub fn draw_sync_progress_new(f: &mut Frame, app: &App, area: Rect) {
    let label = Span::styled(
        format!("{:.2}%", app.topic.cur_perc * 100.0),
        G_THEME.topic.label,
    );
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("waiting sync ……")
                .borders(Borders::ALL),
        )
        .gauge_style(G_THEME.topic.gauge)
        .label(label)
        .ratio(app.topic.cur_perc);

    // let area = centered_rect(60, 20, area);
    let area = bottom_rect(60, area);

    f.render_widget(Clear, area); // this clears out the background
    f.render_widget(gauge, area);
}

/// input to filter question
pub fn draw_input_line(f: &mut Frame, app: &mut App, area: Rect) {
    let (title, sty) = match app.topic.input_line_mode {
        TuiMode::Normal => {
            unreachable!()
        },
        TuiMode::Insert => (
            "Default press `Esc` escape input line",
            G_THEME.topic.text_line_insert,
        ),
        TuiMode::Visual => todo!(),
        TuiMode::OutEdit => (
            "Default press `e` for input",
            G_THEME.topic.text_line_outedit,
        ),
    };
    app.topic.text_line.set_block(
        Block::default()
            .borders(Borders::ALL)
            .set_style(sty)
            .title(title),
    );
    f.render_widget(app.topic.text_line.widget(), area);
}
