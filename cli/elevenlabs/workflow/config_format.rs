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
//! alone. That needs `null` in a JSON or YAML config. A null array element is
//! data rather than an unset field, so a config with one can't be written as
//! TOML at all.

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
            Self::Yaml => serde_yaml::from_str(text).map_err(|e| e.to_string()),
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

// ── JSON → TOML ─────────────────────────────────────────────────────

fn json_to_toml(value: &Value) -> Result<String, String> {
    let Value::Object(map) = value else {
        return Err("A TOML config must be an object at the top level".to_string());
    };
    let mut doc = DocumentMut::new();
    fill_table(doc.as_table_mut(), map, "")?;
    Ok(doc.to_string())
}

/// `path` is the dotted key path of `map`, for error messages.
fn fill_table(table: &mut Table, map: &Map<String, Value>, path: &str) -> Result<(), String> {
    for (key, value) in map {
        if let Some(item) = to_item(value, &join(path, key))? {
            table.insert(key, item);
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

/// A null in an array is data (a constant value, say), not an unset field:
/// dropping it would change what `push` sends, so refuse instead.
fn null_in_array(path: &str) -> String {
    format!(
        "{path} holds a null in an array, which TOML can't represent. \
         Use a JSON or YAML config for this one"
    )
}

/// Objects become `[table]` sections and arrays of objects `[[array]]`
/// sections, which keeps a deeply nested agent config readable. `None` for
/// a null field, which TOML can't hold.
fn to_item(value: &Value, path: &str) -> Result<Option<Item>, String> {
    match value {
        Value::Object(map) => {
            let mut table = Table::new();
            fill_table(&mut table, map, path)?;
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
                fill_table(&mut table, map, &format!("{path}[{i}]"))?;
                tables.push(table);
            }
            Ok(Some(Item::ArrayOfTables(tables)))
        }
        other => Ok(to_value(other, path)?.map(Item::Value)),
    }
}

/// An inline TOML value: used for scalars, and for objects and arrays nested
/// inside an array that isn't a plain array of objects.
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
                match to_value(item, &format!("{path}[{i}]"))? {
                    Some(v) => array.push(v),
                    None => return Err(null_in_array(path)),
                }
            }
            array.into()
        }
        Value::Object(map) => {
            let mut table = InlineTable::new();
            for (key, item) in map {
                if let Some(v) = to_value(item, &join(path, key))? {
                    table.insert(key, v);
                }
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
        ] {
            let err = ConfigFormat::Toml.render(&value).unwrap_err().to_string();
            assert!(err.contains("Use a JSON or YAML config"), "{err}");
        }
        let err = ConfigFormat::Toml
            .render(&json!({"tool": {"constant_value": [1, null]}}))
            .unwrap_err()
            .to_string();
        assert!(err.starts_with("tool.constant_value holds a null"), "{err}");
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
