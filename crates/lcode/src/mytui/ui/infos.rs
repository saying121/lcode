use lcode_config::config::global::G_THEME;
use ratatui::{prelude::*, widgets::*};

use crate::app::inner::App;

pub fn draw_infos(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(11),
            Constraint::Max(app.infos.keymaps_items.len() as u16 + 3),
        ])
        .split(area);
    assert!(chunks.len() >= 2);

    let info = &app.infos.user_status;

    let name = format!(
        "👤 User Name: {}",
        info.real_name
            .as_deref()
            .unwrap_or("unknown")
    );
    let points = format!("🌟 Points: {} 🪙", app.infos.points.points());

    let mut items = Vec::with_capacity(9);
    items.push(name);

    macro_rules! items {
        ($( ($itm:ident, $cond:expr, $status:expr) ); *) => {
            $(
                let mut $itm = $status.to_owned();
                if $cond {
                    $itm.push('👌');
                }
                else {
                    $itm.push('🚫');
                };
                items.push($itm);
            )*
        };
    }
    items!(
        (is_signed_in    , info.is_signed_in                   , "🌏 Signed In: ");
        (checked_in_today, info.checked_in_today               , "🌐 Checked In Today: ");
        (is_verified     , info.is_verified                    , "👍 Is Verified: ");
        (is_premium      , info.is_premium.unwrap_or_default() , "💫 Is Premium: ");
        (is_superuser    , info.is_superuser                   , "🦸 Is Superuser: ");
        (is_translator   , info.is_translator                  , "👨 Is Translator: ");
        (is_admin        , info.is_admin                       , "👑 Is Admin: ")
    );
    items.push(points);

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

    let user_info_list = List::new(items.into_iter().map(ListItem::new)).block(
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
        .highlight_style(G_THEME.info.list_highlight)
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
