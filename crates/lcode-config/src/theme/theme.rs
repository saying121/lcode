use ratatui::{
    prelude::Style,
    style::{Color, Modifier, Stylize},
};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Tab {
    pub border: Style,
    pub style: Style,
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
            border: Style::new(),
            style: Style::new().fg(Color::Cyan).dim(),
            highlight_style: Style::new().add_modifier(Modifier::BOLD),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Select {
    pub text_line_insert: Style,
    pub text_line_outedit: Style,

    pub easy: Style,
    pub medium: Style,
    pub hard: Style,
    pub unknown: Style,

    pub highlight_style: Style,
    pub header: Style,

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
            text_line_insert: Style::new().fg(Color::Yellow),
            text_line_outedit: Style::new(),

            easy: Style::new()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            medium: Style::new()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            hard: Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
            unknown: Style::new()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),

            highlight_style: Style::new().add_modifier(Modifier::REVERSED),
            header: Style::new().bg(Color::Blue),

            label: Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            gauge: Style::new().fg(Color::Cyan),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Edit {
    pub content_title: Style,
    pub content_border: Style,
    pub code_block_cursor: Style,

    pub submit_title: Style,
    pub submit_border: Style,
    pub test_title: Style,
    pub test_border: Style,

    pub gauge_time: Style,
    pub gauge_time_label: Style,
    pub gauge_memory: Style,
    pub gauge_mem_label: Style,
    pub gauge_tcase: Style,
    pub gauge_tcase_label: Style,
}

impl Default for Edit {
    fn default() -> Self {
        Self::new()
    }
}

impl Edit {
    pub fn new() -> Self {
        Self {
            content_title: Style::new().bold().blue(),
            content_border: Style::new().fg(Color::White),
            code_block_cursor: Style::new()
                .fg(Color::Reset)
                .add_modifier(Modifier::REVERSED),

            submit_title: Style::new().bold().cyan(),
            submit_border: Style::new().fg(Color::Cyan),
            test_title: Style::new().bold().cyan(),
            test_border: Style::new().fg(Color::Cyan),

            // gauge_time:   tailwind::PURPLE.c800.into(),
            gauge_time: Color::from_u32(0x6B_21_A8).into(),
            gauge_time_label: Style::new().fg(Color::White),
            // gauge_memory: tailwind::CYAN.c800.into(),
            gauge_memory: Color::from_u32(0x15_5E_75).into(),
            gauge_mem_label: Style::new().fg(Color::White),
            // gauge_tcase:  tailwind::SKY.c800.into(),
            gauge_tcase: Color::from_u32(0x07_59_85).into(),
            gauge_tcase_label: Style::new().fg(Color::White),
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct TopicTags {
    pub active_border: Style,
    pub inactive_border: Style,
    pub list_highlight: Style,

    pub label: Style,
    pub gauge: Style,

    pub text_line_insert: Style,
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
            active_border: Style::new().fg(Color::Blue),
            inactive_border: Style::new(),
            list_highlight: Style::new()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
            label: Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            gauge: Style::new().fg(Color::Cyan),

            text_line_insert: Style::new().fg(Color::Yellow),
            text_line_outedit: Style::new(),
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
    pub const fn new() -> Self {
        Self {
            list_highlight: Style::new()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        }
    }
}
