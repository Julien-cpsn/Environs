use crate::app::App;
use crate::app_state::AppState;
use crate::app_state::AppState::FilteringEnvVariables;

impl App {
    pub fn homepage(&mut self) {
        self.env_variables_list.unselect();
        self.state = AppState::MainMenu;
    }

    pub fn main_menu(&mut self) {
        self.state = AppState::MainMenu;
    }

    pub fn filter_env_variables(&mut self) {
        self.env_variables_list.unselect();
        self.state = FilteringEnvVariables;
    }

    pub fn select_request(&mut self) {
        if self.env_variables_list.state.selected().is_none() {
            return;
        }

        self.env_variables_list.select();
        self.state = AppState::EnvVariableSelected;
    }

    pub fn edit_env_value(&mut self) {
        if self.env_values_list.state.selected().is_none() {
            return;
        }

        self.env_values_list.select();
        self.state = AppState::EditingEnvValue;
    }
}