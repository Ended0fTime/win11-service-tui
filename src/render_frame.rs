use crate::{App, ButtonHighlight, ButtonIdent};
use ratatui::{
    Frame, buffer::Buffer, layout::{Alignment, Constraint, Spacing, Direction::{self, Horizontal}, Layout, Rect}, 
    style::{Color, Style, Stylize}, symbols::{border, scrollbar, merge::MergeStrategy}, text::{Line, Text, Span}, 
    widgets::{Block, Borders, Padding, BorderType::{self, Rounded, Thick}, List, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState, ListState},
};


// Colour declaration
pub const LIGHT_YELLOW: Color = Color::Rgb(255, 255, 24);
pub const LIGHT_BLUE: Color = Color::Rgb(25, 150, 255);
pub const DARK_BLUE: Color = Color::Rgb(3, 12, 43);

// Const Values
const SCREEN_WIDTH_MIN:u16 = 20;
const SCREEN_HEIGHT_MIN:u16 = 15;

impl App {    
    pub fn render_frame(&mut self, frame: &mut Frame) {
        let screen_width_check:bool = self.chunks_screen_inner_cache[0].width < SCREEN_WIDTH_MIN;
        let screen_height_check:bool = self.chunks_screen_inner_cache[0].height < SCREEN_HEIGHT_MIN;
        if screen_width_check || screen_height_check {
            self.render_too_small(frame);
        }
        else {
            self.render_main(frame);
        }
    }


    pub fn calc_cache_layout(&mut self, area: Rect) {
        // Main Area Layout
        let chunks_screen_horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Ratio(1, 3),
        ]).spacing(Spacing::Overlap(1)).split(area);
        let chunks_screen_side_vertical = Layout::vertical([
            Constraint::Ratio(1, 2),
            Constraint::Fill(1),
        ]).spacing(Spacing::Overlap(1)).split(chunks_screen_horizontal[0]);

        //Right Layout
        let chunks_screen_side_vertical_upper = Layout::horizontal([
            Constraint::Ratio(1, 20),
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(2),
        ]).split(chunks_screen_side_vertical[0]);
        
        let chunks_screen_side_vertical_lower = Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]).split(chunks_screen_side_vertical[1]);
        
        self.chunks_screen_cache = vec![
            chunks_screen_horizontal[1], 
            chunks_screen_side_vertical[0], 
            chunks_screen_side_vertical[1]
        ];
        self.chunks_screen_inner_cache = vec![
            Block::bordered().inner(chunks_screen_horizontal[1]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_screen_side_vertical_upper[1]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM).inner(chunks_screen_side_vertical_upper[2]), 
            Block::bordered().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT).inner(chunks_screen_side_vertical_upper[3]), 
            Block::bordered().inner(chunks_screen_side_vertical_lower[0]), 
            Block::bordered().inner(chunks_screen_side_vertical_lower[1]),
            Block::bordered().inner(chunks_screen_side_vertical_lower[2]),
        ];

        // Left Layout
        let chunks_left = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Fill(1),
        ]).split(self.chunks_screen_inner_cache[0]);
        // Further subdivision to horizontally narrow down elements
        let mut chunks_button = Vec::new();
        for i in 1..=2 {
            chunks_button.push(Layout::horizontal([
                Constraint::Ratio(1,5),
                Constraint::Fill(1),
                Constraint::Ratio(1,5),
            ]).split(chunks_left[i]));
        }
        self.chunks_left_cache = vec![
            chunks_left[0], 
            chunks_button[0][1], 
            chunks_button[1][1], 
            chunks_left[3], 
            chunks_left[4]
        ];
        
        //List services count caching
        self.services_count_num.clear();
        //TEMP TEST
        let raw_output_serv_title: Vec<String> = self.services_name_cache.clone();
        for i in 1..=raw_output_serv_title.len() {
            let line = vec![Line::from(vec![i.to_string().into(), ". ".into()]).alignment(Alignment::Right)];
            self.services_count_num.extend(line);
        }
    }


    fn render_too_small(&mut self, frame: &mut Frame) {
        let screen_area = frame.area();
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
        todo!();
        let screen_area = frame.area();
        let main_layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).split(screen_area);

        let searchbar = Paragraph::new("terminal is too small. Please enlarge")
            .fg(LIGHT_YELLOW)
            .bold()
            .centered();
        frame.render_widget(searchbar, main_layout[1]);
    }

    fn render_main(&mut self, frame: &mut Frame) {
        // Outer frames
        let title = Line::from(" Win11 Service Disabler ").fg(LIGHT_YELLOW).italic();
        let keybind_guide = Line::from("<Q>Exit <E>Select <R>Remove <Enter>Execute").white().italic().centered();
        let block_right_top = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .title(title)
            .merge_borders(MergeStrategy::Exact);
        let block_right_btm = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .merge_borders(MergeStrategy::Exact)
            .title_bottom(keybind_guide);
        let block_left = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .merge_borders(MergeStrategy::Exact);

        // Content declaration
        let raw_output_serv_title: Vec<String> = self.services_name_cache.clone();
        let raw_output_serv_short: Vec<String> = self.services_name_short_cache.clone();
        let raw_output_serv_desc: Vec<String> = self.services_desc_cache.clone();

        let list_services = List::new(raw_output_serv_title.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());
        let list_services_count = List::new(self.services_count_num.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}));
        self.services_scrollbar_state = self.services_scrollbar_state
            .position(self.services_scrollbar_index)
            .content_length(self.services_count_num.len());
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let serv_title = raw_output_serv_title.get(self.services_scrollbar_index).unwrap();
        let serv_title_short = raw_output_serv_short.get(self.services_scrollbar_index).unwrap();
        let default_error = String::from("Default");
        let serv_desc = raw_output_serv_desc.get(self.services_scrollbar_index).unwrap_or(&default_error);
        // let serv_desc = SERVICES_DESC.get(self.services_scrollbar_index).unwrap_or(&"Default");

        let para_details_title = Paragraph::new(vec![Line::from(
            Span::styled(serv_title, Style::default().bold())), 
            Line::from(Span::styled(serv_title_short, Style::default().bold()))])
            .block(Block::new().padding(Padding::new(3, 3, 1, 0))); 
        let para_details_desc = Paragraph::new(Line::from(vec![
            Span::styled(serv_desc, Style::default().green())])) 
            .block(Block::new().padding(Padding::new(3, 3, 0, 0)))
            .wrap(Wrap {trim: true}); 

        let cmd_out = Paragraph::new(vec![
            Line::from(self.cmd_output_stop.clone().white().italic()), 
            Line::from(self.cmd_output_disable.clone().white().italic()), 
            ])
            .block(Block::new().padding(Padding::new(3, 3, 0, 0)))
            .wrap(Wrap {trim: true}); 

        let content_header_left = Paragraph::new(
            Line::from("Select batch of services to disable.")
                .fg(LIGHT_YELLOW).bold().italic())
            .wrap(Wrap {trim: true})
            .centered()
            .block(Block::new().padding(Padding::new(1, 1, 4, 0)));
        let mut button1 = App::create_button("Safe services".to_string(), Color::White);
        let mut button2 = App::create_button("Clear".to_string(), Color::White);

        // Color management based on events
        match self.mouse_pos {
            ButtonIdent::Button1(ButtonHighlight::Pressed(content_color)) => {
                button1 = App::create_button("Safe".to_string(), content_color).bold();
            }
            ButtonIdent::Button2(ButtonHighlight::Pressed(content_color)) => {
                button2 = App::create_button("Unsafe".to_string(), content_color).bold();
            }
            _ => {}
        }

        let list_selected_title = Paragraph::new("Services Selected: ")
            .block(Block::new()
            .padding(Padding{left: 2, right: 0, top: 2, bottom: 0}));
        let list_selected= List::new(self.services_selected.clone())
            .block(Block::new()
            .padding(Padding{left: 2, right: 0, top: 0, bottom: 0}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());

        // Rendering
        frame.render_widget(block_left,self.chunks_screen_cache[0]);
        frame.render_widget(block_right_top,self.chunks_screen_cache[1]);
        frame.render_widget(block_right_btm,self.chunks_screen_cache[2]);

        frame.render_widget(content_header_left,self.chunks_left_cache[0]);
        frame.render_widget(button1,self.chunks_left_cache[1]);
        frame.render_widget(button2,self.chunks_left_cache[2]);

        frame.render_stateful_widget(list_services_count,self.chunks_screen_inner_cache[1], &mut self.services_list_state);
        frame.render_stateful_widget(list_services,self.chunks_screen_inner_cache[2], &mut self.services_list_state);
        frame.render_stateful_widget(scrollbar, self.chunks_screen_inner_cache[3], &mut self.services_scrollbar_state);

        frame.render_widget(list_selected_title,self.chunks_left_cache[3]);
        frame.render_widget(list_selected,self.chunks_left_cache[4]);

        frame.render_widget(para_details_title,self.chunks_screen_inner_cache[4]);
        frame.render_widget(para_details_desc,self.chunks_screen_inner_cache[5]);
        frame.render_widget(cmd_out, self.chunks_screen_inner_cache[6]);
    }
       

    fn create_button(content: String, content_color:Color) -> Paragraph<'static>{
        Paragraph::new(
            Line::from(vec![content.fg(content_color).italic()])
            .centered()).block(Block::bordered().border_style(Style::default().fg(LIGHT_BLUE))
        )
    }
}