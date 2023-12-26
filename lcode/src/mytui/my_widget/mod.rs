use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

struct Buttons<'a> {
    buttons: Vec<Button<'a>>,
    states: Vec<State>,
}

impl<'a> Buttons<'a> {
    fn new<T>(labels: Vec<T>) -> Self
    where
        T: Into<Line<'a>>,
    {
        let size = labels.len();
        let mut buttons = Vec::with_capacity(size);
        for i in labels {
            buttons.push(Button::new(i));
        }
        let states = vec![State::Normal;size];
        Self { buttons, states }
    }
}

#[derive(Debug, Clone)]
pub struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

impl<'a> Button<'a> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            State::Normal => {
                (theme.background, theme.text, theme.shadow, theme.highlight)
            }
            State::Selected => {
                (theme.highlight, theme.text, theme.shadow, theme.highlight)
            }
            State::Active => {
                (theme.background, theme.text, theme.highlight, theme.shadow)
            }
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(
            area,
            Style::new()
                .bg(background)
                .fg(text),
        );

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new()
                    .fg(highlight)
                    .bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new()
                    .fg(shadow)
                    .bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x
                + (area
                    .width
                    .saturating_sub(self.label.width() as u16))
                    / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}
pub const CYAN: Theme = Theme {
    text: Color::Cyan,
    background: Color::Reset,
    shadow: Color::DarkGray,
    highlight: Color::Blue,
};

pub const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

pub const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

pub const GREEN: Theme = Theme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight: Color::Rgb(64, 192, 64),
    shadow: Color::Rgb(32, 96, 32),
};

impl<'a> Button<'a> {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            label: label.into(),
            theme: BLUE,
            state: State::Normal,
        }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub const fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Selected,
    Active,
}
