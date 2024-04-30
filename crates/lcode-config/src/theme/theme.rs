use ratatui::{
    prelude::Style,
    style::{palette::tailwind, Color, Modifier, Stylize},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Tab {
    pub border:          Style,
    pub tab_style:       Style,
    pub highlight_style: Style,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        Self {
            border:          Style::default(),
            tab_style:       Style::default().fg(Color::Cyan).dim(),
            highlight_style: Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Select {
    pub text_line_insert:  Style,
    pub text_line_outedit: Style,

    pub easy:    Style,
    pub medium:  Style,
    pub hard:    Style,
    pub unknown: Style,

    pub highlight_style: Style,
    pub header:          Style,

    pub label: Style,
    pub gauge: Style,
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            text_line_insert:  Style::default().fg(Color::Yellow),
            text_line_outedit: Style::default(),

            easy:    Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            medium:  Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            hard:    Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
            unknown: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),

            highlight_style: Style::default().add_modifier(Modifier::REVERSED),
            header:          Style::default().bg(Color::Blue),

            label: Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            gauge: Style::default().fg(Color::Cyan),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Edit {
    pub content_title:     Style,
    pub content_border:    Style,
    pub code_block_cursor: Style,

    pub submit_title:  Style,
    pub submit_border: Style,
    pub test_title:    Style,
    pub test_border:   Style,

    pub gauge_time:   Style,
    pub gauge_memory: Style,
    pub gauge_tcase:  Style,
}

impl Default for Edit {
    fn default() -> Self {
        Self::new()
    }
}

impl Edit {
    pub fn new() -> Self {
        Self {
            content_title:     Style::default().bold().blue(),
            content_border:    Style::default().fg(Color::White),
            code_block_cursor: Style::default()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED),

            submit_title:  Style::default().bold().cyan(),
            submit_border: Style::default().fg(Color::Cyan),
            test_title:    Style::default().bold().cyan(),
            test_border:   Style::default().fg(Color::Cyan),

            gauge_time:   tailwind::PURPLE.c800.into(),
            gauge_memory: tailwind::CYAN.c800.into(),
            gauge_tcase:  tailwind::SKY.c800.into(),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct TopicTags {
    pub active_border:   Style,
    pub inactive_border: Style,
    pub list_highlight:  Style,

    pub label: Style,
    pub gauge: Style,

    pub text_line_insert:  Style,
    pub text_line_outedit: Style,
}

impl Default for TopicTags {
    fn default() -> Self {
        Self::new()
    }
}

impl TopicTags {
    pub fn new() -> Self {
        Self {
            active_border:   Style::default().fg(Color::Blue),
            inactive_border: Style::default(),
            list_highlight:  Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
            label:           Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            gauge:           Style::default().fg(Color::Cyan),

            text_line_insert:  Style::default().fg(Color::Yellow),
            text_line_outedit: Style::default(),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Info {
    pub list_highlight: Style,
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    pub fn new() -> Self {
        Self {
            list_highlight: Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        }
    }
}
