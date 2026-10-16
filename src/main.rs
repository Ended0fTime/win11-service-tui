/*  WIP
Make so all widgets render within the 3 main's block's inner() area and not direclt on top of it
*/



mod windows_interface;
mod handler;
mod render_frame;
use std::io::stdout;
use std::rc::Rc;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, style::{Attribute::Reset, SetColors}};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Alignment, Constraint, Direction::{self, Horizontal}, Layout, Rect}, 
    style::{Color, Style, Stylize, palette::material::YELLOW}, symbols::{border, scrollbar}, text::{Line, Text, Span}, 
    widgets::{Block, Borders, Padding, BorderType::{self, Rounded, Thick}, List, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState, ListState},
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
    services_count_vec: Vec<Line<'static>>,
    services_scrollbar_index: usize,
    services_scrollbar_state: ScrollbarState,

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

// Colour declaration
pub const LIGHT_YELLOW: Color = Color::Rgb(255, 255, 24);
pub const LIGHT_BLUE: Color = Color::Rgb(25, 150, 255);
pub const DARK_BLUE: Color = Color::Rgb(3, 12, 43);

// Text contents Pre-defining
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
    /* Windows Update */ "Do not disable if you want your system to receive security patches and updates.",];

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


impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
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

    fn exit(&mut self) {
        self.exit = true;
    }

    // <#================== RENDER CALCULATIONS ==================#> //
    fn calc_cache_layout(&mut self, area: Rect) {
        // Main Area Layout
        let chunks_screen_horizontal = Layout::horizontal([
            Constraint::Ratio(1, 4),
            Constraint::Fill(1),
        ]).split(area);
        let chunks_screen_side_vertical = Layout::vertical([
            Constraint::Ratio(1, 3),
            Constraint::Fill(1),
        ]).split(chunks_screen_horizontal[1]);
        let chunks_screen_side_vertical_upper = Layout::horizontal([
            Constraint::Ratio(1, 20),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(2),
        ]).split(chunks_screen_side_vertical[0]);
        let chunks_screen_side_vertical_lower = Layout::vertical([
            Constraint::Length(6),
            Constraint::Fill(1),
        ]).split(chunks_screen_side_vertical[1]);
        self.chunks_screen_cache = vec![
            chunks_screen_horizontal[0], 
            chunks_screen_side_vertical[0], 
            chunks_screen_side_vertical[1]
        ];
        self.chunks_screen_inner_cache = vec![
            Block::bordered().inner(chunks_screen_horizontal[0]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_screen_side_vertical_upper[1]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_screen_side_vertical_upper[2]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT).inner(chunks_screen_side_vertical_upper[3]), 
            Block::bordered().inner(chunks_screen_side_vertical_lower[0]), 
            Block::bordered().inner(chunks_screen_side_vertical_lower[1])
        ];

        // Left Layout
        let chunks_left = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]).split(chunks_screen_horizontal[0]);
        // Left Layout Subdivision (To vertically center elements)
        self.chunks_left_cache.clear();
        let mut chunks_content:Vec<Rc<[Rect]>> = Vec::new();
        for i in 0..chunks_left.len() {
            chunks_content.push(Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
                ]).split(chunks_left[i]));
        };
        self.chunks_left_cache.push(chunks_content[0].clone());
        // Further subdivision to horizontally narrow down elements
        for i in 1..chunks_left.len() {
            for j in 0..3 {
            self.chunks_left_cache.push(Layout::horizontal([
                Constraint::Ratio(1,5),
                Constraint::Fill(1),
                Constraint::Ratio(1,5),
                ]).split(chunks_content[i][j]));
            }
        }
        
        self.services_count_vec.clear();
        for i in 1..=SERVICES.len() {
            let line = vec![Line::from(vec![i.to_string().into(), ". ".into()]).alignment(Alignment::Right)];
            self.services_count_vec.extend(line);
        }
    }

    fn create_button(content: String, content_color:Color) -> Paragraph<'static>{
        Paragraph::new(
            Line::from(vec![content.fg(content_color).italic()])
            .centered()).block(Block::bordered().border_style(Style::default().fg(LIGHT_BLUE))
        )
    }
}