use ratatui::{prelude::*, widgets::*};

use crate::app::inner::App;

pub fn draw_infos(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
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
    let pass_info_list = List::new(pass_data).block(
        Block::default()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .title("Pass Infos"),
    );

    let user_info_items = items.into_iter().map(ListItem::new);
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

    let chunks1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);
    assert!(chunks1.len() >= 2);
    f.render_widget(user_info_list, chunks1[0]);
    f.render_widget(pass_info_list, chunks1[1]);
    f.render_stateful_widget(keymap_list, chunks[1], &mut app.infos.keymaps_state);
}

// pub fn draw_avatar(
//     f: &mut Frame,
//     app: &mut App,
//     area: Rect,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut picker = Picker::from_termios()?;
//     picker.guess_protocol();
//     picker.background_color = Some(image::Rgb::<u8>([255, 0, 255]));
//     let dyn_img = Reader::open(app.infos.avatar_path.as_path())?.decode()?;
//
//     let mut image_state = picker.new_resize_protocol(dyn_img);
//
//     // let (tx_worker, rec_worker) = mpsc::channel::<(Box<dyn StatefulProtocol>, Resize, Rect)>();
//
//     // let mut async_state = ThreadProtocol::new(tx_worker, picker.new_resize_protocol(dyn_img));
//     // let img = ThreadImage::new().resize(Resize::Fit);
//
//     let img = StatefulImage::new(None).resize(Resize::Fit);
//     f.render_stateful_widget(img, area, &mut image_state);
//
//     Ok(())
// }
