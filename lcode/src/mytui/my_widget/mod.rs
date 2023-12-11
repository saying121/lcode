use ratatui::{
    prelude::{Buffer, Rect},
    style::Color,
    text::Line,
    widgets::Widget,
};

#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    theme: Theme,
    state: State,
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {

    }
}
// #[test]
// fn temp() {
//     let but = Button::new("test");
// }

const BLUE: Theme = Theme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

const RED: Theme = Theme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

const GREEN: Theme = Theme {
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
enum State {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
struct Theme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}
