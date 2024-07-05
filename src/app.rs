use std::time::Duration;

use ratatui::crossterm::terminal::disable_raw_mode;
use indexmap::IndexMap;
use ratatui::backend::Backend;
use ratatui::Terminal;

use crate::app_state::AppState;
use crate::business_logic::parse_env_variables::parse_env_variables;
use crate::business_logic::sort_env_variables::SortMode;
use crate::models::env_value_tab::EnvValueTab;
use crate::models::env_variables::EnvVariable;
use crate::utils::stateful_list::StatefulList;
use crate::utils::text_input::TextInput;

pub struct App {
    pub tick_rate: Duration,
    pub should_quit: bool,

    pub state: AppState,

    pub env_variables: IndexMap<String, EnvVariable>,

    pub env_variables_list: StatefulList<String>,
    pub env_variables_filter: TextInput,
    pub env_variables_sort: SortMode,

    pub env_values_list: StatefulList<String>,
    pub env_value_tab: EnvValueTab
}

impl App {
    pub fn new() -> App {
        App {
            tick_rate: Duration::from_millis(250),
            should_quit: false,

            state: AppState::MainMenu,

            env_variables: parse_env_variables(),

            env_variables_list: StatefulList::default(),
            env_variables_filter: TextInput::default(),
            env_variables_sort: SortMode::None,
            
            env_values_list: StatefulList::default(),
            env_value_tab: EnvValueTab::Both,
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.clear()?;

        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events().await;
        }

        Ok(())
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame | self.render_ui(frame))?;
        Ok(())
    }

    pub fn chain_hook(&mut self) -> &mut Self {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            disable_raw_mode().unwrap();
            original_hook(panic);
        }));

        self
    }
}