use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::style::Color::DarkGray;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, Paragraph};
use rayon::prelude::*;

use crate::app::App;
use crate::app_state::AppState;
use crate::models::env_values::EnvValue;
use crate::models::modification_types::ModificationType;

impl App {
    pub fn render_environment_values(&mut self, frame: &mut Frame, rect: Rect, selection: usize) {
        let key = &self.env_variables_list.items[selection];
        let env_variable = self.env_variables.get(key).unwrap();

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

        let mapped_values = map_env_values(&env_variable.values, &mut self.env_values_list.items);
        
        let number_values = mapped_values.len();
        let selection = match self.env_values_list.state.selected() {
            None => String::from("?"),
            Some(selection) => (selection + 1).to_string()
        };
        
        let mut environment_values_list = List::new(mapped_values)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .block(
                Block::default()
                    .title(format!("─< Values ({selection}/{number_values}) >"))
                    .borders(Borders::ALL)
            );
        
        if self.state != AppState::EnvVariableSelected {
            environment_values_list = environment_values_list.fg(DarkGray);
            environment_variable_name_paragraph = environment_variable_name_paragraph.fg(DarkGray);
        }

        frame.render_widget(environment_variable_name_paragraph, inner_layout[1]);
        frame.render_stateful_widget(environment_values_list, inner_layout[3], &mut self.env_values_list.state);
    }
}

fn map_env_values<'a>(env_values: &'a Vec<EnvValue>, items: &'a mut Vec<String>) -> Vec<Line<'a>> {
    *items = env_values
        .par_iter()
        .map(|env_value| {
            env_value.value.to_string()
        })
        .collect();

    let mut lines: Vec<Line> = vec![];

    for env_value in env_values {
        let mut line = Line::raw(env_value.value.clone()).white();

        line = match env_value.modification_type {
            ModificationType::None => line,
            ModificationType::Addition => line.on_light_green(),
            ModificationType::Deletion => line.on_light_red(),
            ModificationType::Change => line.on_light_blue(),
        };

        lines.push(line)
    }

    return lines;
}