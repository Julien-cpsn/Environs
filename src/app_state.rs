use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use strum::Display;
use crate::app::App;
use crate::app_state::AppState::*;

#[derive(PartialEq, Display)]
pub enum AppState {
    #[strum(to_string = "Main menu")]
    MainMenu,

    #[strum(to_string = "Filtering")]
    FilteringEnvVariables,

    #[strum(to_string = "Variable selected")]
    EnvVariableSelected
}

impl App {
    pub fn get_state_line(&self) -> Line {
        let filter_text = self.env_variables_filter.text.clone();
        let filter_state = match filter_text.len() {
            0 => Span::raw("No filter").gray().on_dark_gray(),
            _ => Span::raw(filter_text).on_dark_gray()
        };

        match self.state {
            MainMenu => Line::from(vec![
                self.state.to_string().on_dark_gray(),
                format!(" ({}, {})", filter_state, self.env_variables_sort).gray().on_dark_gray(),
            ]),

            FilteringEnvVariables => {
                Line::from(vec![
                    format!("{} > ", self.state.to_string()).dark_gray(),
                    filter_state
                ])
            },
            EnvVariableSelected => {
                let (key, _) = self.env_variables.get_index(self.env_variables_list.selected.unwrap()).unwrap();
                let key = key.clone();

                Line::from(vec![
                    format!("{} > ", self.state.to_string()).dark_gray(),
                    key.white().on_dark_gray()
                ])
            }
        }
    }

    pub fn get_available_keys_line(&self) -> Line {
        let available_keys = match self.state {
            MainMenu => vec!["Exit", "ctrl-c q", "Filter", "/", "Sort", "s", "Up", "up k", "Down", "down j"],
            FilteringEnvVariables => vec!["Quit", "esc", "Confirm", "enter"],
            EnvVariableSelected => vec!["Exit", "ctrl-c q", "Quit", "esc"],
        };

        let mut line = Line::default();

        for (index, key) in available_keys.iter().enumerate() {
            let key_span = match index % 2 == 0 {
                true => Span::raw(key.to_string()).white().on_dark_gray(),
                false => Span::raw(key.to_string()).dark_gray()
            };

            line.push_span(Span::raw(" "));
            line.push_span(key_span);
        }

        return line;
    }
}