use strum::Display;
use crate::business_logic::sort_env_variables::SortMode::*;

#[derive(Display)]
pub enum SortMode {
    #[strum(to_string = "No sort")]
    None,

    #[strum(to_string = "Ascending")]
    Ascending,

    #[strum(to_string = "Descending")]
    Descending
}

pub fn next_sort_mode(sort_mode: &SortMode) -> SortMode {
    match sort_mode {
        None => Ascending,
        Ascending => Descending,
        Descending => None
    }
}