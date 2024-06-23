use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::style::Color::Blue;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, Paragraph};
use rayon::prelude::*;

use crate::app::App;
use crate::app_state::AppState;
use crate::models::env_variables::EnvVariable;

impl App {
    pub(super) fn render_environment_variable(&self, frame: &mut Frame, rect: Rect, environment_variable_name: &str, environment_variable: &EnvVariable) {
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


        let environment_variable_name_paragraph = Paragraph::new(environment_variable_name).centered().white();

        frame.render_widget(block, rect);
        frame.render_widget(environment_variable_name_paragraph, inner_layout[1]);

        let values: Vec<String> = environment_variable.values
            .par_iter()
            .map(|value| value.value.clone())
            .collect();
        
        let list_items = values.len().to_string();
        let list_items_paragraph = Paragraph::new(list_items).centered();
        frame.render_widget(list_items_paragraph, inner_layout[2]);

        let mut values_block = Block::default()
            .title("Values")
            .borders(Borders::ALL);
        
        if self.state == AppState::EnvVariableSelected {
            values_block = values_block.fg(Blue);
        }
        
        let environment_values_list = List::new(values)
            .white()
            .block(values_block);

        frame.render_widget(environment_values_list, inner_layout[3])
    }
}