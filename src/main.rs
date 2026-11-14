mod handler;
mod render_frame;
mod services;

use services::ServiceInfo;

use std::io::stdout;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};
use ratatui::{
    DefaultTerminal, 
    layout::Rect, 
    text::Line, 
    widgets::{ScrollbarState, ListState},
};
use color_eyre::{
    Result, eyre::WrapErr,
};


fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;
    app_result
}


#[derive(Default)]
pub struct App {
    layout: LayoutCache,
    service_info: Vec<ServiceInfo>,
    services: ScrollbarList,
    search: ScrollbarList,
    search_string: Vec<char>,
    search_to_services_idx: Vec<usize>,
    ui_state: UiMode,
    mouse_pos: Option<Button>,
    execute_select: Option<ExecuteMode>,
    cmd_output_stop: String,
    cmd_output_disable: String,
    exit: bool,
}

#[derive(Default)]
pub struct ScrollbarList {
    list_state: ListState,
    count_num:Vec<Line<'static>>,
    idx: usize,
    scroll_state: ScrollbarState,
    selected: Vec<String>,
    code_selected: Vec<String>,
    idx_selected: Vec<usize>
}

#[derive(Default)]
pub struct LayoutCache {
    screen_size:Rect,
    chunks_screen: Vec<Rect>,
    chunks_screen_inner: Vec<Rect>,
    chunks_right: Vec<Rect>,
    chunks_execute: Vec<Rect>,
}

#[derive(Default)]
pub enum UiMode {
    #[default]
    Main,
    Search,
    Execute,
}

pub enum Button {
    Button1,
    Button2,
    ButtonAuto,
    ButtonAutoDelayed,
    ButtonManual,
    ButtonDisable,
}

pub enum ExecuteMode {
    Auto,
    AutoDelayed,
    Manual,
    Disable,
}


impl App {
    /// runs the application's main loop until the user quits
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.new();
        while !self.exit {
            let screen_area = get_term_size(terminal);
            self.cache_layout(screen_area);
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn new(&mut self) {
        self.services.list_state.select(Some(0));
        self.service_info = services::get_service_metadata();
        self.services.count_num = render_frame::cache_list_numbering(self.service_info.len());
    }
}

fn get_term_size(terminal: &mut DefaultTerminal) -> Rect {
    let size = terminal.size().expect("FATAL: get terminal size failed:");
    Rect::new(0, 0, size.width, size.height)
}