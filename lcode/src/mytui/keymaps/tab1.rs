use std::io::Stdout;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute
};
use miette::{IntoDiagnostic, Result};
use tui_textarea::{CursorMove, Input, Key, Scrolling};

use super::common_keymap;
use crate::{
    editor::{edit, CodeTestFile},
    leetcode::IdSlug,
    mytui::{
        app::{App, TuiMode},
        term::Term,
        ui::start_ui
    }
};

pub async fn init(
    app: &mut App<'_>,
    terminal: &mut Term,
    event: &Event,
    stdout: &mut Stdout
) -> Result<()> {
    if app.tab1.edit_code {
        match app.tab1.code_block_mode {
            TuiMode::Insert => tab1_keymap_insert(app, terminal, event, stdout),
            TuiMode::Normal => tab1_keymap_normal(app, terminal, event, stdout).await?
        }
    }
    else {
        tab1_keymap(app, terminal, event, stdout).await?;
    }

    Ok(())
}

pub async fn tab1_keymap(
    app: &mut App<'_>,
    terminal: &mut Term,
    event: &Event,
    stdout: &mut Stdout
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            // KeyCode::Char('o') => {
            //     let qs_slug = app
            //         .cur_qs
            //         .qs_slug
            //         .clone()
            //         .unwrap_or_default();
            //     if qs_slug.is_empty() {
            //         return Ok(());
            //     }
            //     edit(IdSlug::Slug(qs_slug), CodeTestFile::Code).await?;
            //
            //     app.get_code(&app.cur_qs.clone())
            //         .await?;
            //
            //     terminal.redraw()?;
            // },
            // KeyCode::Char('S') if app.tab1.show_pop_menu => app.submit_code(),
            // KeyCode::Char('T') if app.tab1.show_pop_menu => app.test_code(),
            // KeyCode::Char('q') | KeyCode::Esc => app.tab1.close_pop(),
            // KeyCode::Char('p') if keyevent.modifiers == KeyModifiers::CONTROL => {
            //     app.tab1.toggle_menu();
            // },
            // KeyCode::Char('t') if keyevent.modifiers == KeyModifiers::CONTROL => {
            //     app.tab1.toggle_test_res();
            // },
            // KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
            //     app.tab1.toggle_submit_res();
            // },
            // KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
            //     app.get_qs_detail(IdSlug::Id(app.tab0.current_qs()), true);
            // },
            // KeyCode::Char('e')
            //     if !app.tab1.show_pop_menu
            //         && !app.tab1.show_test_res
            //         && !app.tab1.show_submit_res =>
            // {
            //     app.tab1.start_edit_code();
            // },
            // KeyCode::Char('^' | '0') if app.tab1.show_test_res => {
            //     app.tab1.test_res_view_head();
            // },
            // KeyCode::Char('^' | '0') if app.tab1.show_submit_res => {
            //     app.tab1.submit_res_view_head();
            // },
            // KeyCode::Char('h') => app.tab1.horizontal_scroll_h(),
            // KeyCode::Char('j') => app.tab1.vertical_scroll_j(),
            // KeyCode::Char('k') => app.tab1.vertical_scroll_k(),
            // KeyCode::Char('l') => app.tab1.horizontal_scroll_l(),
            // KeyCode::Char('g') => {
            //     if let Event::Key(key) = event::read().into_diagnostic()? {
            //         if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
            //             app.tab1.vertical_scroll_gg();
            //         }
            //     }
            // },
            // KeyCode::Char('G') => app.tab1.vertical_scroll_G(),
            _ => common_keymap(app, terminal, event, stdout).await?
        },
        _ => common_keymap(app, terminal, event, stdout).await?
    }

    Ok(())
}

pub async fn tab1_keymap_normal(
    app: &mut App<'_>,
    terminal: &mut Term,
    event: &Event,
) -> Result<()> {
    if let Event::Key(keyevent) = event {
        match keyevent.code {
            KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.save_code = true;
                terminal
                    .draw(|f| start_ui(f, app))
                    .into_diagnostic()?;
                app.save_code().await?;
                app.save_code = false;
            },
            KeyCode::Char('q') => app.tab1.edit_code = false,
            _ => vim_normal_map(event, app)?
        }
    }
    Ok(())
}

