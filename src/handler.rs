use crate::{App, ButtonHighlight, ButtonIdent, SERVICES_SAFE, SERVICES_SAFE_SHORT, SERVICES, SERVICES_SHORT, render_frame::{DARK_BLUE, LIGHT_BLUE, LIGHT_YELLOW}};
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
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.exit()
            },
            KeyCode::Up | KeyCode::Char('w') => {
                self.services_scrollbar_index = self.services_scrollbar_index.saturating_sub(1);
                self.services_list_state.select_previous();
            },
            KeyCode::Down | KeyCode::Char('s') => {
                if self.check_scrollbar_within_bounds() {
                    self.services_scrollbar_index = self.services_scrollbar_index.saturating_add(1);
                    self.services_list_state.select_next();
                }
            },
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let selection = &SERVICES[self.services_scrollbar_index].to_string();
                if !self.check_services_dupe(selection) {
                    self.services_selected.push((self.services_name_cache[self.services_scrollbar_index]).to_string());
                    self.services_code_selected.push((self.services_name_short_cache[self.services_scrollbar_index]).to_string());
                }
            },
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.services_selected.pop();
                self.services_code_selected.pop();
            },
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.search_toggle = true;
            }
            KeyCode::Enter => {
                self.disable_services();
            },

            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match mouse_event.kind {
            // Button 
            MouseEventKind::Down(MouseButton::Left) => {
                if is_inside(self.chunks_left_cache[1], mouse_event.column, mouse_event.row) {
                    self.mouse_pos = ButtonIdent::Button1(ButtonHighlight::Pressed(Color::Green));
                    if self.services_selected.is_empty() {
                        self.services_selected.extend(SERVICES_SAFE.iter().map(|x| x.to_string()));
                        self.services_code_selected.extend(SERVICES_SAFE_SHORT.iter().map(|x| x.to_string()));
                    }
                }
                else if is_inside(self.chunks_left_cache[2], mouse_event.column, mouse_event.row) {
                    self.mouse_pos = ButtonIdent::Button2(ButtonHighlight::Pressed(Color::Red));
                    if !self.services_selected.is_empty() {
                        self.services_selected.clear();
                        self.services_code_selected.clear();
                    }
                }
            },
            MouseEventKind::Up(MouseButton::Left) => {
                self.mouse_pos = ButtonIdent::default();   // Default button case
            },
            // List scroll
            MouseEventKind::ScrollUp => {
                self.services_scrollbar_index = self.services_scrollbar_index.saturating_sub(1);
                self.services_list_state.select_previous();
            },
            MouseEventKind::ScrollDown => {
                if self.check_scrollbar_within_bounds() {
                    self.services_scrollbar_index = self.services_scrollbar_index.saturating_add(1);
                    self.services_list_state.select_next();
                }
            },
            
            _ => {}
        }

        Ok(())
    }

    fn check_services_dupe (&self, selection: &String) -> bool {
        self.services_selected.contains(selection)
    }

    fn check_scrollbar_within_bounds (&self) -> bool {
        self.services_scrollbar_index < self.services_count_num.len() - 1
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}


// Purely mathematical funcs
fn is_inside(rect: Rect, col: u16, row: u16) -> bool {
    col >= rect.x && col < rect.x + rect.width &&
    row >= rect.y && row < rect.y + rect.height
}