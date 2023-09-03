use crate::mytui::{
    app::{App, InputMode},
    myevent::UserEvent,
    ui::start_ui,
};

use super::common_keymap;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::{self, KeyEventKind};
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::Backend;
use ratatui::Terminal;
use std::io::Stdout;
use tui_textarea::CursorMove;
use tui_textarea::Input;
use tui_textarea::Key;
use tui_textarea::Scrolling;

pub async fn init<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    stdout: &mut Stdout,
) -> Result<()> {
    if app.edit_code {
        match app.code_block_mode {
            InputMode::Insert => {
                tab1_keymap_insert(app, terminal, &event, stdout).await?;
            }
            InputMode::Normal => {
                tab1_keymap_normal(app, terminal, &event, stdout).await?;
            }
        }
    } else {
        tab1_keymap(app, terminal, &event, stdout).await?;
    }

    Ok(())
}

pub async fn tab1_keymap<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('S') if app.pop_menu => {
                app.tx
                    .send(UserEvent::SubmitCode)
                    .into_diagnostic()?;
                app.submiting = true;
            }
            KeyCode::Char('T') if app.pop_menu => {
                app.tx
                    .send(UserEvent::TestCode)
                    .into_diagnostic()?;
                app.submiting = true;
            }
            KeyCode::Char('q') if app.show_submit_res => {
                app.show_submit_res = false;
            }
            KeyCode::Char('q') if app.show_test_res => {
                app.show_test_res = false;
            }
            KeyCode::Char('p') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.pop_menu = !app.pop_menu;
            }
            KeyCode::Char('t') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.show_test_res = !app.show_test_res;
            }
            KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.show_submit_res = !app.show_submit_res;
            }
            KeyCode::Char('r') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.tx
                    .send(UserEvent::GetQs((app.current_qs(), true)))
                    .into_diagnostic()?;
            }
            KeyCode::Char('e') => app.edit_code = true,
            KeyCode::Char('j') => {
                if app.vertical_scroll
                    < app
                        .vertical_row_len
                        .saturating_sub(4)
                {
                    app.vertical_scroll = app
                        .vertical_scroll
                        .saturating_add(1);
                }
                app.vertical_scroll_state = app
                    .vertical_scroll_state
                    .position(app.vertical_scroll as u16);
            }
            KeyCode::Char('k') => {
                app.vertical_scroll = app
                    .vertical_scroll
                    .saturating_sub(1);
                app.vertical_scroll_state = app
                    .vertical_scroll_state
                    .position(app.vertical_scroll as u16);
            }
            KeyCode::Char('g') => {
                if let Event::Key(key) = event::read().into_diagnostic()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('g') => {
                                app.vertical_scroll = 0;
                                app.vertical_scroll_state = app
                                    .vertical_scroll_state
                                    .position(app.vertical_scroll as u16);
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Char('G') => {
                app.vertical_scroll = app
                    .vertical_row_len
                    .saturating_sub(4);
                app.vertical_scroll_state = app
                    .vertical_scroll_state
                    .position(app.vertical_scroll as u16);
            }
            KeyCode::Char('h') => {
                app.horizontal_scroll = app
                    .horizontal_scroll
                    .saturating_sub(1);
                app.horizontal_scroll_state = app
                    .horizontal_scroll_state
                    .position(app.horizontal_scroll as u16);
            }
            KeyCode::Char('l') => {
                if app.horizontal_scroll
                    < app
                        .horizontal_col_len
                        .saturating_sub(4)
                {
                    app.horizontal_scroll = app
                        .horizontal_scroll
                        .saturating_add(1);
                }
                app.horizontal_scroll_state = app
                    .horizontal_scroll_state
                    .position(app.horizontal_scroll as u16);
            }
            _ => {
                common_keymap(app, terminal, event, _stdout).await?;
            }
        },
        _ => {
            common_keymap(app, terminal, event, _stdout).await?;
        }
    }

    Ok(())
}

pub async fn tab1_keymap_insert<B: Backend>(
    app: &mut App<'_>,
    _terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
) -> Result<()> {
    match event.clone().into() {
        Input { key: Key::Esc, .. } => {
            app.code_block_mode = InputMode::Normal;
        }
        input => {
            app.code_block.input(input); // Use default key mappings in insert mode(emacs)
        }
    }

    Ok(())
}

pub async fn tab1_keymap_normal<B: Backend>(
    app: &mut App<'_>,
    terminal: &mut Terminal<B>,
    event: &Event,
    _stdout: &mut Stdout,
) -> Result<()> {
    match event {
        Event::Key(keyevent) => match keyevent.code {
            KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
                app.save_code = true;
                terminal
                    .draw(|f| start_ui(f, app))
                    .into_diagnostic()?;
                app.save_code().await?;
                app.save_code = false;
            }
            KeyCode::Char('q') => app.edit_code = false,
            _ => {
                vim_normal_map(event, app)?;
            }
        },
        _ => {}
    }
    Ok(())
}

fn vim_normal_map(event: &Event, app: &mut App) -> Result<(), miette::ErrReport> {
    Ok(match event.clone().into() {
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
                            app.code_block
                                .move_cursor(CursorMove::Head);
                            app.code_block.delete_line_by_end();
                            app.code_block.delete_next_char();
                        }
                        KeyCode::Char('w') => {
                            app.code_block.delete_next_word();
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
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('g') => app
                            .code_block
                            .move_cursor(CursorMove::Top),
                        _ => {}
                    }
                }
            }
        }
        Input {
            key: Key::Char('G'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Bottom),
        Input {
            key: Key::Char('h'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Back),
        Input {
            key: Key::Char('j'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Down),
        Input {
            key: Key::Char('k'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Up),
        Input {
            key: Key::Char('l'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Forward),
        Input {
            key: Key::Char('w'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::WordForward),
        Input {
            key: Key::Char('b'),
            ctrl: false,
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::WordBack),
        Input {
            key: Key::Char('^'),
            ..
        }
        | Input {
            key: Key::Char('0'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::Head),
        Input {
            key: Key::Char('$'),
            ..
        } => app
            .code_block
            .move_cursor(CursorMove::End),
        Input {
            key: Key::Char('D'),
            ..
        } => {
            app.code_block.delete_line_by_end();
        }
        Input {
            key: Key::Char('C'),
            ..
        } => {
            app.code_block.delete_line_by_end();
            app.code_block_mode = InputMode::Insert;
        }
        Input {
            key: Key::Char('p'),
            ..
        } => {
            app.code_block.paste();
        }
        Input {
            key: Key::Char('u'),
            ctrl: false,
            ..
        } => {
            app.code_block.undo();
        }
        Input {
            key: Key::Char('r'),
            ctrl: true,
            ..
        } => {
            app.code_block.redo();
        }
        Input {
            key: Key::Char('x'),
            ..
        } => {
            app.code_block.delete_next_char();
        }
        Input {
            key: Key::Char('i'),
            ..
        } => app.code_block_mode = InputMode::Insert,
        Input {
            key: Key::Char('a'),
            ..
        } => {
            app.code_block
                .move_cursor(CursorMove::Forward);
            app.code_block_mode = InputMode::Insert
        }
        Input {
            key: Key::Char('A'),
            ..
        } => {
            app.code_block
                .move_cursor(CursorMove::End);
            app.code_block_mode = InputMode::Insert
        }
        Input {
            key: Key::Char('o'),
            ..
        } => {
            app.code_block
                .move_cursor(CursorMove::End);
            app.code_block.insert_newline();
            app.code_block_mode = InputMode::Insert
        }
        Input {
            key: Key::Char('O'),
            ..
        } => {
            app.code_block
                .move_cursor(CursorMove::Head);
            app.code_block.insert_newline();
            app.code_block
                .move_cursor(CursorMove::Up);
            app.code_block_mode = InputMode::Insert
        }
        Input {
            key: Key::Char('I'),
            ..
        } => {
            app.code_block
                .move_cursor(CursorMove::Head);
            app.code_block_mode = InputMode::Insert
        }
        Input {
            key: Key::Char('e'),
            ctrl: true,
            ..
        } => app.code_block.scroll((1, 0)),
        Input {
            key: Key::Char('y'),
            ctrl: true,
            ..
        } => app.code_block.scroll((-1, 0)),
        Input {
            key: Key::Char('d'),
            ctrl: true,
            ..
        } => app
            .code_block
            .scroll(Scrolling::HalfPageDown),
        Input {
            key: Key::Char('u'),
            ctrl: true,
            ..
        } => app
            .code_block
            .scroll(Scrolling::HalfPageUp),
        Input {
            key: Key::Char('f'),
            ctrl: true,
            ..
        } => app
            .code_block
            .scroll(Scrolling::PageDown),
        Input {
            key: Key::Char('b'),
            ctrl: true,
            ..
        } => app
            .code_block
            .scroll(Scrolling::PageUp),

        Input {
            key: Key::Char('q'),
            ..
        } => app.edit_code = false,
        _ => {}
    })
}
