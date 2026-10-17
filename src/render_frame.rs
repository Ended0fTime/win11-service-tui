use crate::{App, ButtonHighlight, ButtonIdent, SERVICES, SERVICES_DESC, SERVICES_SHORT};
use ratatui::{
    Frame, buffer::Buffer, layout::{Alignment, Constraint, Direction::{self, Horizontal}, Layout, Rect}, 
    style::{Color, Style, Stylize}, symbols::{border, scrollbar}, text::{Line, Text, Span}, 
    widgets::{Block, Borders, Padding, BorderType::{self, Rounded, Thick}, List, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState, ListState},
};
use std::rc::Rc;


// Colour declaration
pub const LIGHT_YELLOW: Color = Color::Rgb(255, 255, 24);
pub const LIGHT_BLUE: Color = Color::Rgb(25, 150, 255);
pub const DARK_BLUE: Color = Color::Rgb(3, 12, 43);


impl App {
    pub fn check_admin(&self) {
        todo!();
    }
    
    pub fn check_term_size(&self) {
        todo!();
    }

    // <#================== RENDER CALCULATIONS ==================#> //
    pub fn calc_cache_layout(&mut self, area: Rect) {
        // Main Area Layout
        let chunks_screen_horizontal = Layout::horizontal([
            Constraint::Ratio(1, 4),
            Constraint::Fill(1),
        ]).split(area);
        let chunks_screen_side_vertical = Layout::vertical([
            Constraint::Ratio(1, 3),
            Constraint::Fill(1),
        ]).split(chunks_screen_horizontal[1]);

        //Right Layout
        let chunks_screen_side_vertical_upper = Layout::horizontal([
            Constraint::Ratio(1, 20),
            Constraint::Length(4),
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
            Block::bordered().inner(chunks_screen_side_vertical_lower[1]),
            Block::bordered().inner(chunks_screen_side_vertical_lower[2]),
        ];

        // Left Layout
        let chunks_left = Layout::vertical([
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(4),
        ]).split(self.chunks_screen_inner_cache[0]);
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
        
        //List services count caching
        self.services_count_num.clear();
        for i in 1..=SERVICES.len() {
            let line = vec![Line::from(vec![i.to_string().into(), ". ".into()]).alignment(Alignment::Right)];
            self.services_count_num.extend(line);
        }
    }


    pub fn render_frame(&mut self, frame: &mut Frame) {
        // Outer frames
        let title = Line::from(" Ferlex TUI ").fg(LIGHT_YELLOW).italic();
        let block_left = Block::bordered().border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .title(title);
        let block_right_top = Block::bordered().border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE));
        let block_right_btm = Block::bordered().border_set(border::PLAIN)
            .border_style(Style::default().fg(LIGHT_BLUE));

        // Content declaration
        let content_header_left = Paragraph::new(Line::from("Select batch of services to disable.")
                .fg(LIGHT_YELLOW).bold().italic().centered())
            .wrap(Wrap {trim: true});
        let mut button1 = App::create_button("Safe".to_string(), Color::White);
        let mut button2 = App::create_button("Unsafe".to_string(), Color::White);

        let list_services = List::new(SERVICES.to_vec().clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());
        let list_services_count = List::new(self.services_count_num.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 1}));
        self.services_scrollbar_state = self.services_scrollbar_state
            .position(self.services_scrollbar_index)
            .content_length(SERVICES.len());
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let serv_title = SERVICES.get(self.services_scrollbar_index).unwrap_or(&"Default");
        let serv_title_short = SERVICES_SHORT.get(self.services_scrollbar_index).unwrap_or(&"Default");
        let serv_desc = SERVICES_DESC.get(self.services_scrollbar_index).unwrap_or(&"Default");

        let para_details_title = Paragraph::new(vec![Line::from(
            Span::styled(*serv_title, Style::default().bold())), 
            Line::from(Span::styled(*serv_title_short, Style::default().bold()))])
            .block(Block::new().padding(Padding::new(3, 0, 1, 0))); 
        let para_details_desc = Paragraph::new(Line::from(vec![
            Span::styled(*serv_desc, Style::default().green())])) 
            .block(Block::new().padding(Padding::new(3, 0, 0, 0)))
            .wrap(Wrap {trim: true}); 

        let cmd_out = Paragraph::new(Line::from(vec![
            Span::styled(self.cmd_output.clone(), Style::default().white().italic())])) 
            .block(Block::new().padding(Padding::new(3, 0, 0, 0)))
            .wrap(Wrap {trim: true}); 

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

        // Rendering
        frame.render_widget(block_left,self.chunks_screen_cache[0]);
        frame.render_widget(block_right_top,self.chunks_screen_cache[1]);
        frame.render_widget(block_right_btm,self.chunks_screen_cache[2]);

        frame.render_widget(content_header_left,self.chunks_left_cache[0][1]);
        frame.render_widget(button1,self.chunks_left_cache[2][1]);
        frame.render_widget(button2,self.chunks_left_cache[5][1]);

        frame.render_stateful_widget(list_services_count,self.chunks_screen_inner_cache[1], &mut self.services_list_state);
        frame.render_stateful_widget(list_services,self.chunks_screen_inner_cache[2], &mut self.services_list_state);
        frame.render_stateful_widget(scrollbar, self.chunks_screen_inner_cache[3], &mut self.services_scrollbar_state);

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