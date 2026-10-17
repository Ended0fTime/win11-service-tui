/*  WIP
Make so all widgets render within the 3 main's block's inner() area and not direclt on top of it
*/



mod handler;
mod render_frame;
use std::io::stdout;
use std::rc::Rc;
use std::process::Command;
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
    chunks_left_cache: Vec<Rc<[Rect]>>,

    services_list_state: ListState,
    services_count_num: Vec<Line<'static>>,
    services_scrollbar_index: usize,
    services_scrollbar_state: ScrollbarState,

    cmd_output: String,

    mouse_pos: ButtonIdent,
    exit_clicked: bool,

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
    "Windows Health and Optimized Experiences",
    "Windows Modules Installer",
    "Windows Update",
];
const SERVICES_SHORT:&[&str] = &[
    "(BITS)",
    "(DiagTrack)",
    "(DoSvc)",
    "(MapsBroker)",
    "(lfsvc)",
    "(InstallService)",
    "(diagnosticshub.standardcollector.service)",
    "(SysMain)",
    "(UsoSvc)",
    "(WERConSvc)",
    "(TrustedInstaller)",
    "(wuauserv)",
];
const SERVICES_DESC: &[&str] = &[
    /* Background Intelligent Transfer Service */ "Safe to disable (used for background file downloads like Windows updates).",
    /* Connected User Experiences and Telemetry */ "Safe to disable (telemetry and diagnostic data collection).",
    /* Delivery Optimization */ "Safe to disable (shares Windows updates with other PCs over the internet/LAN).",
    /* Downloaded Maps Manager */ "Safe to disable unless you use the Windows Maps app offline.",
    /* Geolocation Service */ "Not recommended if you use location-based apps, weather, or auto time zones.",
    /* Microsoft Store Install Service */ "Do not disable if you use the Microsoft Store to install or update apps.",
    /* Microsoft Usage and Quality Insights */ "Safe to disable (diagnostic telemetry and quality tracking).",
    /* SysMain */ "Optional; safe to disable on SSDs (sometimes causes high RAM prefetching, previously Superfetch).",
    /* Update Orchestrator Service */ "Not recommended (manages Windows Update restart schedules and workflows).",
    /* Windows Health and Optimized Experiences */ "Safe to disable (monitors system health and telemetry reporting).",
    /* Windows Modules Installer */ "Do not disable (required to install Windows updates and features, aka TrustedInstaller).",
    /* Windows Update */ "Do not disable if you want your system to receive security patches and updates.",
];


impl App {
    /// runs the application's main loop until the user quits
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.init();
        while !self.exit {
            let size = terminal.size()?;
            let screen_area = Rect::new(0, 0, size.width, size.height);
            self.calc_cache_layout(screen_area);                              // Calculate and cache layout
            terminal.draw(|frame| self.render_frame(frame))?;   // rendering
            self.handle_events().wrap_err("handle events failed")?;           // event handling (inputs)
        }
        Ok(())
    }

    fn init(&mut self) {
        self.services_list_state.select(Some(0));
    }

    fn disable_services(&mut self) {
        const CREATE_NO_WINDOW: u32 = 0x08000000; // Windows API flag
        let output = Command::new("sc")
            .arg("stop")
            .arg("wuauserv")
            .creation_flags(CREATE_NO_WINDOW)
            .output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.cmd_output = stdout.to_string();
    }
}