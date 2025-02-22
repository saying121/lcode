use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: ButtonState,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ButtonStates {
    pub states: Vec<ButtonState>,
}

impl Default for ButtonStates {
    fn default() -> Self {
        Self {
            states: vec![ButtonState::Selected, ButtonState::Normal],
        }
    }
}

impl Button<'_> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let theme = self.theme;
        match self.state {
            ButtonState::Normal => (theme.background, theme.text, theme.shadow, theme.highlight),
            ButtonState::Selected => (theme.highlight, theme.text, theme.shadow, theme.highlight),
            ButtonState::Active => (theme.background, theme.text, theme.highlight, theme.shadow),
        }
    }
}

impl Widget for Button<'_> {
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
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

impl Theme {
    pub const fn test_color() -> Self {
        let (r, g, b) = (21, 21, 16);
        Self {
            text: Color::Rgb(r, g, b),
            shadow: Color::Rgb(r * 2, g * 2, b * 2),
            background: Color::Rgb(r * 3, g * 3, b * 3),
            highlight: Color::Rgb(r * 6, g * 6, b * 6),
        }
    }
    pub const fn blue() -> Self {
        let (r, g, b) = (16, 24, 48);
        Self {
            text: Color::Rgb(r, g, b),
            shadow: Color::Rgb(r * 2, g * 2, b * 2),
            background: Color::Rgb(r * 3, g * 3, b * 3),
            highlight: Color::Rgb(r * 5, g * 5, b * 5),
        }
    }

    pub const fn red() -> Self {
        let (r, g, b) = (48, 16, 16);
        Self {
            text: Color::Rgb(r, g, b),
            shadow: Color::Rgb(r * 2, g * 2, b * 2),
            background: Color::Rgb(r * 3, g * 3, b * 3),
            highlight: Color::Rgb(r * 4, g * 4, b * 4),
        }
    }
    pub const fn green() -> Self {
        let (r, g, b) = (16, 48, 16);
        Self {
            text: Color::Rgb(r, g, b),
            shadow: Color::Rgb(r * 2, g * 2, b * 2),
            background: Color::Rgb(r * 3, g * 3, b * 3),
            highlight: Color::Rgb(r * 4, g * 4, b * 4),
        }
    }
    pub const fn submit_color() -> Self {
        Self {
            text: Color::Blue,
            background: Color::Reset,
            highlight: Color::Rgb(64, 96, 192),
            shadow: Color::Rgb(32, 48, 96),
        }
    }
}

impl<'a> Button<'a> {
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Line<'a>>,
    {
        Self {
            label: label.into(),
            theme: Theme::blue(),
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
