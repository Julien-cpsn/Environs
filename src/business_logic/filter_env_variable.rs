use crate::app::App;
use crate::app_state::AppState;

impl App {
    pub fn escape_filtering(&mut self) {
        self.env_variables_filter.reset_input();
        self.state = AppState::MainMenu;
    }

    pub fn confirm_filtering(&mut self) {
        self.state = AppState::MainMenu;
    }
}