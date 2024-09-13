use lcode_config::global::G_THEME;
use ratatui::{prelude::*, widgets::*};
use ratatui_image::{thread::ThreadImage, Resize};

use crate::{app::inner::App, mytui::helper};

pub fn draw_info(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(11),
            Constraint::Max(app.info.keymap.items.len() as u16 + 3),
        ])
        .split(area);
    assert!(chunks.len() >= 2);

    let info = &app.info.user_status;

    let name = format!(
        "👤 User Name: {}",
        info.real_name
            .as_deref()
            .unwrap_or("unknown")
    );
    let points = format!("🌟 Points: {} 🪙", app.info.points.points());

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
        .info
        .pass_data
        .info()
        .into_iter()
        .map(ListItem::new);
    let pass_data = vec![ListItem::new("🐾 Pass Info")]
        .into_iter()
        .chain(pass_data);
    let pass_info_list = List::new(pass_data).block(helper::title_block("Pass Info"));

    let user_info_list =
        List::new(items.into_iter().map(ListItem::new)).block(helper::title_block("User Info"));

    let keymap_list = List::new(app.info.keymap.items.clone())
        .block(helper::title_block("Keymaps"))
        .highlight_style(G_THEME.info.list_highlight)
        .highlight_symbol(">>");

    let chunks1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);
    assert!(chunks1.len() >= 2);
    f.render_widget(user_info_list, chunks1[0]);
    draw_avatar(
        f,
        app,
        helper::top_right_rect(14, 9, chunks1[0].inner(Margin::new(1, 1))),
    );
    f.render_widget(pass_info_list, chunks1[1]);
    f.render_stateful_widget(keymap_list, chunks[1], &mut app.info.keymap.list_state);
}

pub fn draw_avatar(f: &mut Frame, app: &mut App, area: Rect) {
    let image = ThreadImage::default().resize(Resize::Fit(None));
    if let Some(state) = &mut app.img_state {
        f.render_stateful_widget(image, area, state);
    }
}
