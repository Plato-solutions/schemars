use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use time03::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time};

macro_rules! formatted_string_impl {
    ($ty:ident, $format:literal) => {
        formatted_string_impl!($ty, $format, JsonSchema for $ty);
    };
    ($ty:ident, $format:literal, $($desc:tt)+) => {
        impl $($desc)+ {
            no_ref_schema!();

            fn schema_name() -> String {
                stringify!($ty).to_owned()
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    format: Some($format.to_owned()),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

formatted_string_impl!(Date, "date");
formatted_string_impl!(Time, "time");
formatted_string_impl!(OffsetDateTime, "date-time");
forward_impl!(Duration => std::time::Duration);

//
impl JsonSchema for PrimitiveDateTime {
    fn schema_name() -> String {
        "PrimitiveDateTime".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            ..Default::default()
        };
        let obj = schema.object();
        obj.required.insert("date".to_owned());
        obj.required.insert("time".to_owned());
        obj.properties
            .insert("date".to_owned(), <Date>::json_schema(gen));
        obj.properties
            .insert("time".to_owned(), <Time>::json_schema(gen));
        schema.into()
    }
}