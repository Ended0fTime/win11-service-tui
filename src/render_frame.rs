use crate::{App, ButtonHighlight, ButtonIdent, DARK_BLUE, LIGHT_BLUE, LIGHT_YELLOW, SERVICES, SERVICES_DESC, SERVICES_SHORT};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Alignment, Constraint, Direction::{self, Horizontal}, Layout, Rect}, 
    style::{Color, Style, Stylize, palette::material::YELLOW}, symbols::{border, scrollbar}, text::{Line, Text, Span}, 
    widgets::{Block, Borders, Padding, BorderType::{self, Rounded, Thick}, List, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState, ListState},
};


impl App {
    pub fn render_frame(&mut self, frame: &mut Frame) {
        // Outer frames
        let title = Line::from(" Ferlex TUI ").fg(LIGHT_YELLOW).italic();
        let block_left = Block::bordered().border_set(border::THICK)
            .border_style(Style::default().fg(LIGHT_BLUE))
            .title(title);
        let block_right_top = Block::bordered().border_set(border::THICK)
            .border_style(Style::default().fg(LIGHT_BLUE));
        let block_right_btm = Block::bordered().border_set(border::THICK)
            .border_style(Style::default().fg(LIGHT_BLUE));

        // Content declaration
        let para_right = Paragraph::new(Line::from("Select batch of services to disable.")
                .fg(LIGHT_YELLOW).bold().italic().centered())
            .wrap(Wrap {trim: true});
        let mut content1 = App::create_button("Safe".to_string(), Color::White);
        let mut content2 = App::create_button("Unsafe".to_string(), Color::White);
        let mut content3 = App::create_button("Exit".to_string(), LIGHT_YELLOW);

        let list_services = List::new(SERVICES.to_vec().clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 0}))
            .highlight_style(Style::default().bg(LIGHT_YELLOW).fg(DARK_BLUE).bold());
        let list_services_count = List::new(self.services_count_vec.clone())
            .block(Block::new()
            .padding(Padding{left: 0, right: 0, top: 1, bottom: 0}));
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
            Line::from("\n"), 
            Line::from(Span::styled(*serv_title_short, Style::default().bold()))])
            .block(Block::new().padding(Padding::new(3, 0, 1, 0))); 
        let para_details_desc = Paragraph::new(Line::from(vec![
            Span::styled(*serv_desc, Style::default().green())])) 
            .block(Block::new().padding(Padding::new(3, 0, 0, 0))); 

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
        frame.render_widget(block_left,self.chunks_screen_cache[0]);
        frame.render_widget(block_right_top,self.chunks_screen_cache[1]);
        frame.render_widget(block_right_btm,self.chunks_screen_cache[2]);

        frame.render_widget(para_right,self.chunks_left_cache[0][1]);
        frame.render_widget(para_details_title,self.chunks_screen_inner_cache[4]);
        frame.render_widget(para_details_desc,self.chunks_screen_inner_cache[5]);
        frame.render_widget(content1,self.chunks_left_cache[2][1]);
        frame.render_widget(content2,self.chunks_left_cache[5][1]);
        frame.render_widget(content3,self.chunks_left_cache[8][1]);

        frame.render_stateful_widget(list_services_count,self.chunks_screen_inner_cache[1], &mut self.services_list_state);
        frame.render_stateful_widget(list_services,self.chunks_screen_inner_cache[2], &mut self.services_list_state);
        frame.render_stateful_widget(scrollbar, self.chunks_screen_inner_cache[3], &mut self.services_scrollbar_state);
    }
}