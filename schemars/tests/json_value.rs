mod util;

use std::collections::HashMap;
use schemars::JsonSchema;
use serde_json::Value;
use schemars::gen::SchemaSettings;
use util::*;

#[derive(Debug, JsonSchema)]
pub struct Configuration {
    pub parameters: HashMap<String, Value>,
}

#[test]
fn json_value_struct() -> TestResult {
    test_generated_schema::<Configuration>("json-value-struct", SchemaSettings::openapi3())
}
