use schemars::{JsonSchema, generate::SchemaSettings, json_schema};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

pub trait Parameters: DeserializeOwned + JsonSchema {
    fn into_schema() -> Value {
        let mut settings = SchemaSettings::draft07();

        settings.inline_subschemas = true;

        let generator = settings.into_generator();
        let mut parameters = generator.into_root_schema_for::<Self>().to_value();

        if let Some(o) = parameters.as_object_mut() {
            o.remove("$schema");
            o.remove("title");
            o.entry("properties").or_insert(json!({}));
        }

        parameters
    }
}

impl<P: DeserializeOwned + JsonSchema> Parameters for P {}

#[derive(Deserialize)]
pub struct EmptyParameters {}

impl JsonSchema for EmptyParameters {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "EmptyParameters".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "properties": {}
        })
    }
}
