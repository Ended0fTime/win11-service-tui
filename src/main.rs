mod handler;
mod render_frame;
mod services;

use std::io::stdout;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};
use ratatui::{
    DefaultTerminal, layout::{Rect}, 
    style::{Color}, text::{Line}, 
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


#[derive(Debug, Default)]
pub struct App {
    layout: LayoutCache,
    services_name_cache: Vec<String>,
    services_name_short_cache: Vec<String>,
    services_desc_cache: Vec<String>,

    services: ScrollbarList,
    search: ScrollbarList,
    
    search_string: Vec<char>,
    search_to_services_idx: Vec<usize>,

    search_toggle: bool,
    execute_toggle: bool,
    execute_select: ExecuteMode,

    cmd_output_stop: String,
    cmd_output_disable: String,

    mouse_pos: Button,

    exit: bool,
}

#[derive(Debug, Default)]
pub struct ScrollbarList {
    list_state: ListState,
    count_num:Vec<Line<'static>>,
    idx: usize,
    scroll_state: ScrollbarState,
    selected: Vec<String>,
    code_selected: Vec<String>,
}

#[derive(Debug, Default)]
pub struct LayoutCache {
    screen_size:Rect,
    chunks_screen: Vec<Rect>,
    chunks_screen_inner: Vec<Rect>,
    chunks_right: Vec<Rect>,
}


#[derive(Debug)]
pub enum Button {
    Button1(Color),
    Button2(Color),
    ButtonAuto(Color),
    ButtonAutoDelayed(Color),
    ButtonManual(Color),
    ButtonDisable(Color),
    Default(),
}
impl Default for Button {
    fn default() -> Self {
        Button::Default()
    }
}

#[derive(Debug)]
pub enum ExecuteMode {
    Auto,
    AutoDelayed,
    Manual,
    Disable,
    Default
}
impl Default for ExecuteMode {
    fn default() -> Self {
        ExecuteMode::Default
    }
}

// Text contents Pre-defining
const SERVICES_SAFE:&[&str] = &[
    "Connected User Experiences and Telemetry",
    "Downloaded Maps Manager",
    "Geolocation Service",
    "Windows Insider Service",
    "Microsoft Usage and Quality Insights",
    "Windows Health and Optimized Experiences",
    "Windows Insider Service"
];

const SERVICES_SAFE_SHORT:&[&str] = &[
    "DiagTrack",
    "MapsBroker",
    "lfsvc",
    "wisvc",
    "wuqisvc",
    "whesvc",
    ""
];


impl App {
    /// runs the application's main loop until the user quits
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.init_values();
        while !self.exit {
            let screen_area = get_term_size(terminal);
            self.cache_layout(screen_area);                              // Calculate and cache layout
            terminal.draw(|frame| self.render_frame(frame))?;   // rendering
            self.handle_events().wrap_err("handle events failed")?;           // event handling (inputs)
        }
        Ok(())
    }

    fn init_values(&mut self) {
        self.services.list_state.select(Some(0));
        self.services_name_cache = services::get_services_name();
        self.services_name_short_cache = services::get_services_name_short();
        self.services_desc_cache = services::get_services_desc();
        self.services.count_num = render_frame::cache_service_numbering(self.services_name_short_cache.len());
    }
}

fn get_term_size(terminal: &mut DefaultTerminal) -> Rect {
    let size = terminal.size().expect("FATAL: get terminal size failed:");
    Rect::new(0, 0, size.width, size.height)
}