use crate::{App, ButtonHighlight, ButtonIdent, render_frame::{LIGHT_BLUE, LIGHT_YELLOW, DARK_BLUE}};
use ratatui::{layout::Rect, style::Color};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use color_eyre::eyre::{Context, Result}; // Or your Result type

impl App{ 
    /// Input Handling
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
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),     //Exit term key event
            KeyCode::Up => {
                self.services_scrollbar_index = self.services_scrollbar_index.saturating_sub(1);
                self.services_list_state.select_previous();
            },
            KeyCode::Down => {
                if !(self.services_scrollbar_index > 10) {
                    self.services_scrollbar_index = self.services_scrollbar_index.saturating_add(1);
                    self.services_list_state.select_next();
                }
            },
            KeyCode::Enter => self.disable_services(),

            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match mouse_event.kind {
            // Button 
            MouseEventKind::Down(MouseButton::Left) => {
                // if is_inside(self.chunks_left_cache[8][1], mouse_event.column, mouse_event.row) {
                //     self.mouse_pos = ButtonIdent::Exit(ButtonHighlight::Pressed(LIGHT_BLUE));
                //     self.exit_clicked = true;
                // }
                if is_inside(self.chunks_left_cache[2][1], mouse_event.column, mouse_event.row) {
                    self.mouse_pos = ButtonIdent::Button1(ButtonHighlight::Pressed(Color::Green));
                }
                else if is_inside(self.chunks_left_cache[5][1], mouse_event.column, mouse_event.row) {
                    self.mouse_pos = ButtonIdent::Button2(ButtonHighlight::Pressed(Color::Red));
                }
            },
            MouseEventKind::Up(MouseButton::Left) => {
                if !self.exit_clicked {
                    self.mouse_pos = ButtonIdent::default();   // Default button case
                }
                else {
                    self.exit();
                }     
            },
            // List scroll
            MouseEventKind::ScrollUp => {
                self.services_scrollbar_index = self.services_scrollbar_index.saturating_sub(1);
                self.services_list_state.select_previous();
            },
            MouseEventKind::ScrollDown => {
                if !(self.services_scrollbar_index > 10) {
                    self.services_scrollbar_index = self.services_scrollbar_index.saturating_add(1);
                    self.services_list_state.select_next();
                }
            },
            
            _ => {}
        }

        Ok(())
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