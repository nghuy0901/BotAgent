use schemars::{JsonSchema, generate::SchemaSettings, json_schema};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

pub trait Arguments: DeserializeOwned + JsonSchema {
    fn into_schema() -> Value {
        let mut settings = SchemaSettings::draft07();

        settings.inline_subschemas = true;

        let generator = settings.into_generator();
        let mut arguments = generator.into_root_schema_for::<Self>().to_value();

        if let Some(o) = arguments.as_object_mut() {
            o.remove("$schema");
            o.remove("title");
            o.entry("properties").or_insert(json!({}));
        }

        arguments
    }
}

impl<P: DeserializeOwned + JsonSchema> Arguments for P {}

#[derive(Deserialize)]
pub struct EmptyArguments {}

impl JsonSchema for EmptyArguments {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "".into() // discarded anyway
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "properties": {}
        })
    }
}
