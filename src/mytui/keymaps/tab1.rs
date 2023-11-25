use std::io::Stdout;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::Backend, Terminal};
use tui_textarea::{CursorMove, Input, Key, Scrolling};

use crate::{
    leetcode::IdSlug,
    mytui::{
        app::{App, InputMode},
        myevent::UserEvent,
        ui::start_ui,
    },
};

use super::common_keymap;

pub async fn init<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    if app.tab1.edit_code {
        match app.tab1.code_block_mode {
            InputMode::Insert => tab1_keymap_insert(app, terminal, event, stdout),
            InputMode::Normal => tab1_keymap_normal(app, terminal, event, stdout).await?,
        }
    } else {
        tab1_keymap(app, terminal, event, stdout).await?;
    }

    Ok(())
}

pub async fn tab1_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('S') if app.tab1.show_pop_menu => app.submit_code()?,
            KeyCode::Char('T') if app.tab1.show_pop_menu => app.test_code()?,
            KeyCode::Char('q') | KeyCode::Esc => app.tab1.close_pop(),
            KeyCode::Char('p') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab1.toggle_menu();
            }
            KeyCode::Char('t') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab1.toggle_test_res();
            }
            KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tab1.toggle_submit_res();
            }
            KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tx
                    .send(UserEvent::GetQs((IdSlug::Id(app.tab0.current_qs()), true)))
                    .into_diagnostic()?;
            }
            KeyCode::Char('e')
                if !app.tab1.show_pop_menu
                    && !app.tab1.show_test_res
                    && !app.tab1.show_submit_res =>
            {
                app.tab1.start_edit_code();
            }
            KeyCode::Char('^' | '0') if app.tab1.show_test_res => {
                app.tab1.test_res_view_head();
            }
            KeyCode::Char('^' | '0') if app.tab1.show_submit_res => {
                app.tab1.submit_res_view_head();
            }
            KeyCode::Char('h') => app.tab1.horizontal_scroll_h(),
            KeyCode::Char('j') => app.tab1.vertical_scroll_j(),
            KeyCode::Char('k') => app.tab1.vertical_scroll_k(),
            KeyCode::Char('l') => app.tab1.horizontal_scroll_l(),
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                        app.tab1.vertical_scroll_gg();
                    }
                }
            }
            KeyCode::Char('G') => app.tab1.vertical_scroll_G(),
            _ => common_keymap(app, terminal, event, stdout).await?,
        },
        _ => common_keymap(app, terminal, event, stdout).await?,
    }

    Ok(())
}

pub fn tab1_keymap_insert<B: Backend>(
    app: &mut App<'_>,
    _terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
) {
    match event.clone().into() {
        Input { key: Key::Esc, .. } => app.tab1.be_code_normal(),
        input => {
            app.tab1.code_block.input(input); // Use default key mappings in insert mode(emacs)
        }
    }
}

pub async fn tab1_keymap_normal<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
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
            }
            KeyCode::Char('q') => app.tab1.edit_code = false,
            _ => vim_normal_map(event, app)?,
        }
    }
    Ok(())
}

fn vim_normal_map(event: &Event, app: &mut App) -> Result<(), miette::ErrReport> {
    match event.clone().into() {
        // Mappings in normal mode
        Input {
            key: Key::Char('d'),
            ctrl: false,
            ..
        } => {
            if let Event::Key(keyevent) = event::read().into_diagnostic()? {
                if keyevent.kind == KeyEventKind::Press {
                    match keyevent.code {
                        KeyCode::Char('d') => {
                            app.tab1
                                .code_block
                                .move_cursor(CursorMove::Head);
                            app.tab1
                                .code_block
                                .delete_line_by_end();
                            app.tab1
                                .code_block
                                .delete_next_char();
                        }
                        KeyCode::Char('w') => {
                            app.tab1
                                .code_block
                                .delete_next_word();
                        }
                        _ => {}
                    }
                }
            }
        }
        Input {
            key: Key::Char('g'),
            ..
        } => {
            if let Event::Key(key) = event::read().into_diagnostic()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('g') {
                    app.tab1
                        .code_block
                        .move_cursor(CursorMove::Top);
                }
            }
        }
        Input {
            key: Key::Char('G'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Bottom),
        Input {
            key: Key::Char('h'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Back),
        Input {
            key: Key::Char('j'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Down),
        Input {
            key: Key::Char('k'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Up),
        Input {
            key: Key::Char('l'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Forward),
        Input {
            key: Key::Char('w'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::WordForward),
        Input {
            key: Key::Char('b'),
            ctrl: false,
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::WordBack),
        Input {
            key: Key::Char('^' | '0'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::Head),
        Input {
            key: Key::Char('$'),
            ..
        } => app
            .tab1
            .code_block
            .move_cursor(CursorMove::End),
        Input {
            key: Key::Char('D'),
            ..
        } => {
            app.tab1
                .code_block
                .delete_line_by_end();
        }
        Input {
            key: Key::Char('C'),
            ..
        } => {
            app.tab1
                .code_block
                .delete_line_by_end();
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('p'),
            ..
        } => {
            app.tab1.code_block.paste();
        }
        Input {
            key: Key::Char('u'),
            ctrl: false,
            ..
        } => {
            app.tab1.code_block.undo();
        }
        Input {
            key: Key::Char('r'),
            ctrl: true,
            ..
        } => {
            app.tab1.code_block.redo();
        }
        Input {
            key: Key::Char('x'),
            ..
        } => {
            app.tab1
                .code_block
                .delete_next_char();
        }
        Input {
            key: Key::Char('i'),
            ..
        } => app.tab1.code_block_mode = InputMode::Insert,
        Input {
            key: Key::Char('a'),
            ..
        } => {
            app.tab1
                .code_block
                .move_cursor(CursorMove::Forward);
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('A'),
            ..
        } => {
            app.tab1
                .code_block
                .move_cursor(CursorMove::End);
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('o'),
            ..
        } => {
            app.tab1
                .code_block
                .move_cursor(CursorMove::End);
            app.tab1
                .code_block
                .insert_newline();
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('O'),
            ..
        } => {
            app.tab1
                .code_block
                .move_cursor(CursorMove::Head);
            app.tab1
                .code_block
                .insert_newline();
            app.tab1
                .code_block
                .move_cursor(CursorMove::Up);
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('I'),
            ..
        } => {
            app.tab1
                .code_block
                .move_cursor(CursorMove::Head);
            app.tab1.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('e'),
            ctrl: true,
            ..
        } => app.tab1.code_block.scroll((1, 0)),
        Input {
            key: Key::Char('y'),
            ctrl: true,
            ..
        } => app.tab1.code_block.scroll((-1, 0)),
        Input {
            key: Key::Char('d'),
            ctrl: true,
            ..
        } => app
            .tab1
            .code_block
            .scroll(Scrolling::HalfPageDown),
        Input {
            key: Key::Char('u'),
            ctrl: true,
            ..
        } => app
            .tab1
            .code_block
            .scroll(Scrolling::HalfPageUp),
        Input {
            key: Key::Char('f'),
            ctrl: true,
            ..
        } => app
            .tab1
            .code_block
            .scroll(Scrolling::PageDown),
        Input {
            key: Key::Char('b'),
            ctrl: true,
            ..
        } => app
            .tab1
            .code_block
            .scroll(Scrolling::PageUp),

        Input {
            key: Key::Char('q'),
            ..
        } => app.tab1.edit_code = false,
        _ => {}
    };
    Ok(())
}
