use ratatui::Frame;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::widgets::block::Title;

use crate::app::App;
use crate::models::env_value_tab::EnvValueTab;
use crate::utils::centered_rect::centered_rect;

impl App {
    pub fn render_environment_value_popup(&mut self, frame: &mut Frame, env_variable_selection: usize, env_value_selection: usize) {
        let key = &self.env_variables_list.items[env_variable_selection];
        let env_variable = self.env_variables.get(key).unwrap();
        let env_value = &env_variable.values[env_value_selection];

        let centered_rect = centered_rect(100, 25, frame.size());

        let env_value_tab_string = match self.env_value_tab {
            EnvValueTab::ModifiedValueOnly | EnvValueTab::Both => "Modified value",
            EnvValueTab::OriginalValueOnly => "Original value",
        };

        let block = Block::new()
            .title(
                Title::from(
                    Line::from(vec![
                        Span::raw(format!("─< {env_value_tab_string}")).white(),
                        Span::raw(format!(" ─ {key} ({}/{}) >", env_value_selection + 1, env_variable.values.len(), )).gray(),
                    ])
                )
            )
            .borders(Borders::ALL)
            .on_dark_gray();


        let inner_block_area = block.inner(centered_rect);

        frame.render_widget(Clear, inner_block_area);
        frame.render_widget(block, inner_block_area);
        
        match self.env_value_tab {
            EnvValueTab::ModifiedValueOnly => {
                let modified_value_layout = Layout::new(
                    Vertical,
                    vec![
                        Constraint::Fill(1)
                    ]
                )
                    .margin(1)
                    .split(inner_block_area);
                
                frame.render_widget("test", modified_value_layout[0]);
            }
            EnvValueTab::OriginalValueOnly => {
                let original_value_layout = Layout::new(
                    Vertical,
                    vec![
                        Constraint::Fill(1)
                    ]
                )
                    .margin(1)
                    .split(inner_block_area);

                frame.render_widget(&env_value.original_value, original_value_layout[0]);
            }
            EnvValueTab::Both => {
                let both_layout = Layout::new(
                    Vertical,
                    vec![
                        Constraint::Percentage(49),
                        Constraint::Percentage(51)
                    ]
                )
                    .margin(1)
                    .split(inner_block_area);

                let original_value_paragraph = Paragraph::new(env_value.original_value.clone())
                    .block(
                        Block::new()
                            .title(Title::from("─< Original value >"))
                            .borders(Borders::TOP)
                    );
                
                frame.render_widget("test", both_layout[0]);
                frame.render_widget(original_value_paragraph, both_layout[1]);
            }
        }
    }
}