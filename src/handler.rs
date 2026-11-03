use crate::{App, services, Button, ExecuteMode, SERVICES_SAFE, SERVICES_SAFE_SHORT, render_frame::{DARK_BLUE, LIGHT_BLUE, LIGHT_YELLOW}};
use ratatui::{layout::Rect, style::Color};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use color_eyre::eyre::{Context, Result}; // Or your Result type

impl App{ 
    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => 
                self.handle_key_event(key_event)
                    .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            Event::Mouse(mouse_event) =>
                self.handle_mouse_event(mouse_event)
                    .wrap_err_with(|| format!("handling mouse event failed:\n{mouse_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if self.search_toggle {
            self.key_event_search(key_event);
        }
        else if self.execute_toggle {
            self.key_event_execute(key_event);
        }
        else {
            self.key_event_main(key_event);
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        if self.search_toggle {
            match mouse_event.kind {
                MouseEventKind::ScrollUp => {
                    self.scroll_up_search();
                },
                MouseEventKind::ScrollDown => {
                    self.scroll_down_search();
                },
                _ => {}
            }
        }
        else if self.execute_toggle {
            match mouse_event.kind {
                // Button 
                MouseEventKind::Down(MouseButton::Left) => {
                    self.check_click_pos_execute(mouse_event);
                },
                MouseEventKind::ScrollUp => {
                    self.scroll_up();
                },
                MouseEventKind::ScrollDown => {
                    self.scroll_down();
                },
                _ => {}
            }
        }
        else {
            match mouse_event.kind {
                // Button 
                MouseEventKind::Down(MouseButton::Left) => {
                    self.check_click_pos(mouse_event);
                },
                MouseEventKind::ScrollUp => {
                    self.scroll_up();
                },
                MouseEventKind::ScrollDown => {
                    self.scroll_down();
                },
                _ => {}
            }
        }
        Ok(())
    }


    fn check_click_pos(&mut self, mouse_event: MouseEvent) {
        if is_inside(self.layout.chunks_right[1], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::Button1(Color::Green);
            self.select_telemetry();
        }
        else if is_inside(self.layout.chunks_right[2], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::Button2(Color::Red);
            self.selection_reset();
        }
    }

    fn selection_reset(&mut self) {
        if !self.services.selected.is_empty() {
            self.services.selected.clear();
            self.services.code_selected.clear();
        }
    }
    
    fn select_telemetry(&mut self) {
        if self.services.selected.is_empty() {
            self.services.selected.extend(SERVICES_SAFE.iter().map(|x| x.to_string()));
            self.services.code_selected.extend(SERVICES_SAFE_SHORT.iter().map(|x| x.to_string()));
        }
    }
    
    fn check_click_pos_execute(&mut self, mouse_event: MouseEvent) {
        if is_inside(self.layout.chunks_execute[0], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::ButtonAuto(Color::Red);
            self.execute_select = ExecuteMode::Auto;
        }
        else if is_inside(self.layout.chunks_execute[1], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::ButtonAutoDelayed(Color::Red);
            self.execute_select = ExecuteMode::AutoDelayed;
        }
        if is_inside(self.layout.chunks_execute[2], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::ButtonManual(Color::Red);
            self.execute_select = ExecuteMode::Manual;
        }
        if is_inside(self.layout.chunks_execute[3], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Button::ButtonDisable(Color::Red);
            self.execute_select = ExecuteMode::Disable;
        }
    }


    fn key_event_search(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.search_toggle = false;
            },
            KeyCode::Enter => {
                self.save_search_to_services();
                self.search_toggle = false;
            },
            KeyCode::Backspace => {
                self.search_string.pop();
                self.search.selected = self.search_services();
            },
            KeyCode::Up => {
                self.scroll_up_search();
            },
            KeyCode::Down => {
                self.scroll_down_search();
            },
            KeyCode::Char(c)  => {
                self.search_string.push(c);
                self.search.selected = self.search_services();
            },
            _ => {}
        }
    }

    fn save_search_to_services(&mut self) {
        if self.search_to_services_idx.len() > 0 {
            let idx = self.search_to_services_idx[self.search.idx];
            let search_selected = self.services_name_cache[idx].clone();
            self.services.selected.push(search_selected);
        }
    }
    
    fn key_event_execute(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.execute_toggle = false;
            },
            KeyCode::Enter => {
                self.execute_command();
            },
            KeyCode::Char('1') => {
                self.execute_select = ExecuteMode::Auto;
                self.mouse_pos = Button::ButtonAuto(Color::Red);
            },
            KeyCode::Char('2') => {
                self.execute_select = ExecuteMode::AutoDelayed;
                self.mouse_pos = Button::ButtonAutoDelayed(Color::Red);
            },
            KeyCode::Char('3') => {
                self.execute_select = ExecuteMode::Manual;
                self.mouse_pos = Button::ButtonManual(Color::Red);
            },
            KeyCode::Char('4') => {
                self.execute_select = ExecuteMode::Disable;
                self.mouse_pos = Button::ButtonDisable(Color::Red);
            }
            _ => {         
                self.execute_select = ExecuteMode::default();
                self.mouse_pos = Button::default();
            }
        }
    } 

    fn execute_command(&mut self) {
        match self.execute_select {
            ExecuteMode::Auto => {
                if !self.check_services_selection_empty() {
                    self.cmd_output_disable = services::enable_auto(&self.services.code_selected)
                        .unwrap_or_else(|e| e);
                }
                else {
                    self.cmd_output_disable = String::from("No service selected.");
                }
            }
            ExecuteMode::AutoDelayed => {
                if !self.check_services_selection_empty() {
                    self.cmd_output_disable = services::enable_auto_delayed(&self.services.code_selected)
                        .unwrap_or_else(|e| e);
                }
                else {
                    self.cmd_output_disable = String::from("No service selected.");
                }
            }
            ExecuteMode::Manual => {
                if !self.check_services_selection_empty() {
                    self.cmd_output_disable = services::enable_manual(&self.services.code_selected)
                        .unwrap_or_else(|e| e);
                }
                else {
                    self.cmd_output_disable = String::from("No service selected.");
                }            }
            ExecuteMode::Disable => {
                if !self.check_services_selection_empty() {
                    self.cmd_output_stop = services::stop_services(&self.services.code_selected)
                        .unwrap_or_else(|e| e);
                    self.cmd_output_disable = services::disable_services(&self.services.code_selected)
                        .unwrap_or_else(|e| e);
                }
                else {
                    self.cmd_output_disable = String::from("No service selected.");
                }            
            }
            ExecuteMode::Default => {
                self.cmd_output_disable = String::from("No option selected.");
            }
        }
        self.execute_toggle = false;
    }

    fn key_event_main(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.exit()
            },
            KeyCode::Up | KeyCode::Char('w') => {
                self.scroll_up();
            },
            KeyCode::Down | KeyCode::Char('s') => {
                self.scroll_down();
            },
            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter => {
                let selection = &self.services_name_cache[self.services.idx].to_string();
                if !check_dupe(&self.services.selected, selection) {
                    self.services.selected.push((self.services_name_cache[self.services.idx]).to_string());
                    self.services.code_selected.push((self.services_name_short_cache[self.services.idx]).to_string());
                }
            },
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.services.selected.pop();
                self.services.code_selected.pop();
            },
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.search_toggle = true;
                self.search.selected = self.search_services();
            },
            KeyCode::Char('1') => {
                self.mouse_pos = Button::Button1(Color::Green);
                self.select_telemetry();
            },
            KeyCode::Char('2') => {
                self.mouse_pos = Button::Button2(Color::Red);
                self.selection_reset();
            },            
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.execute_toggle = true;
            },
            _ => {}
        }            
    }

    fn scroll_up(&mut self) {
        if self.services.idx > 0 {
            self.services.idx = self.services.idx.saturating_sub(1);
            self.services.list_state.select_previous();
        }
        else {
            self.services.idx = self.services_name_cache.len() - 1;
            self.services.list_state.select_last();
        }
    }

    fn scroll_down(&mut self) {
        if check_scrollbar_within_bounds(self.services_name_cache.len(), self.services.idx) {
            self.services.idx = self.services.idx.saturating_add(1);
            self.services.list_state.select_next();
        }
        else {
            self.services.idx = 0;
            self.services.list_state.select_first();
        }
    }

    fn scroll_up_search(&mut self) {
        if self.search.idx > 0 {
            self.search.idx = self.search.idx.saturating_sub(1);
            self.search.list_state.select_previous();
        }
        else {
            self.search.idx = self.search.selected.len() - 1;
            self.search.list_state.select_last();
        }
    }

    fn scroll_down_search(&mut self) {
        if check_scrollbar_within_bounds(self.search.selected.len(), self.search.idx) {
            self.search.idx = self.search.idx.saturating_add(1);
            self.search.list_state.select_next();
        }
        else {
            self.search.idx = 0;
            self.search.list_state.select_first();
        }
    }

    fn search_services(&mut self) -> Vec<String> {
        self.search.idx = 0;
        self.search.list_state.select(Some(0));
        let query_lower: String = self.search_string.iter().collect();
        let services_name = self.services_name_cache.clone();
         self.search_to_services_idx = services_name.iter()
            .enumerate()
            .filter(|(_, s)| s.to_lowercase().contains(&query_lower))
            .map(|(idx, _)| idx)
            .collect();
        services_name.iter()
            .filter(|s| s.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    fn check_services_selection_empty(&self) -> bool {
        self.services.selected.len() <= 0
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}


fn check_scrollbar_within_bounds(len: usize, idx: usize) -> bool {
    idx < len -1
}

fn is_inside(rect: Rect, col: u16, row: u16) -> bool {
    col >= rect.x && col < rect.x + rect.width &&
    row >= rect.y && row < rect.y + rect.height
}

fn check_dupe(select_list: &Vec<String>, selection: &String) -> bool {
    select_list.contains(selection)
}
