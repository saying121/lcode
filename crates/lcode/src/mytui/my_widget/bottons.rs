use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Buttons<'a> {
    buttons: Vec<Button<'a>>,
    states:  Vec<ButtonState>,
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
        let states = vec![ButtonState::Normal; size];
        Self { buttons, states }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: ButtonState,
}

impl<'a> Button<'a> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            ButtonState::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
            ButtonState::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
            ButtonState::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

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
                Style::new().fg(shadow).bg(background),
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

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Theme {
    text:       Color,
    background: Color,
    highlight:  Color,
    shadow:     Color,
}
pub const CYAN: Theme = Theme {
    text:       Color::Cyan,
    background: Color::LightCyan,
    shadow:     Color::DarkGray,
    highlight:  Color::Blue,
};

pub const BLUE: Theme = Theme {
    text:       Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight:  Color::Rgb(64, 96, 192),
    shadow:     Color::Rgb(32, 48, 96),
};

pub const RED: Theme = Theme {
    text:       Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight:  Color::Rgb(192, 64, 64),
    shadow:     Color::Rgb(96, 32, 32),
};

pub const GREEN: Theme = Theme {
    text:       Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight:  Color::Rgb(64, 192, 64),
    shadow:     Color::Rgb(32, 96, 32),
};

impl<'a> Button<'a> {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            label: label.into(),
            theme: BLUE,
            state: ButtonState::Normal,
        }
    }

    pub const fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub const fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Default)]
pub enum ButtonState {
    #[default]
    Normal,
    Selected,
    Active,
}
