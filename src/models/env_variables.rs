use crate::models::env_values::EnvValue;
use crate::models::modification_types::ModificationType;

#[derive(Debug)]
pub struct EnvVariable {
    pub values: Vec<EnvValue>,
    pub modification_type: ModificationType
}