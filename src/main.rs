mod handler;
mod render_frame;
mod services;

use std::io::stdout;

use windows_service::{
    service::{Service, ServiceAccess, ServiceStartType, ServiceState}, service_manager::{ServiceManager, ServiceManagerAccess},
};
use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;

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
    chunks_screen_cache: Vec<Rect>,
    chunks_screen_inner_cache: Vec<Rect>,
    chunks_left_cache: Vec<Rect>,

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
const SERVICES:&[&str] = &[
    "Background Intelligent Transfer Service", 
    "Connected User Experiences and Telemetry",
    "Delivery Optimization", 
    "Downloaded Maps Manager",
    "Geolocation Service",
    "Microsoft Store Install Service",
    "Microsoft Usage and Quality Insights",
    "SysMain",
    "Update Orchestrator Service",
    "Windows Insider Service",
    "Windows Health and Optimized Experiences",
    "Windows Modules Installer",
    "Windows Update",
];
const SERVICES_SHORT:&[&str] = &[
    "BITS",
    "DiagTrack",
    "DoSvc",
    "MapsBroker",
    "lfsvc",
    "InstallService",
    "wuqisvc",
    "SysMain",
    "UsoSvc",
    "wisvc",
    "whesvc",
    "TrustedInstaller",
    "wuauserv",
];
const SERVICES_DESC: &[&str] = &[
    /* Background Intelligent Transfer Service */ "Used for background file downloads like Windows updates. *should be bundled together with Windows Update*",
    /* Connected User Experiences and Telemetry */ "Telemetry and diagnostic data collection. (Privacy concern) *SAFE to disable*",
    /* Delivery Optimization */ "Shares Windows updates with other PCs over the internet/LAN. (A waste of internet bandwidth) **CAN'T BE DISABLED DUE TO PERMISSION BUGS IN WIN 11",
    /* Downloaded Maps Manager */ "Safe. unless you use the Windows Maps app offline. (Waste of storage space)",
    /* Geolocation Service */ "Safe. Not recommended if you use location-based apps, weather, or auto time zones. (Privacy concern)",
    /* Microsoft Store Install Service */ "Unsafe. Do not disable if you use the Microsoft Store to install or update apps.",
    /* Microsoft Usage and Quality Insights */ "Safe to disable (diagnostic telemetry and quality tracking). (Privacy concern)",
    /* SysMain */ "Optional; safe to disable on SSDs (sometimes causes high RAM prefetching, previously Superfetch). (Useless on modern SSDs)",
    /* Update Orchestrator Service */ "Unsafe (manages Windows Update restart schedules and workflows). *should be bundled together with Windows Update*",
    /* Windows Health and Optimized Experiences */ "Safe to disable (monitors system health and telemetry reporting). (Privacy concern)",
    /* Windows Modules Installer */ "Unsafe (required to install Windows updates and features, aka TrustedInstaller). (should be bundled together with Windows Update)",
    /* Windows Update */ "Unsafe. if you want your system to receive security patches and updates.",
];

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
            let size = terminal.size()?;
            let screen_area = Rect::new(0, 0, size.width, size.height);
            self.calc_cache_layout(screen_area);                              // Calculate and cache layout
            terminal.draw(|frame| self.render_frame(frame))?;   // rendering
            self.handle_events().wrap_err("handle events failed")?;           // event handling (inputs)
        }
        Ok(())
    }

    fn init_values(&mut self) {
        self.services_list_state.select(Some(0));
        self.services_name_cache = self.get_services_name();
        self.services_name_short_cache = self.get_services_name_short();
        self.services_desc_cache = self.get_services_desc();
    }

}