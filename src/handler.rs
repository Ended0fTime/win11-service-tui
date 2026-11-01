use crate::{App, services, ButtonHighlight, ButtonIdent, SERVICES_SAFE, SERVICES_SAFE_SHORT, render_frame::{DARK_BLUE, LIGHT_BLUE, LIGHT_YELLOW}};
use ratatui::{layout::Rect, style::Color};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use color_eyre::eyre::{Context, Result}; // Or your Result type

impl App{ 
    // Input Handling
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
        if !self.search_toggle {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.exit()
                },
                KeyCode::Up | KeyCode::Char('w') => {
                    self.services.idx = self.services.idx.saturating_sub(1);
                    self.services.list_state.select_previous();
                },
                KeyCode::Down | KeyCode::Char('s') => {
                    if check_scrollbar_within_bounds(self.services_name_cache.len(), self.services.idx) {
                        self.services.idx = self.services.idx.saturating_add(1);
                        self.services.list_state.select_next();
                    }
                },
                KeyCode::Char('e') | KeyCode::Char('E') => {
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
                KeyCode::Char('m') => {
                    self.cmd_output_stop = services::stop_services(&self.services.code_selected).unwrap_or_else(|e| e);
                    self.cmd_output_disable = services::disable_services(&self.services.code_selected).unwrap_or_else(|e| e);
                },
                _ => {}
            }
        }
        else {
            match key_event.code {
                KeyCode::Enter => {
                    if self.search_to_services_idx.len() > 0 {
                        let idx = self.search_to_services_idx[self.search.idx];
                        let search_selected = self.services_name_cache[idx].clone();
                        self.services.selected.push(search_selected);
                    }
                    self.search_toggle = false;
                },
                KeyCode::Backspace => {
                    self.search_string.pop();
                    self.search.selected = self.search_services();
                }
                KeyCode::Up => {
                    self.search.idx = self.search.idx.saturating_sub(1);
                    self.search.list_state.select_previous();
                },
                KeyCode::Down => {
                    if check_scrollbar_within_bounds(self.search_to_services_idx.len(), self.search.idx) {
                        self.search.idx = self.search.idx.saturating_add(1);
                        self.search.list_state.select_next();
                    }
                },
                KeyCode::Char(c)  => {
                    self.search_string.push(c);
                    self.search.selected = self.search_services();
                }
                _ => {}
            }

        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        if !self.search_toggle {
            match mouse_event.kind {
                // Button 
                MouseEventKind::Down(MouseButton::Left) => {
                    if is_inside(self.layout.chunks_right[1], mouse_event.column, mouse_event.row) {
                        self.mouse_pos = ButtonIdent::Button1(ButtonHighlight::Pressed(Color::Green));
                        if self.services.selected.is_empty() {
                            self.services.selected.extend(SERVICES_SAFE.iter().map(|x| x.to_string()));
                            self.services.code_selected.extend(SERVICES_SAFE_SHORT.iter().map(|x| x.to_string()));
                        }
                    }
                    else if is_inside(self.layout.chunks_right[2], mouse_event.column, mouse_event.row) {
                        self.mouse_pos = ButtonIdent::Button2(ButtonHighlight::Pressed(Color::Red));
                        if !self.services.selected.is_empty() {
                            self.services.selected.clear();
                            self.services.code_selected.clear();
                        }
                    }
                },
                MouseEventKind::Up(MouseButton::Left) => {
                    self.mouse_pos = ButtonIdent::default();
                },
                MouseEventKind::ScrollUp => {
                    self.services.idx = self.services.idx.saturating_sub(1);
                    self.services.list_state.select_previous();
                },
                MouseEventKind::ScrollDown => {
                    if check_scrollbar_within_bounds(self.services_name_cache.len(), self.services.idx) {
                        self.services.idx = self.services.idx.saturating_add(1);
                        self.services.list_state.select_next();
                    }
                },
                _ => {}
            }
        }
        Ok(())
    }
    

    pub fn search_services(&mut self) -> Vec<String> {
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

pub fn check_dupe(select_list: &Vec<String>, selection: &String) -> bool {
    select_list.contains(selection)
}
