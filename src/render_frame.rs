use crate::{App, Button};
use ratatui::{
    Frame, 
    layout::{Alignment, Constraint, Spacing, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    symbols::{border, merge::MergeStrategy}, text::{Line, Span}, 
    widgets::{Block, Borders, Padding, 
        List, Paragraph, Wrap, Scrollbar, ScrollbarOrientation},
};


// Colour declaration
pub const LIGHT_YELLOW: Color = Color::Rgb(255, 255, 24);
pub const LIGHT_BLUE: Color = Color::Rgb(25, 150, 255);
pub const DARK_BLUE: Color = Color::Rgb(3, 12, 43);

// Const Values
const SCREEN_WIDTH_MIN:u16 = 20;
const SCREEN_HEIGHT_MIN:u16 = 15;

impl App {    
    pub fn cache_layout(&mut self, area: Rect) {        
        let chunks_main = calc_layout_main_horizontal(area);
        let chunks_left = calc_layout_main_vertical(chunks_main[0]);
        let chunks_left_upper = calc_layout_left_upper(chunks_left[0]);
        let chunks_left_lower = calc_layout_left_lower(chunks_left[1]);
        let chunks_right = calc_layout_right(chunks_main[1]);

        self.layout.screen_size = area;     // Entire screen area size
        self.layout.chunks_screen = vec![
            chunks_main[1], 
            chunks_left[0], 
            chunks_left[1]
        ];
        self.layout.chunks_screen_inner = vec![
            Block::bordered().inner(chunks_main[1]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_left_upper[1]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_left_upper[2]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT).inner(chunks_left_upper[3]), 
            Block::bordered().inner(chunks_left_lower[0]), 
            Block::bordered().inner(chunks_left_lower[1]),
            Block::bordered().inner(chunks_left_lower[2]),
        ];
        self.layout.chunks_right = chunks_right;
    }


    pub fn render_frame(&mut self, frame: &mut Frame) {
        let screen_width_check:bool = self.layout.chunks_screen_inner[0].width < SCREEN_WIDTH_MIN;
        let screen_height_check:bool = self.layout.chunks_screen_inner[0].height < SCREEN_HEIGHT_MIN;
        if screen_width_check || screen_height_check {
            self.render_too_small(frame);
        }
        else if self.search_toggle {
                self.render_search(frame);
            }
        else {
            self.render_main(frame);
            if self.execute_toggle {
                self.render_execute_select(frame);
            }
        }
    }

    fn render_too_small(&mut self, frame: &mut Frame) {
        let screen_area = self.layout.screen_size;
        let main_layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).split(screen_area);

        let black_block = Block::new().bg(Color::Black);
        let too_small_para = Paragraph::new("terminal is too small. Please enlarge")
            .fg(LIGHT_YELLOW)
            .bold()
            .centered();
        frame.render_widget(black_block, screen_area);
        frame.render_widget(too_small_para, main_layout[1]);
    }

    fn render_search(&mut self, frame: &mut Frame) {
        let screen_area = self.layout.screen_size;
        let usable_layout = calc_layout_search(screen_area);

        let search_list = List::new(self.search.selected.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());

        self.search.scroll_state = self.search.scroll_state
            .position(self.search.idx)
            .content_length(self.search.count_num.len());

        let black_block = Block::new().bg(Color::Black);
        let searchbar_content: String = self.search_string.iter().collect();
        let too_small_para = Paragraph::new(searchbar_content) 
            .fg(Color::default()).bold().centered()
            .block(Block::bordered().border_style(Style::default().fg(LIGHT_YELLOW)));
        frame.render_widget(black_block, screen_area);
        frame.render_widget(too_small_para, usable_layout[0]);
        frame.render_stateful_widget(search_list, usable_layout[1], &mut self.search.list_state);

    }

    fn render_execute_select(&mut self, frame: &mut Frame) {
        let screen_area = self.layout.screen_size;
        let usable_layout = calc_layout_execute(screen_area);

        let mut button_auto = create_button(String::from("Auto"), LIGHT_YELLOW);
        let mut button_auto_delayed = create_button(String::from("Auto (Delayed)"), LIGHT_YELLOW);
        let mut button_manual = create_button(String::from("Manually Triggered"), LIGHT_YELLOW);
        let mut button_disable = create_button(String::from("Disable"), LIGHT_YELLOW);

        // Color management based on events
        match self.mouse_pos {
            Button::ButtonAuto(content_color) => {
                button_auto = create_button("Auto".to_string(), content_color).bold();
            }
            Button::ButtonAutoDelayed(content_color) => {
                button_auto_delayed = create_button("Auto (Delayed)".to_string(), content_color).bold();
            }
            Button::ButtonManual(content_color) => {
                button_manual = create_button("Manually Triggered".to_string(), content_color).bold();
            }
            Button::ButtonDisable(content_color) => {
                button_disable = create_button("Disable".to_string(), content_color).bold();
            }
            _ => {}
        }

        frame.render_widget(button_auto, usable_layout[0]);
        frame.render_widget(button_auto_delayed, usable_layout[1]);
        frame.render_widget(button_manual, usable_layout[2]);
        frame.render_widget(button_disable, usable_layout[3]);
    }

    fn render_main(&mut self, frame: &mut Frame) {
        // Outer frames
        let title = Line::from(" Win11 Service Disabler ").fg(LIGHT_YELLOW).italic();
        let keybind_guide = Line::from("<Q>Exit <E/Enter>Select <R>Remove <F>Find <Shift+Enter>Execute").white().italic().centered();
        let block_left_upper = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .title(title)
            .merge_borders(MergeStrategy::Exact);
        let block_left_lower = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .merge_borders(MergeStrategy::Exact)
            .title_bottom(keybind_guide);
        let block_right = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .merge_borders(MergeStrategy::Exact);

        // Content declaration
        let output_serv_title: &Vec<String> = &self.services_name_cache;
        let output_serv_short: &Vec<String> = &self.services_name_short_cache;
        let output_serv_desc: &Vec<String> = &self.services_desc_cache;

        let list_services = List::new(output_serv_title.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());
        let list_services_count = List::new(self.services.count_num.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}));
        self.services.scroll_state = self.services.scroll_state
            .position(self.services.idx)
            .content_length(self.services.count_num.len());
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let defaulterr = String::from("No description given.");
        let serv_title_short = output_serv_short.get(self.services.idx).unwrap_or(&defaulterr).clone();
        let serv_desc = output_serv_desc.get(self.services.idx).unwrap_or(&defaulterr);

        let para_details_title = Paragraph::new(
            Line::from(serv_title_short).bold())
            .block(Block::new().padding(Padding::new(3, 3, 1, 0)));        
        let para_details_desc = Paragraph::new(
            Span::styled(serv_desc, Style::default().green()))
            .block(Block::new().padding(Padding::new(3, 3, 0, 0)))
            .wrap(Wrap {trim: true}); 

        let cmd_out = Paragraph::new(vec![
            Line::from(self.cmd_output_stop.clone().white().italic()), 
            Line::from(self.cmd_output_disable.clone().white().italic()), 
            ])
            .block(Block::new().padding(Padding::new(3, 3, 0, 0)))
            .wrap(Wrap {trim: true}); 

        let content_header_right = Paragraph::new(
            Line::from("Select batch of services to disable.")
                .fg(LIGHT_YELLOW).bold().italic())
            .wrap(Wrap {trim: true})
            .centered()
            .block(Block::new().padding(Padding::new(1, 1, 4, 0)));
        let mut button1 = create_button("Telemetry".to_string(), Color::White);
        let mut button2 = create_button("Clear selection".to_string(), Color::White);

        // Color management based on events
        match self.mouse_pos {
            Button::Button1(content_color) => {
                button1 = create_button("Telemetry".to_string(), content_color).bold();
            }
            Button::Button2(content_color) => {
                button2 = create_button("Clear selection".to_string(), content_color).bold();
            }
            _ => {}
        }

        let list_selected_title = Paragraph::new("Services Selected: ")
            .block(Block::new()
            .padding(Padding{left: 2, right: 0, top: 2, bottom: 0}));
        let list_selected= List::new(self.services.selected.clone())
            .block(Block::new()
            .padding(Padding{left: 2, right: 0, top: 0, bottom: 0}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());

        // Rendering
        frame.render_widget(block_right,self.layout.chunks_screen[0]);
        frame.render_widget(block_left_upper,self.layout.chunks_screen[1]);
        frame.render_widget(block_left_lower,self.layout.chunks_screen[2]);

        frame.render_widget(content_header_right,self.layout.chunks_right[0]);
        frame.render_widget(button1,self.layout.chunks_right[1]);
        frame.render_widget(button2,self.layout.chunks_right[2]);

        frame.render_stateful_widget(list_services_count,self.layout.chunks_screen_inner[1], &mut self.services.list_state);
        frame.render_stateful_widget(list_services,self.layout.chunks_screen_inner[2], &mut self.services.list_state);
        frame.render_stateful_widget(scrollbar, self.layout.chunks_screen_inner[3], &mut self.services.scroll_state);

        frame.render_widget(list_selected_title,self.layout.chunks_right[3]);
        frame.render_widget(list_selected,self.layout.chunks_right[4]);

        frame.render_widget(para_details_title,self.layout.chunks_screen_inner[4]);
        frame.render_widget(para_details_desc,self.layout.chunks_screen_inner[5]);
        frame.render_widget(cmd_out, self.layout.chunks_screen_inner[6]);
    }
}


fn create_button(content: String, content_color:Color) -> Paragraph<'static>{
    Paragraph::new(
        Line::from(vec![content.fg(content_color).italic()])
        .centered()).block(Block::bordered().border_style(Style::default().fg(LIGHT_BLUE))
    )
}

pub fn cache_service_numbering(vec_len: usize)-> Vec<Line<'static>> {
    let mut serv_count_num: Vec<Line> = Vec::with_capacity(vec_len);
    for i in 1..=vec_len {
        let line = Line::from(vec![i.to_string().into(), ". ".into()]).alignment(Alignment::Right);
        serv_count_num.push(line);
    }
    return serv_count_num;
}

fn calc_layout_main_horizontal(area: Rect) -> Vec<Rect> {
    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Ratio(1, 3),
    ]).spacing(Spacing::Overlap(1)).split(area).to_vec()
}

fn calc_layout_main_vertical(chunk: Rect) -> Vec<Rect> {
    Layout::vertical([
        Constraint::Ratio(1, 2),
        Constraint::Fill(1),
    ]).spacing(Spacing::Overlap(1)).split(chunk).to_vec()
}

fn calc_layout_left_upper(chunk: Rect) -> Vec<Rect> {
    Layout::horizontal([
        Constraint::Ratio(1, 20),   //margin
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(2),      //margin
    ]).split(chunk).to_vec()
}

fn calc_layout_left_lower(chunk: Rect) -> Vec<Rect> {
    Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ]).split(chunk).to_vec()
}

fn calc_layout_right(chunk: Rect) -> Vec<Rect> {
    let chunks_right = Layout::vertical([
        Constraint::Length(8),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Fill(1),
    ]).split(chunk);
    // Further subdivision to horizontally narrow down elements
    let mut chunks_button = Vec::with_capacity(3);
    for i in 1..=2 {
        chunks_button.push(Layout::horizontal([
            Constraint::Ratio(1,5),
            Constraint::Fill(1),
            Constraint::Ratio(1,5),
        ]).split(chunks_right[i]));
    }
    return vec![
        chunks_right[0], 
        chunks_button[0][1], 
        chunks_button[1][1], 
        chunks_right[3], 
        chunks_right[4]
    ];
}

fn calc_layout_search(chunk: Rect) -> Vec<Rect> {
    let vertical_layout = Layout::vertical([
        Constraint::Ratio(1, 20),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Ratio(1, 20),
    ]).split(chunk);
    let mut horizontal_layout = Vec::with_capacity(2);
    for i in 1..=2 {
        horizontal_layout.push(Layout::horizontal([
            Constraint::Ratio(1, 8),
            Constraint::Fill(1),
            Constraint::Ratio(1, 8),
        ]).split(vertical_layout[i]).to_vec());
    }
    return vec![horizontal_layout[0][1], horizontal_layout[1][1]];
}

fn calc_layout_execute(chunk: Rect) -> Vec<Rect> {
    let vertical_layout = Layout::vertical([
        Constraint::Ratio(3, 4),
        Constraint::Length(3),
        Constraint::Fill(1),
    ]).split(chunk);
    let horizontal_layout = Layout::horizontal([
        Constraint::Ratio(1, 20),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Ratio(1, 20),
        Constraint::Ratio(1, 3),
    ]).split(vertical_layout[1]).to_vec();
    return vec![horizontal_layout[1], horizontal_layout[2], horizontal_layout[3], horizontal_layout[4]];
}