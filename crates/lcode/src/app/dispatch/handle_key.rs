use crossterm::event::{Event as CrossEvent, KeyCode, KeyEvent, KeyModifiers};
use lcode_config::{global::G_USER_CONFIG, keymap::*};
use miette::Result;

use crate::app::{inner::App, Tab2Panel, TuiIndex, TuiMode};

impl<'app_lf> App<'app_lf> {
    pub async fn handle_key(&mut self, keyevent: KeyEvent) {
        let temp = if matches!(self.tab_index, TuiIndex::Select)
            && matches!(self.select.input_line_mode, TuiMode::Insert)
        {
            self.select
                .keymap_insert(CrossEvent::Key(keyevent))
        }
        else if matches!(self.tab_index, TuiIndex::Topic)
            && matches!(self.topic.input_line_mode, TuiMode::Insert)
        {
            self.topic
                .keymap_insert(CrossEvent::Key(keyevent))
        }
        else if matches!(self.tab_index, TuiIndex::Edit) {
            match self.edit.code_block_mode {
                TuiMode::Normal => match keyevent.code {
                    KeyCode::Char('s') if keyevent.modifiers == KeyModifiers::CONTROL => {
                        self.save_code().await.ok();
                        true
                    },
                    KeyCode::Esc => {
                        self.save_code = false;
                        true
                    },
                    _ => self
                        .edit
                        .normal_map(CrossEvent::Key(keyevent)),
                },
                TuiMode::Insert => self
                    .edit
                    .insert_keymap(CrossEvent::Key(keyevent)),
                TuiMode::Visual => unreachable!(),
                TuiMode::OutEdit => false,
            }
        }
        else {
            false
        };
        if temp {
            self.render();
            return;
        }

        if self.next_key.have_next() {
            if let Some(action) = self.next_key.handle_key(keyevent) {
                self.do_action(action)
                    .await
                    .expect("have_next do action failed");
                self.render();
            }
            return;
        }
        for KeyMap { keys, action: r#do, .. } in &G_USER_CONFIG.keymap.keymap {
            if keys.is_empty() || keys[0] != keyevent.into() {
                continue;
            }
            if keys.len() > 1 {
                self.next_key.store_next(keyevent);
            }
            else {
                self.do_action(r#do)
                    .await
                    .expect("do action failed");
            }
        }

        self.render();
    }
    /// do a action
    pub async fn do_action(&mut self, action: &str) -> Result<()> {
        let cond = match self.tab_index {
            TuiIndex::Select if matches!(self.select.input_line_mode, TuiMode::OutEdit) => {
                match action {
                    UP => self.select.prev_qs(),
                    DOWN => self.select.next_qs(),
                    SYNC_INDEX => self.sync_index(),
                    EDIT_IN_TUI => self.select.edit(),
                    TOGGLE_CURSOR => self.goto_tab(TuiIndex::Edit),
                    TOP => self.select.first_qs(),
                    BOTTOM => self.select.last_qs(),
                    EDIT_CODE_EDITOR => self.select_edit_cur_qs().await.is_ok(),
                    _ => false,
                }
            },
            TuiIndex::Edit if matches!(self.edit.code_block_mode, TuiMode::OutEdit) => match action
            {
                UP => self.edit.vertical_scroll_k(),
                DOWN => self.edit.vertical_scroll_j(),
                LEFT => self.edit.horizontal_scroll_h(),
                RIGHT => self.edit.horizontal_scroll_l(),
                TOP => self.edit.vertical_scroll_gg(),
                BOTTOM => self.edit.vertical_scroll_G(),
                HEAD => self.edit.goto_pop_head(),
                EDIT_IN_TUI => self.edit.start_edit_tui(),
                EDIT_CODE_EDITOR => self
                    .edit_tab_edit_with_editor()
                    .await
                    .is_ok(),

                TOGGLE_CURSOR if self.edit.show_pop_menu => self.menu_button_trig(),

                TOGGLE_MENU => self.edit.toggle_menu(),
                TOGGLE_SUBMIT_RES => self.edit.toggle_submit_res(),
                TOGGLE_TEST_RES => self.edit.toggle_test_res(),
                ESCAPE => {
                    if self.save_code {
                        self.save_code = false;
                    }
                    self.edit.close_pop()
                },
                _ => false,
            },
            TuiIndex::Topic if matches!(self.topic.input_line_mode, TuiMode::OutEdit) => {
                match action {
                    UP => self.topic.up(),
                    DOWN => self.topic.down(),
                    EDIT_IN_TUI => self.topic.enter_input_line(),
                    EDIT_CODE_EDITOR => self.topic_edit_cur_qs().await.is_ok(),

                    TOP => self.topic.top(),
                    BOTTOM => self.topic.bottom(),

                    // GOTO_EDIT => self.goto_tab(TabIndex::Tab1),
                    TOGGLE_CURSOR => {
                        if matches!(self.topic.index, Tab2Panel::Questions) {
                            self.goto_tab(TuiIndex::Edit);
                            return Ok(());
                        }
                        self.topic.toggle_cursor().await
                    },

                    PANEL_UP => self.topic.panel_up(),
                    PANEL_DOWN => self.topic.panel_down(),
                    PANEL_LEFT => self.topic.panel_left(),
                    PANEL_RIGHT => self.topic.panel_right(),

                    SYNC_INDEX => self.sync_new(),
                    _ => false,
                }
            },
            TuiIndex::Info => match action {
                UP => self.info.prev_item(),
                DOWN => self.info.next_item(),
                TOP => self.info.first_item(),
                BOTTOM => self.info.last_item(),
                TOGGLE_CURSOR => self.info.trigger(),
                _ => false,
            },
            _ => false,
        };
        if cond {
            return Ok(());
        }

        // common command
        match action {
            NEXT_TAB => self.next_tab(),
            PREV_TAB => self.prev_tab(),
            REDRAW => {
                self.events.redraw_tui();
                false
            },
            EXIT => self.exit(),
            _ => false,
        };
        Ok(())
    }
}
