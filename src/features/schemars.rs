use chrono::{DateTime, TimeZone};
use schemars_08::{gen::SchemaGenerator, schema::Schema, JsonSchema};

use crate::DateTimeDefaultNow;

impl<Tz, const OFFSET_HOURS: i32> JsonSchema for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn schema_name() -> String {
        DateTime::<Tz>::schema_name()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        DateTime::<Tz>::json_schema(gen)
    }
}

#[cfg(test)]
mod test {
    use chrono::FixedOffset;
    use schemars::schema_for;
    use schemars_08 as schemars;
    use serde_json::json;

    use crate::DateTimeDefaultNow;

    #[test]
    fn schemas_test() {
        #[allow(dead_code)]
        #[derive(Default, schemars::JsonSchema)]
        struct A {
            update_at: DateTimeDefaultNow<FixedOffset>,
        }

        let schema = schema_for!(A);
        assert_eq!(
            serde_json::to_value(&schema).unwrap(),
            json!(
                {
                    "$schema": "http://json-schema.org/draft-07/schema#",
                    "title": "A",
                    "type": "object",
                    "required": [
                        "update_at"
                    ],
                    "properties": {
                        "update_at": {
                            "$ref": "#/definitions/DateTime"
                        }
                    },
                    "definitions": {
                        "DateTime": {
                            "type": "string",
                            "format": "date-time"
                        }
                    }
                }
            )
        );
    }
}
