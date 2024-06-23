use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Color::DarkGray;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, Paragraph};
use rayon::prelude::*;

use crate::app::App;
use crate::app_state::AppState;

impl App {
    pub(super) fn render_environment_variable(&mut self, frame: &mut Frame, rect: Rect, selection: usize) {
        let key = &self.env_variables_list.items[selection];
        let environment_variable = self.env_variables.get_mut(key).unwrap();

        let block = Block::new();

        let inner_block_area = block.inner(rect);

        let inner_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1)
            ]
        )
            .horizontal_margin(1)
            .split(inner_block_area);

        frame.render_widget(block, rect);

        let mut environment_variable_name_paragraph = Paragraph::new(key.clone()).centered().white();

        let values: Vec<String> = environment_variable.values
            .par_iter()
            .map(|value| value.value.clone())
            .collect();

        let number_values = values.len();
        let selection = match self.env_values_list.state.selected() {
            None => String::from("?"),
            Some(selection) => (selection + 1).to_string()
        };

        self.env_values_list.items = values.clone();

        let mut environment_values_list = List::new(values)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .block(
                Block::default()
                    .title(format!("—< Values ({selection}/{number_values}) >"))
                    .borders(Borders::ALL)
            );
        
        if self.state != AppState::EnvVariableSelected {
            environment_values_list = environment_values_list.fg(DarkGray);
            environment_variable_name_paragraph = environment_variable_name_paragraph.fg(DarkGray);
        }

        frame.render_widget(environment_variable_name_paragraph, inner_layout[1]);
        frame.render_stateful_widget(environment_values_list, inner_layout[3], &mut self.env_values_list.state)
    }
}