//! On-disk formats for entity config files: JSON, YAML and TOML.
//!
//! Configs are held in memory as raw wire JSON ([`Value`], see
//! [`super::project`]), so each format is only a different rendering of the
//! same value. The format of a file is picked by its extension; anything that
//! isn't `.yaml`/`.yml`/`.toml` is JSON, so existing projects read exactly as
//! before. Index files (`agents.json` & co.) are always JSON.
//!
//! TOML has no `null`, so writing TOML drops null fields. For an agent that
//! loses nothing: `push` sends a PATCH, where a missing field is left as it
//! is, and a field that pulled as null was unset already. What TOML can't
//! express is *clearing* a field: deleting its line leaves the remote value
//! alone. That needs `null` in a JSON or YAML config.
//!
//! Some nulls are data rather than unset fields, and a config holding one
//! can't be written as TOML at all: any null inside an array that isn't a
//! plain array of objects, and any null within a free-form JSON value such as
//! a tool parameter's `constant_value` (see [`LITERAL_KEYS`]).

use std::path::Path;

use fern_cli_sdk::error::CliError;
use serde_json::{Map, Value};
use toml_edit::{Array, ArrayOfTables, DocumentMut, InlineTable, Item, Table};

/// The format of a config file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

/// Values accepted by `--config-format`.
pub const FORMAT_NAMES: [&str; 3] = ["json", "yaml", "toml"];

impl ConfigFormat {
    /// The format a path is read and written in, from its extension.
    pub fn from_path(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase);
        match ext.as_deref() {
            Some("yaml" | "yml") => Self::Yaml,
            Some("toml") => Self::Toml,
            _ => Self::Json,
        }
    }

    /// Parse a `--config-format` value (one of [`FORMAT_NAMES`]).
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }

    /// Extension, with its leading dot, for a newly generated config file.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Json => ".json",
            Self::Yaml => ".yaml",
            Self::Toml => ".toml",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Yaml => "YAML",
            Self::Toml => "TOML",
        }
    }

    /// Parse a config file's text.
    pub fn parse(self, text: &str) -> Result<Value, String> {
        match self {
            Self::Json => serde_json::from_str(text).map_err(|e| e.to_string()),
            Self::Yaml => {
                let yaml: serde_yaml::Value =
                    serde_yaml::from_str(text).map_err(|e| e.to_string())?;
                check_yaml_numbers(&yaml, "")?;
                serde_yaml::from_value(yaml).map_err(|e| e.to_string())
            }
            Self::Toml => {
                let doc: DocumentMut = text.parse().map_err(|e| format!("{e}"))?;
                table_to_json(doc.as_table())
            }
        }
    }

    /// Render a config for writing. JSON goes through
    /// [`super::project::to_pretty_string`] for v0's 4-space indent.
    pub fn render(self, value: &Value) -> Result<String, CliError> {
        match self {
            Self::Json => super::project::to_pretty_string(value),
            Self::Yaml => serde_yaml::to_string(value)
                .map_err(|e| CliError::Other(anyhow::anyhow!("Could not serialize YAML: {e}"))),
            Self::Toml => json_to_toml(value).map_err(CliError::Validation),
        }
    }
}

// ── YAML → JSON ─────────────────────────────────────────────────────

/// YAML's `.nan` and `.inf` would quietly become `null` in JSON, which
/// `push` then sends as a cleared field. Refuse them, as the TOML reader does.
fn check_yaml_numbers(value: &serde_yaml::Value, path: &str) -> Result<(), String> {
    use serde_yaml::Value as Y;
    match value {
        Y::Number(n) if n.is_nan() || n.is_infinite() => {
            let at = if path.is_empty() { "value" } else { path };
            Err(format!("{at}: {n} has no JSON equivalent"))
        }
        Y::Sequence(items) => items
            .iter()
            .enumerate()
            .try_for_each(|(i, item)| check_yaml_numbers(item, &format!("{path}[{i}]"))),
        Y::Mapping(map) => map.iter().try_for_each(|(key, item)| {
            let key = match key {
                Y::String(s) => s.clone(),
                other => serde_yaml::to_string(other)
                    .map(|s| s.trim_end().to_string())
                    .unwrap_or_default(),
            };
            check_yaml_numbers(item, &join(path, &key))
        }),
        Y::Tagged(tagged) => check_yaml_numbers(&tagged.value, path),
        _ => Ok(()),
    }
}

// ── JSON → TOML ─────────────────────────────────────────────────────

fn json_to_toml(value: &Value) -> Result<String, String> {
    let Value::Object(map) = value else {
        return Err("A TOML config must be an object at the top level".to_string());
    };
    let mut doc = DocumentMut::new();
    fill_table(doc.as_table_mut(), map, "", false)?;
    Ok(doc.to_string())
}

/// API fields whose value is free-form JSON sent through as it is, so a
/// null inside one is part of the value. The field itself being null still
/// just means it's unset.
const LITERAL_KEYS: [&str; 5] = [
    "constant_value",
    "dynamic_variable_placeholders",
    "dynamic_variables",
    "input_schema",
    "output_schema",
];

/// `path` is the dotted key path of `map`, for error messages. `literal` is
/// set within a [`LITERAL_KEYS`] value, where a null field is refused rather
/// than dropped.
fn fill_table(
    table: &mut Table,
    map: &Map<String, Value>,
    path: &str,
    literal: bool,
) -> Result<(), String> {
    for (key, value) in map {
        let item_path = join(path, key);
        let literal_child = literal || LITERAL_KEYS.contains(&key.as_str());
        match to_item(value, &item_path, literal_child)? {
            Some(item) => {
                table.insert(key, item);
            }
            None if literal => return Err(literal_null(&item_path)),
            None => {}
        }
    }
    Ok(())
}

fn join(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

/// A null that is data (in an array, or within a constant value), not an
/// unset field: dropping it would change what `push` sends, so refuse
/// instead. `path` is where the null is.
fn literal_null(path: &str) -> String {
    format!(
        "{path} is null, which TOML can't represent: inside an array or a \
         free-form value like constant_value, a null is data. \
         Use a JSON or YAML config for this one"
    )
}

/// Objects become `[table]` sections and arrays of objects `[[array]]`
/// sections, which keeps a deeply nested agent config readable. `None` for
/// a null field, which TOML can't hold.
fn to_item(value: &Value, path: &str, literal: bool) -> Result<Option<Item>, String> {
    match value {
        Value::Object(map) => {
            let mut table = Table::new();
            fill_table(&mut table, map, path, literal)?;
            // An implicit table prints no header of its own when it holds
            // only subtables. An empty one must keep its header, or the
            // (empty) object would vanish from the file. Checked after
            // filling, since an object of nulls ends up empty too.
            table.set_implicit(!table.is_empty());
            Ok(Some(Item::Table(table)))
        }
        Value::Array(items) if !items.is_empty() && items.iter().all(Value::is_object) => {
            let mut tables = ArrayOfTables::new();
            for (i, map) in items.iter().filter_map(Value::as_object).enumerate() {
                let mut table = Table::new();
                fill_table(&mut table, map, &format!("{path}[{i}]"), literal)?;
                tables.push(table);
            }
            Ok(Some(Item::ArrayOfTables(tables)))
        }
        other => Ok(to_value(other, path)?.map(Item::Value)),
    }
}

/// An inline TOML value: used for scalars, and for objects and arrays nested
/// inside an array that isn't a plain array of objects. Any object reached
/// here is inside such an array, so its null fields are refused too.
fn to_value(value: &Value, path: &str) -> Result<Option<toml_edit::Value>, String> {
    Ok(Some(match value {
        Value::Null => return Ok(None),
        Value::Bool(b) => (*b).into(),
        Value::Number(n) => match (n.as_i64(), n.as_f64()) {
            (Some(i), _) => i.into(),
            // A u64 above i64::MAX would silently lose precision as a float.
            (None, Some(f)) if n.is_f64() => f.into(),
            _ => return Err(format!("{path}: {n} is too large for a TOML integer")),
        },
        Value::String(s) => s.as_str().into(),
        Value::Array(items) => {
            let mut array = Array::new();
            for (i, item) in items.iter().enumerate() {
                let item_path = format!("{path}[{i}]");
                match to_value(item, &item_path)? {
                    Some(v) => array.push(v),
                    None => return Err(literal_null(&item_path)),
                }
            }
            array.into()
        }
        Value::Object(map) => {
            let mut table = InlineTable::new();
            for (key, item) in map {
                let item_path = join(path, key);
                match to_value(item, &item_path)? {
                    Some(v) => table.insert(key, v),
                    None => return Err(literal_null(&item_path)),
                };
            }
            table.into()
        }
    }))
}

// ── TOML → JSON ─────────────────────────────────────────────────────

fn table_to_json(table: &Table) -> Result<Value, String> {
    let mut map = Map::new();
    for (key, item) in table.iter() {
        map.insert(key.to_string(), item_to_json(item)?);
    }
    Ok(Value::Object(map))
}

fn item_to_json(item: &Item) -> Result<Value, String> {
    match item {
        Item::None => Ok(Value::Null),
        Item::Value(v) => value_to_json(v),
        Item::Table(t) => table_to_json(t),
        Item::ArrayOfTables(tables) => tables.iter().map(table_to_json).collect(),
    }
}

fn value_to_json(value: &toml_edit::Value) -> Result<Value, String> {
    use toml_edit::Value as T;
    Ok(match value {
        T::String(s) => Value::String(s.value().clone()),
        T::Integer(i) => (*i.value()).into(),
        T::Float(f) => serde_json::Number::from_f64(*f.value())
            .map(Value::Number)
            .ok_or_else(|| format!("{} has no JSON equivalent", f.value()))?,
        T::Boolean(b) => Value::Bool(*b.value()),
        // The API has no datetime type: it takes these as strings.
        T::Datetime(d) => Value::String(d.value().to_string()),
        T::Array(items) => items.iter().map(value_to_json).collect::<Result<_, _>>()?,
        T::InlineTable(table) => {
            let mut map = Map::new();
            for (key, v) in table.iter() {
                map.insert(key.to_string(), value_to_json(v)?);
            }
            Value::Object(map)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample() -> Value {
        json!({
            "name": "Support",
            "tags": ["a", "b"],
            "conversation_config": {
                "agent": {
                    "first_message": "Hi!",
                    "prompt": {
                        "prompt": "Line one\nLine \"two\"",
                        "temperature": 0.5,
                        "tool_ids": [],
                        "tools": [
                            {"name": "lookup", "params": {"retries": 3}},
                            {"name": "end_call", "params": {}}
                        ]
                    }
                },
                "tts": {"voice_id": "abc", "stability": 1, "enabled": true}
            },
            "platform_settings": {},
            "mixed": [1, "two", {"three": 3}, [4]]
        })
    }

    #[test]
    fn format_follows_the_extension() {
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b.json")),
            ConfigFormat::Json
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b.yaml")),
            ConfigFormat::Yaml
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b.YML")),
            ConfigFormat::Yaml
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b.toml")),
            ConfigFormat::Toml
        );
        // No extension, or an unknown one, stays JSON as it always was.
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b")),
            ConfigFormat::Json
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("a/b.txt")),
            ConfigFormat::Json
        );
    }

    #[test]
    fn every_name_parses_and_round_trips_its_extension() {
        for name in FORMAT_NAMES {
            let format = ConfigFormat::from_name(name).expect(name);
            let path = format!("x{}", format.extension());
            assert_eq!(ConfigFormat::from_path(Path::new(&path)), format);
        }
        assert_eq!(ConfigFormat::from_name("xml"), None);
    }

    #[test]
    fn yaml_and_toml_round_trip_losslessly() {
        for format in [ConfigFormat::Json, ConfigFormat::Yaml, ConfigFormat::Toml] {
            let rendered = format.render(&sample()).unwrap();
            let parsed = format.parse(&rendered).unwrap();
            assert_eq!(parsed, sample(), "{format:?}:\n{rendered}");
        }
    }

    #[test]
    fn toml_uses_sections_for_nested_objects() {
        let rendered = ConfigFormat::Toml.render(&sample()).unwrap();
        assert!(rendered.contains("[conversation_config.tts]"), "{rendered}");
        assert!(
            rendered.contains("[[conversation_config.agent.prompt.tools]]"),
            "{rendered}"
        );
        // Only-subtables parents print no header of their own.
        assert!(!rendered.contains("[conversation_config]\n"), "{rendered}");
        // An empty object keeps its header so it survives the round trip.
        assert!(rendered.contains("[platform_settings]"), "{rendered}");
    }

    #[test]
    fn toml_drops_null_fields() {
        let value = json!({
            "name": "x",
            "language": null,
            "nested": {"keep": 1, "drop": null},
            "only_nulls": {"drop": null},
            "tables": [{"a": 1, "b": null}]
        });
        let rendered = ConfigFormat::Toml.render(&value).unwrap();
        assert_eq!(
            ConfigFormat::Toml.parse(&rendered).unwrap(),
            json!({
                "name": "x",
                "nested": {"keep": 1},
                // Kept as an empty object rather than vanishing.
                "only_nulls": {},
                "tables": [{"a": 1}]
            }),
            "{rendered}"
        );
    }

    #[test]
    fn toml_refuses_nulls_in_arrays() {
        // A null array element is data, so dropping it would change the
        // config: a tool's constant value, for instance.
        for value in [
            json!({"tool": {"constant_value": [1, null, 2]}}),
            json!({"tables": [{"a": 1}, null]}),
            json!({"nested": [[null]]}),
            // An object mixed in with other values is literal data too.
            json!({"mixed": [1, {"a": null}]}),
            json!({"nested": [[{"a": {"b": null}}]]}),
            // So is anything within a free-form value, even a list of objects.
            json!({"tool": {"constant_value": {"key": null}}}),
            json!({"tool": {"constant_value": [{"key": null}]}}),
            json!({"dynamic_variables": {"user": null}}),
        ] {
            let err = ConfigFormat::Toml.render(&value).unwrap_err().to_string();
            assert!(err.contains("Use a JSON or YAML config"), "{err}");
        }
        let err = ConfigFormat::Toml
            .render(&json!({"tool": {"constant_value": [1, null]}}))
            .unwrap_err()
            .to_string();
        assert!(err.starts_with("tool.constant_value[1] is null"), "{err}");
        let err = ConfigFormat::Toml
            .render(&json!({"mixed": [1, {"a": null}]}))
            .unwrap_err()
            .to_string();
        assert!(err.starts_with("mixed[1].a is null"), "{err}");
        let err = ConfigFormat::Toml
            .render(&json!({"tool": {"constant_value": {"key": null}}}))
            .unwrap_err()
            .to_string();
        assert!(err.starts_with("tool.constant_value.key is null"), "{err}");
        // A free-form field that is null itself is just unset.
        assert_eq!(
            ConfigFormat::Toml
                .render(&json!({"tool": {"constant_value": null, "a": 1}}))
                .unwrap(),
            "[tool]\na = 1\n"
        );
    }

    #[test]
    fn toml_rejects_what_it_cannot_hold() {
        assert!(ConfigFormat::Toml.render(&json!([1, 2])).is_err());
        assert!(ConfigFormat::Toml
            .render(&json!({"big": u64::MAX}))
            .unwrap_err()
            .to_string()
            .contains("too large"));
        assert!(ConfigFormat::Toml.parse("x = nan").is_err());
    }

    #[test]
    fn yaml_rejects_non_finite_numbers() {
        // These would otherwise read as null and clear the field on push.
        for text in ["temperature: .nan", "a:\n  b: [1, .inf]", "x: -.inf"] {
            assert!(
                ConfigFormat::Yaml.parse(text).is_err(),
                "{text} should be refused"
            );
        }
        let err = ConfigFormat::Yaml
            .parse("tts:\n  stability: .nan")
            .unwrap_err();
        assert!(err.starts_with("tts.stability:"), "{err}");
        assert_eq!(
            ConfigFormat::Yaml.parse("t: 0.5").unwrap(),
            json!({"t": 0.5})
        );
    }

    #[test]
    fn toml_datetimes_read_as_strings() {
        assert_eq!(
            ConfigFormat::Toml
                .parse("at = 2026-01-02T03:04:05Z")
                .unwrap(),
            json!({"at": "2026-01-02T03:04:05Z"})
        );
    }

    #[test]
    fn parse_errors_are_reported() {
        assert!(ConfigFormat::Json.parse("{").is_err());
        assert!(ConfigFormat::Yaml.parse("a: [").is_err());
        assert!(ConfigFormat::Toml.parse("a = ").is_err());
    }
}
