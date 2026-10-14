/*  WIP
Make so all widgets render within the 3 main's block's inner() area and not direclt on top of it
*/



mod windows_interface;
mod handler;
use std::io::stdout;
use std::rc::Rc;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, style::{Attribute::Reset, SetColors}};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Alignment, Constraint, Direction::{self, Horizontal}, Layout, Rect}, style::{Color, Style, Stylize, palette::material::YELLOW}, symbols::border, text::{Line, Text}, widgets::{Block, BorderType::{self, Rounded, Thick}, List, Paragraph, Wrap, Widget},
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
    chunks_left_cache: Vec<Rc<[Rect]>>,
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


impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            let size = terminal.size()?;
            let screen_area = Rect::new(0, 0, size.width, size.height);
            self.calc_cache_layout(screen_area);                                                 // Calculate and cache layout
            terminal.draw(|frame| self.render_frame(frame))?;   // rendering
            self.handle_events().wrap_err("handle events failed")?;                             // event handling (inputs)
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

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
        let chunks_screen_side_vertical_lower = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).split(chunks_screen_side_vertical[1]);
        self.chunks_screen_cache = vec![chunks_screen_horizontal[0]
            , chunks_screen_side_vertical[0]
            , chunks_screen_side_vertical[1]
            , chunks_screen_side_vertical_lower[0]
            , chunks_screen_side_vertical_lower[1]];
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
    }

    fn create_button(content: String, content_color:Color) -> Paragraph<'static>{
        Paragraph::new(Line::from(vec![ 
                content.fg(content_color).italic()])
            .centered()).block(Block::bordered().border_style(Style::default().fg(LIGHT_BLUE)))
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Outer frame
        let title = Line::from(" Ferlex TUI ").fg(LIGHT_YELLOW).italic();
        let block_left = Block::bordered().border_set(border::ROUNDED)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .title(title);
        let block_right_top = Block::bordered().border_set(border::ROUNDED)
            .border_style(Style::default().fg(LIGHT_BLUE));
        let block_right_btm = Block::bordered().border_set(border::ROUNDED)
            .border_style(Style::default().fg(LIGHT_BLUE));

        // Content declaration
        let para_right = Paragraph::new(Line::from("Select badge of services to disable.")
                .fg(Color::LightGreen).bold().italic().centered())
            .wrap(Wrap {trim: true});
        let mut content1 = App::create_button("Safe".to_string(), Color::White);
        let mut content2 = App::create_button("Unsafe".to_string(), Color::White);
        let mut content3 = App::create_button("Exit".to_string(), LIGHT_YELLOW);

        let list_services = List::new(["1", "2"]);

        // Color management based on events
        match self.mouse_pos {
            ButtonIdent::Button1(ButtonHighlight::Pressed(content_color)) => {
                content1 = App::create_button("Safe".to_string(), content_color).bold();
            }
            ButtonIdent::Button2(ButtonHighlight::Pressed(content_color)) => {
                content2 = App::create_button("Unsafe".to_string(), content_color).bold();
            }
            ButtonIdent::Exit(ButtonHighlight::Pressed(content_color)) => {
                content3 = App::create_button("Exit".to_string(), content_color).bold();
            }
            _ => {}
        }

        // Rendering
        block_left.render(self.chunks_screen_cache[0], buf);
        block_right_top.render(self.chunks_screen_cache[1], buf);
        block_right_btm.render(self.chunks_screen_cache[2], buf);

        para_right.render(self.chunks_left_cache[0][1], buf);
        content1.render(self.chunks_left_cache[2][1], buf);
        content2.render(self.chunks_left_cache[5][1], buf);
        content3.render(self.chunks_left_cache[8][1], buf);

        list_services.render(self.chunks_screen_cache[1], buf);
    }
}
