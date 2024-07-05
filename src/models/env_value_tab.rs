pub enum EnvValueTab {
    ModifiedValueOnly,
    OriginalValueOnly,
    Both
}

pub fn next_env_value_tab(env_value_tab: &EnvValueTab) -> EnvValueTab {
    match env_value_tab {
        EnvValueTab::ModifiedValueOnly => EnvValueTab::OriginalValueOnly,
        EnvValueTab::OriginalValueOnly => EnvValueTab::Both,
        EnvValueTab::Both => EnvValueTab::ModifiedValueOnly
    }
}