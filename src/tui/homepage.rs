use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::widgets::{Block, Paragraph};
use tui_big_text::{BigTextBuilder, PixelSize};
use crate::app::App;

impl App {
    pub(super) fn render_homepage(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::new();

        let inner_block_area = block.inner(rect);

        let inner_layout = Layout::new(
            Vertical,
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Percentage(50)
            ]
        )
            .split(inner_block_area);

        let title_length = 32;

        let title_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage((100-title_length)/2+2),
                Constraint::Length(title_length),
            ]
        )
            .split(inner_layout[2]);

        let title = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines([
                "Environs".into(),
            ])
            .build()
            .unwrap();


        let welcome_to = Paragraph::new("Welcome to").centered();
        let description = Paragraph::new("Manage your environment variables").centered();

        frame.render_widget(block, rect);
        frame.render_widget(welcome_to, inner_layout[1]);
        frame.render_widget(title, title_layout[1]);
        frame.render_widget(description, inner_layout[3]);
    }
}