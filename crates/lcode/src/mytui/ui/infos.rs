use ratatui::{prelude::*, widgets::*};

use crate::app::inner::App;

pub fn draw_infos(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(16),
            Constraint::Min(app.infos.keymaps_items.len() as u16),
        ])
        .split(area);
    assert!(chunks.len() >= 2);

    let info = &app.infos.user_status;
    macro_rules! items {
        ($itm:ident, $cond:expr, $status:expr, $success:expr, $failure:expr) => {
            let mut $itm = $status.to_owned();
            if $cond {
                $itm.push($success);
            }
            else {
                $itm.push($failure);
            };
        };
    }

    #[rustfmt::skip]
    items!(is_premium      , info.is_premium.unwrap_or_default() , "💫 Is Premium: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(is_admin        , info.is_admin                       , "👑 Is Admin: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(is_signed_in    , info.is_signed_in                   , "🌏 Signed In: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(is_superuser    , info.is_superuser                   , "🦸 Is Superuser: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(checked_in_today, info.checked_in_today               , "🌐 Checked In Today: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(is_translator   , info.is_translator                  , "👨 Is Translator: " , '👌', '🚫');
    #[rustfmt::skip]
    items!(is_verified     , info.is_verified                    , "👍 Is Verified: " , '👌', '🚫');

    let name = format!(
        "👤 User Name: {}",
        info.real_name
            .as_deref()
            .unwrap_or("unknown")
    );
    let points = format!("🌟 Points: {} 🪙", app.infos.points.points);
    let items = vec![
        name,
        is_signed_in,
        checked_in_today,
        is_verified,
        is_premium,
        is_superuser,
        is_translator,
        is_admin,
        points,
    ];

    let pass_data = app
        .infos
        .pass_data
        .infos()
        .into_iter()
        .map(ListItem::new);
    let pass_data = vec![ListItem::new("🐾 Pass Info")]
        .into_iter()
        .chain(pass_data);

    let user_info_items = items
        .into_iter()
        .map(ListItem::new)
        .chain(pass_data);
    let user_info_list = List::new(user_info_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .title("User Infos"),
    );

    let keymap_list = List::new(app.infos.keymaps_items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center)
                .title("Keymaps"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");

    f.render_widget(user_info_list, chunks[0]);
    f.render_stateful_widget(keymap_list, chunks[1], &mut app.infos.keymaps_state);
}
// pub fn draw_avatar(f: &mut Frame, app: &mut App, area: Rect) {
//
// }
