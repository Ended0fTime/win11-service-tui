use crate::{App, Button, ExecuteMode, UiMode, services};
use ratatui::layout::{Rect, Position};
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use color_eyre::eyre::{Context, Result}; 


// Text contents Pre-defining
const SERVICES_SAFE:&[&str] = &[
    "Connected User Experiences and Telemetry",
    "Downloaded Maps Manager",
    "Geolocation Service",
    "Microsoft Usage and Quality Insights",
    "Windows Health and Optimized Experiences",
    "Windows Insider Service"
];

const SERVICES_SAFE_SHORT:&[&str] = &[
    "DiagTrack",
    "MapsBroker",
    "lfsvc",
    "wuqisvc",
    "whesvc",
    "wisvc"
];


impl App{ 
    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press =>   //KeyEventKind::Release doesnt work on cmd shell
                self.handle_key_event(key_event).wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            Event::Mouse(mouse_event) =>
                self.handle_mouse_event(mouse_event).wrap_err_with(|| format!("handling mouse event failed:\n{mouse_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.ui_state {
            UiMode::Search => {
                self.key_event_search(key_event);
            }
            UiMode::Execute => {
                self.key_event_execute(key_event);
            }
            UiMode::Main => {
                self.key_event_main(key_event);
            }
        }
        Ok(())
    }


    fn key_event_search(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.ui_state = UiMode::Main;
            },
            KeyCode::Enter => {
                self.save_search_to_services();
                self.ui_state = UiMode::Main;
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
        let idx = self.search_to_services_idx[self.search.idx];
        self.services.idx = idx;
        self.services.list_state.select(Some(idx));
    }
    
    fn key_event_execute(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.ui_state = UiMode::Main;
            },
            KeyCode::Enter => {
                self.execute_command();
            },
            KeyCode::Char('1') => {
                self.execute_select = Some(ExecuteMode::Auto);
                self.mouse_pos = Some(Button::ButtonAuto);
            },
            KeyCode::Char('2') => {
                self.execute_select = Some(ExecuteMode::AutoDelayed);
                self.mouse_pos = Some(Button::ButtonAutoDelayed);
            },
            KeyCode::Char('3') => {
                self.execute_select = Some(ExecuteMode::Manual);
                self.mouse_pos = Some(Button::ButtonManual);
            },
            KeyCode::Char('4') => {
                self.execute_select = Some(ExecuteMode::Disable);
                self.mouse_pos = Some(Button::ButtonDisable);
            }
            _ => {         
                self.execute_select = None;
                self.mouse_pos = None;
            }
        }
    } 

    fn execute_command(&mut self) {
        if self.services.selected.is_empty() {
            self.cmd_output_disable = String::from("No service selected.");
            self.ui_state = UiMode::Main;
            return;
        }
        match self.execute_select {
            Some(ExecuteMode::Auto) => {
                self.cmd_output_disable = services::enable_auto(&self.services.code_selected).unwrap_or_else(|e| e);
                self.update_statuses(&self.services.code_selected.clone());
            }
            Some(ExecuteMode::AutoDelayed) => {
                self.cmd_output_disable = services::enable_auto_delayed(&self.services.code_selected).unwrap_or_else(|e| e);
                self.update_statuses(&self.services.code_selected.clone());
            }
            Some(ExecuteMode::Manual) => {
                self.cmd_output_disable = services::enable_manual(&self.services.code_selected).unwrap_or_else(|e| e);
                self.update_statuses(&self.services.code_selected.clone());
            }
            Some(ExecuteMode::Disable) => {
                self.cmd_output_stop = services::stop_services(&self.services.code_selected).unwrap_or_else(|e| e);
                self.cmd_output_disable = services::disable_services(&self.services.code_selected).unwrap_or_else(|e| e);
                self.update_statuses(&self.services.code_selected.clone());
            }
            None => {
                self.cmd_output_disable = String::from("No option selected.");
            }
        }
        self.ui_state = UiMode::Main;
    }

    fn update_statuses(&mut self, code_selected: &[String]) {
        for (i, code) in code_selected.iter().enumerate() {
            self.service_info[self.services.idx_selected[i]].start_type = services::update_service_start_type(code);
        }
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
                let selection = &self.service_info[self.services.idx].names.to_string();
                if !check_dupe(&self.services.selected, selection) {
                    self.services.selected.push((self.service_info[self.services.idx].names).to_string());
                    self.services.code_selected.push((self.service_info[self.services.idx].codes).to_string());
                    self.services.idx_selected.push(self.services.idx);
                }
            },
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.services.selected.pop();
                self.services.code_selected.pop();
                self.services.idx_selected.pop();
            },
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.ui_state = UiMode::Search;
                self.search.selected = self.search_services();
            },
            KeyCode::Char('1') => {
                self.mouse_pos = Some(Button::Button1);
                self.select_telemetry();
            },
            KeyCode::Char('2') => {
                self.mouse_pos = Some(Button::Button2);
                self.selection_reset();
            },            
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.ui_state = UiMode::Execute;
            },
            _ => {}
        }            
    }


    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        match self.ui_state {
            UiMode::Search => {
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
            UiMode::Execute => {
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
            UiMode::Main => {
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
        }
        Ok(())
    }

    fn check_click_pos(&mut self, mouse_event: MouseEvent) {
        if is_inside(self.layout.chunks_right[1], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Some(Button::Button1);
            self.select_telemetry();
        }
        else if is_inside(self.layout.chunks_right[2], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Some(Button::Button2);
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
            self.mouse_pos = Some(Button::ButtonAuto);
            self.execute_select = Some(ExecuteMode::Auto);
        }
        else if is_inside(self.layout.chunks_execute[1], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Some(Button::ButtonAutoDelayed);
            self.execute_select = Some(ExecuteMode::AutoDelayed);
        }
        if is_inside(self.layout.chunks_execute[2], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Some(Button::ButtonManual);
            self.execute_select = Some(ExecuteMode::Manual);
        }
        if is_inside(self.layout.chunks_execute[3], mouse_event.column, mouse_event.row) {
            self.mouse_pos = Some(Button::ButtonDisable);
            self.execute_select = Some(ExecuteMode::Disable);
        }
    }


    fn scroll_up(&mut self) {
        if self.services.idx > 0 {
            self.services.idx = self.services.idx.saturating_sub(1);
            self.services.list_state.select_previous();
        }
        else {
            self.services.idx = self.service_info.len() - 1;
            self.services.list_state.select_last();
        }
    }

    fn scroll_down(&mut self) {
        if check_scrollbar_within_bounds(self.service_info.len(), self.services.idx) {
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
        let (idx, names) = self.service_info
            .iter()
            .enumerate()
            .filter(|(_, serv)| serv.names.to_lowercase().contains(&query_lower))
            .map(|(idx, serv)| (idx, serv.names.clone()))
            .unzip();
        self.search_to_services_idx = idx;
        names
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}


fn check_scrollbar_within_bounds(len: usize, idx: usize) -> bool {
    idx < len -1    
}

fn is_inside(rect: Rect, col: u16, row: u16) -> bool {
    rect.contains(Position { x: col, y: row })
}

fn check_dupe(select_list: &[String], selection: &String) -> bool {
    select_list.contains(selection)
}
