mod util;
use schemars::JsonSchema;
use time03::{Date,Time,PrimitiveDateTime,OffsetDateTime,Duration};
use util::*;

#[derive(Debug, JsonSchema)]
struct Time03Types {
    date: Date,
    time: Time,
    primitive_date: PrimitiveDateTime,
    offset_date_time: OffsetDateTime,
    duration: Duration,
}

#[test]
fn time03_types() -> TestResult {
    test_default_generated_schema::<Time03Types>("time03-types")
}
