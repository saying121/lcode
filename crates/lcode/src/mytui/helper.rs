use ratatui::{
    prelude::*,
    widgets::{Block, Borders, block::Title},
};

/// helper function to create a bottom rect using up certain percentage of the available rect `r`
pub(super) fn bottom_rect(percent_x: u16, r: Rect) -> Rect {
    let [_, area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .areas(r);
    let [_, area, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .areas(area);
    area
}
pub fn nested_rect(r: Rect, left: u16, right: u16, top: u16, bottom: u16) -> Rect {
    let [_, layout, _] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top),
            Constraint::Min(0),
            Constraint::Length(bottom),
        ])
        .areas(r);
    let [_, layout, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(left),
            Constraint::Min(0),
            Constraint::Length(right),
        ])
        .areas(layout);
    layout
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub(super) fn centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let [_, popup_layout, _] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .areas(r);

    let [_, area, _] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .areas(popup_layout);
    area
}
pub(super) fn top_right_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let [popup_layout, _] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(percent_y), Constraint::Min(0)].as_ref())
        .areas(r);

    let [_, area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Max(percent_x)].as_ref())
        .areas(popup_layout);
    area
}

pub fn title_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Line<'a>>,
{
    let title = Title::from(title);
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
}
