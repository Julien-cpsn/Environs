use crokey::{key, KeyCombination};
use crokey::OneToThree::One;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crate::app::App;
use crate::app_state::AppState::*;
use crate::business_logic::sort_env_variables::next_sort_mode;

impl App {
    pub async fn handle_events(&mut self) {
        // Refreshes the app every tick_rate
        if event::poll(self.tick_rate).unwrap() {
            // Block while a key is pressed
            if let Event::Key(key_event) = event::read().unwrap() {
                // We do not need
                if key_event.kind != KeyEventKind::Press {
                    return;
                }

                let key = KeyCombination::from(key_event);

                match self.state {
                    MainMenu => match key {
                        key!(ctrl-c) | key!(q) => self.should_quit = true,

                        KeyCombination { codes: One(KeyCode::Char('/')), modifiers: KeyModifiers::NONE } => self.filter_env_variables(),
                        key!(s) => self.env_variables_sort = next_sort_mode(&self.env_variables_sort),
                        
                        key!(up) | key!(k) => self.env_variables_list.previous(),
                        key!(down) | key!(j) => self.env_variables_list.next(),

                        key!(left) | key!(esc) => self.homepage(),
                        key!(right) | key!(enter) => self.select_request(),

                        _ => {}
                    }
                    FilteringEnvVariables => match key {
                        key!(esc) => self.escape_filtering(),
                        key!(enter) => self.confirm_filtering(),

                        key!(delete) => self.env_variables_filter.delete_char_forward(),
                        key!(backspace) => self.env_variables_filter.delete_char_backward(),

                        key!(left) => self.env_variables_filter.move_cursor_left(),
                        key!(right) => self.env_variables_filter.move_cursor_right(),

                        KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.env_variables_filter.enter_char(char),

                            _ => {}
                    },
                    EnvVariableSelected => match key {
                        key!(ctrl-c) | key!(q) => self.should_quit = true,
                        key!(esc) => self.main_menu(),

                        _ => {}
                    },
                }
            }
        }
    }
}