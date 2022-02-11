mod util;
use schemars::JsonSchema;
use serde_json::{Value};
use rocket::serde::json::Json;
use util::*;

#[derive(Debug, JsonSchema)]
struct RocketTypes {
    date: Json,
}

#[test]
fn time03_types() -> TestResult {
    test_default_generated_schema::<Time03Types>("Json")
}


forward_impl!(Json => Value);