use indexmap::IndexMap;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Color::DarkGray;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List};
use rayon::prelude::*;

use crate::app::App;
use crate::app_state::AppState;
use crate::business_logic::sort_env_variables::SortMode;
use crate::models::env_variables::EnvVariable;
use crate::models::modification_types::ModificationType;

impl App {
    pub fn render_env_variables_list(&mut self, frame: &mut Frame, area: Rect) {
        let filtered_and_sorted_env_variables = filter_and_sort_env_variables(&mut self.env_variables, &mut self.env_variables_list.items, &self.env_variables_filter.text, &self.env_variables_sort);

        let number_variables = filtered_and_sorted_env_variables.len();
        let selection = match self.env_variables_list.state.selected() {
            None => String::from("?"),
            Some(selection) => (selection + 1).to_string()
        };

        let mut env_block = Block::default()
            .title(format!("—< Variables ({selection}/{number_variables}) >"))
            .borders(Borders::ALL);

        if self.state != AppState::MainMenu {
            env_block = env_block.fg(DarkGray);
        }

        let env_list = List::new(filtered_and_sorted_env_variables)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .block(env_block);

        frame.render_stateful_widget(env_list, area, &mut self.env_variables_list.state);

        if self.state == AppState::FilteringEnvVariables {
            let frame_size = frame.size();
            let cursor_adjustment = self.env_variables_filter.cursor_position;

            frame.set_cursor(12 + cursor_adjustment as u16, frame_size.height - 1);
        }
    }
}

fn filter_and_sort_env_variables<'a>(env_variables: &IndexMap<String, EnvVariable>, items: &'a mut Vec<String>, filter: &String, sort_mode: &SortMode) -> Vec<Line<'a>> {
    let should_filter = filter.len() != 0;

    let mut local_env_variables: IndexMap<&String, &EnvVariable> = env_variables.iter().clone().collect();

    match sort_mode {
        SortMode::None => {}
        SortMode::Ascending => local_env_variables.par_sort_keys(),
        SortMode::Descending => local_env_variables.par_sort_by(|key_a, _, key_b, _| key_b.cmp(key_a))
    }

    *items = local_env_variables
        .par_keys()
        .filter_map(|key| {
            if should_filter && !key.contains(filter) {
                None
            }
            else {
                Some(key.to_string())
            }
        })
        .collect();

    let mut lines: Vec<Line> = vec![];

    for key in items.iter() {
        let variable = local_env_variables[&key];
        let key = format!(" {key}");

        let line = match variable.modification_type {
            ModificationType::None => Line::raw(key.clone()).white(),
            ModificationType::Addition => Line::from(vec![Span::raw("+"), Span::raw(key)]).white().on_light_green(),
            ModificationType::Deletion => Line::from(vec![Span::raw("﹘"), Span::raw(key)]).white().on_light_red(),
            ModificationType::Change => Line::from(vec![Span::raw("~"), Span::raw(key)]).white().on_light_blue(),
        };

        lines.push(line)
    }

    return lines;
}