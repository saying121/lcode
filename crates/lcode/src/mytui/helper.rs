use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders},
};

/// helper function to create a bottom rect using up certain percentage of the available rect `r`
pub(super) fn bottom_rect(percent_x: u16, r: Rect) -> Rect {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(area[1])[1]
}
pub fn nest_rect(r: Rect, left: u16, right: u16, top: u16, bottom: u16) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top),
            Constraint::Min(0),
            Constraint::Length(bottom),
        ])
        .split(r);
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left),
            Constraint::Min(0),
            Constraint::Length(right),
        ])
        .split(layout[1]);
    layout[1]
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub(super) fn centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn title_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Line<'a>>,
{
    let title = Title::from(title).alignment(Alignment::Center);
    Block::default()
        .title(title)
        .borders(Borders::ALL)
}
