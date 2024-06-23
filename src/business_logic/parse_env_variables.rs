use std::env;
use indexmap::IndexMap;
use crate::models::env_values::EnvValue;
use crate::models::env_variables::EnvVariable;
use crate::models::modification_types::ModificationType;

pub fn parse_env_variables() -> IndexMap<String, EnvVariable> {
    let mut env_variables = IndexMap::new();

    for (key, values) in env::vars() {
        let split_values: Vec<EnvValue> = values
            .split(":")
            .map(|value|
                EnvValue {
                    original_value: value.to_string(),
                    value: value.to_string(),
                    modification_type: ModificationType::None,
                }
            )
            .collect();

        let env_variable = EnvVariable {
            values: split_values,
            modification_type: ModificationType::None,
        };

        env_variables.insert(key, env_variable);
    }

    return env_variables;
}