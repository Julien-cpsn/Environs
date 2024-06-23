use crate::models::modification_types::ModificationType;

#[derive(Debug)]
pub struct EnvValue {
    pub original_value: String,
    pub value: String,
    pub modification_type: ModificationType
}