use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::Modifier;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;

use crate::app::App;

impl App {
    pub fn render_ui(&mut self, frame: &mut Frame) {
        // MAIN LAYOUT

        let main_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ],
        )
            .split(frame.size());

        // HEADER

        let header = Block::new()
            .title("* Environs *")
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        frame.render_widget(header, main_layout[0]);

        // INNER LAYOUT

        let inner_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(75)
            ],
        )
            .split(main_layout[1]);

        self.render_env_variables_list(frame, inner_layout[0]);

        match self.env_variables_list.selected.clone() {
            None => self.render_homepage(frame, inner_layout[1]),
            Some(selection) => {
                self.render_environment_values(frame, inner_layout[1], selection);
            }
        }

        // FOOTER

        let state_line = self.get_state_line();
        let available_keys = self.get_available_keys_line();

        let footer = Block::new()
            .title(Title::from(state_line).alignment(Alignment::Left))
            .title(Title::from(available_keys).alignment(Alignment::Right));

        frame.render_widget(footer, main_layout[2]);
    }
}