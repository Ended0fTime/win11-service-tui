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

    services_list_state: ListState,
    services_count_num: Vec<Line<'static>>,
    services_scrollbar_index: usize,
    services_scrollbar_state: ScrollbarState,

    services_selected: Vec<String>,
    services_code_selected: Vec<String>,

    search_toggle: bool,

    cmd_output_stop: String,
    cmd_output_disable: String,

    mouse_pos: ButtonIdent,

    exit: bool,
}

#[derive(Debug, Default)]
pub struct LayoutCache {
    screen_size:Rect,
    chunks_screen: Vec<Rect>,
    chunks_screen_inner: Vec<Rect>,
    chunks_right: Vec<Rect>,
    chunks_search: Vec<Rect>
}

#[derive(Debug)]
pub enum ButtonHighlight {
    NotPressed(Color),
    Pressed(Color),
}
impl Default for ButtonHighlight {
    fn default() -> Self {
        ButtonHighlight::NotPressed(Color::Red)
    }
}

#[derive(Debug)]
pub enum ButtonIdent {
    Button1(ButtonHighlight),
    Button2(ButtonHighlight),
    Exit(ButtonHighlight),
}
impl Default for ButtonIdent {
    fn default() -> Self {
        ButtonIdent::Exit(ButtonHighlight::default())
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
];

const SERVICES_SAFE_SHORT:&[&str] = &[
    "DiagTrack",
    "MapsBroker",
    "lfsvc",
    "wisvc",
    "wuqisvc",
    "whesvc",
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
        self.services_list_state.select(Some(0));
        self.services_name_cache = services::get_services_name();
        self.services_name_short_cache = services::get_services_name_short();
        self.services_desc_cache = services::get_services_desc();
        self.services_count_num = render_frame::cache_service_numbering(self.services_name_short_cache.len());
    }
}

fn get_term_size(terminal: &mut DefaultTerminal) -> Rect {
    let size = terminal.size().expect("FATAL: get terminal size failed:");
    Rect::new(0, 0, size.width, size.height)
}