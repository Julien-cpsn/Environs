use ratatui::Frame;
use ratatui::prelude::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear};
use ratatui::widgets::block::Title;

use crate::app::App;
use crate::utils::centered_rect::centered_rect;

impl App {
    pub fn render_environment_value_popup(&mut self, frame: &mut Frame, env_variable_selection: usize, env_value_selection: usize) {
        let key = &self.env_variables_list.items[env_variable_selection];
        let env_variable = self.env_variables.get(key).unwrap();
        let env_value = &env_variable.values[env_value_selection];

        let centered_rect = centered_rect(75, 25, frame.size());

        let block = Block::new()
            .title(
                Title::from(
                    Line::from(vec![
                        Span::raw(format!("{key}")).white(),
                        Span::raw(format!(" ({}/{})", env_value_selection + 1, env_variable.values.len())).gray(),
                    ])
                )
            )
            .borders(Borders::ALL)
            .on_dark_gray();

        let inner_block_area = block.inner(centered_rect);

        frame.render_widget(Clear, inner_block_area);
        frame.render_widget(block, inner_block_area);
    }
}