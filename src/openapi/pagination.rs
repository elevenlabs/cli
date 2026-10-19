//! `--page-all` continuation when an operation has no `x-fern-pagination`.
//!
//! The executor's document-wide fallback looks for `nextPageToken` and sends
//! it back as `pageToken`. Specs that paginate with other field names never
//! match that pair, so the loop stops after the first response and the
//! process still exits 0. This module reads the operation's query parameters
//! and response schema and picks the pair the API actually documents.

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::error::CliError;
use crate::openapi::discovery::{JsonSchema, JsonSchemaProperty, MethodParameter, RestMethod};

/// How to request the page after the one just received.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferredPagination {
    /// Copy `response_path` into the `query_param` query parameter.
    ///
    /// `has_more: false` ends the loop even when the token field is still
    /// populated. `has_more: true` with an empty token is an error: the
    /// server said the result is incomplete and gave us nothing to follow.
    Cursor {
        query_param: String,
        response_path: String,
    },
    /// Add one to the `query_param` page index.
    ///
    /// `default_page` is the page the server serves when the flag is omitted
    /// (the OpenAPI `default`). `default_page_size` is that same default for
    /// the `page_size` parameter, used only when the response has no
    /// `has_more` flag.
    Page {
        query_param: String,
        default_page: u64,
        default_page_size: Option<u64>,
    },
    /// Add the number of returned items to the `query_param` offset.
    Offset {
        query_param: String,
        results_path: String,
        default_page_size: u64,
    },
}

impl InferredPagination {
    pub fn query_param(&self) -> &str {
        match self {
            Self::Cursor { query_param, .. }
            | Self::Page { query_param, .. }
            | Self::Offset { query_param, .. } => query_param,
        }
    }
}

/// What the loop should do with the response it just parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageStep {
    Stop,
    /// Send this token as the cursor query parameter.
    Cursor(String),
    /// Send this page index on the next request.
    Page(u64),
    /// Send this offset on the next request.
    Offset(u64),
}

/// Values the previous iteration injected. `None` means this was the first
/// request, so a flag the user passed (if any) is the only copy of the
/// parameter on the query string.
#[derive(Debug, Clone, Copy, Default)]
pub struct RequestedPage {
    pub injected_page: Option<u64>,
    pub injected_offset: Option<u64>,
}

/// Request/response field pairs this CLI knows how to follow, most specific
/// first. Names are matched exactly so a similarly named field (an auth
/// `token`, a `page_size`) is not treated as a cursor.
const CURSOR_PAIRS: &[(&str, &str)] = &[
    ("next_page_token", "next_page_token"),
    ("cursor", "next_cursor"),
    ("start_after_history_item_id", "last_history_item_id"),
    ("last_doc", "next_doc"),
    ("page_token", "pagination.next_page_token"),
    ("page_token", "next_page_token"),
];

/// Classify one operation. `None` means `--page-all` keeps the document
/// heuristic (`nextPageToken` / `pageToken`) and does not guess.
pub fn infer_pagination(
    method: &RestMethod,
    schemas: &HashMap<String, JsonSchema>,
) -> Option<InferredPagination> {
    let response = response_schema(method, schemas)?;

    for &(query_name, response_path) in CURSOR_PAIRS {
        if query_param(method, query_name).is_some()
            && property_at_path(response, response_path, schemas).is_some()
        {
            return Some(InferredPagination::Cursor {
                query_param: query_name.to_string(),
                response_path: response_path.to_string(),
            });
        }
    }

    if query_param(method, "page").is_some()
        && (property_at_path(response, "has_more", schemas).is_some()
            || property_at_path(response, "meta.total", schemas).is_some()
            || property_at_path(response, "meta.page", schemas).is_some())
    {
        let default_page = query_param(method, "page")
            .and_then(param_default_u64)
            .unwrap_or(1);
        let default_page_size = query_param(method, "page_size").and_then(param_default_u64);
        return Some(InferredPagination::Page {
            query_param: "page".to_string(),
            default_page,
            default_page_size,
        });
    }

    if query_param(method, "offset").is_some() {
        if let Some(results_path) = sole_array_property(response, schemas) {
            let default_page_size = query_param(method, "page_size")
                .and_then(param_default_u64)
                .unwrap_or(0);
            if default_page_size > 0 || query_param(method, "page_size").is_some() {
                return Some(InferredPagination::Offset {
                    query_param: "offset".to_string(),
                    results_path,
                    default_page_size,
                });
            }
        }
    }

    None
}

/// Decide whether the response just parsed has another page.
pub fn next_step(
    inferred: &InferredPagination,
    body: &Value,
    requested: RequestedPage,
    query: &[(String, String)],
) -> Result<PageStep, CliError> {
    match inferred {
        InferredPagination::Cursor { response_path, .. } => cursor_step(body, response_path),
        InferredPagination::Page {
            query_param,
            default_page,
            default_page_size,
        } => page_step(
            body,
            query,
            query_param,
            *default_page,
            *default_page_size,
            requested.injected_page,
        ),
        InferredPagination::Offset {
            query_param,
            results_path,
            default_page_size,
        } => offset_step(
            body,
            query,
            query_param,
            results_path,
            *default_page_size,
            requested.injected_offset,
        ),
    }
}

fn cursor_step(body: &Value, response_path: &str) -> Result<PageStep, CliError> {
    let has_more = flag_has_more(body);
    if has_more == Some(false) {
        return Ok(PageStep::Stop);
    }
    match nested_str(body, response_path).filter(|token| !token.is_empty()) {
        Some(token) => Ok(PageStep::Cursor(token.to_string())),
        None if has_more == Some(true) => Err(CliError::Validation(format!(
            "response has has_more=true but no usable value at '{response_path}'"
        ))),
        None => Ok(PageStep::Stop),
    }
}

fn page_step(
    body: &Value,
    query: &[(String, String)],
    query_param: &str,
    default_page: u64,
    default_page_size: Option<u64>,
    injected_page: Option<u64>,
) -> Result<PageStep, CliError> {
    let current = injected_page
        .or_else(|| query_u64(query, query_param))
        .unwrap_or(default_page);
    let has_more = flag_has_more(body);
    if has_more == Some(false) || page_is_exhausted(body, query, current, default_page_size) {
        return Ok(PageStep::Stop);
    }
    // `has_more` was either true or absent. Absent only reaches here when
    // `page_is_exhausted` found a full page that does not cover `meta.total`.
    Ok(PageStep::Page(current.saturating_add(1)))
}

fn page_is_exhausted(
    body: &Value,
    query: &[(String, String)],
    current_page: u64,
    default_page_size: Option<u64>,
) -> bool {
    // An explicit flag wins over the length heuristic.
    if flag_has_more(body).is_some() {
        return false;
    }
    let len = array_len(body, "results")
        .or_else(|| array_len(body, "items"))
        .or_else(|| first_array_len(body));
    if len == Some(0) {
        return true;
    }
    let page_size = nested_u64(body, "meta.page_size")
        .or_else(|| query_u64(query, "page_size"))
        .or(default_page_size);
    let total = nested_u64(body, "meta.total");
    if let (Some(total), Some(page_size)) = (total, page_size.filter(|n| *n > 0)) {
        // Page indexes documented as starting at 1: page 2 of size 30 has
        // covered 60 items. A 0-based index would under-count; those
        // operations in this spec expose `has_more` and never get here.
        if current_page.saturating_mul(page_size) >= total {
            return true;
        }
    }
    if let (Some(len), Some(page_size)) = (len, page_size.filter(|n| *n > 0)) {
        return len < page_size as usize;
    }
    // No flag and no size to compare against: do not keep requesting.
    true
}

fn offset_step(
    body: &Value,
    query: &[(String, String)],
    query_param: &str,
    results_path: &str,
    default_page_size: u64,
    injected_offset: Option<u64>,
) -> Result<PageStep, CliError> {
    let len = array_len(body, results_path).unwrap_or(0);
    let page_size = query_u64(query, "page_size").unwrap_or(default_page_size);
    if len == 0 || (page_size > 0 && (len as u64) < page_size) {
        return Ok(PageStep::Stop);
    }
    let current = injected_offset
        .or_else(|| query_u64(query, query_param))
        .unwrap_or(0);
    Ok(PageStep::Offset(current.saturating_add(len as u64)))
}

fn flag_has_more(body: &Value) -> Option<bool> {
    nested_bool(body, "has_more").or_else(|| nested_bool(body, "pagination.has_more"))
}

fn response_schema<'a>(
    method: &'a RestMethod,
    schemas: &'a HashMap<String, JsonSchema>,
) -> Option<&'a JsonSchema> {
    let name = method.response.as_ref()?.schema_ref.as_deref()?;
    schemas.get(name)
}

fn query_param<'a>(method: &'a RestMethod, name: &str) -> Option<&'a MethodParameter> {
    method
        .parameters
        .get(name)
        .filter(|param| param.location.as_deref() == Some("query"))
}

fn param_default_u64(param: &MethodParameter) -> Option<u64> {
    param
        .documentation_default_value
        .as_ref()
        .and_then(number_u64)
}

fn query_u64(query: &[(String, String)], name: &str) -> Option<u64> {
    query
        .iter()
        .find(|(key, _)| key == name)
        .and_then(|(_, value)| value.parse().ok())
}

fn nested<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

fn nested_str<'a>(value: &'a Value, path: &str) -> Option<&'a str> {
    nested(value, path)?.as_str()
}

fn nested_bool(value: &Value, path: &str) -> Option<bool> {
    nested(value, path)?.as_bool()
}

fn nested_u64(value: &Value, path: &str) -> Option<u64> {
    number_u64(nested(value, path)?)
}

fn number_u64(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_i64().and_then(|n| u64::try_from(n).ok()))
        .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
}

fn array_len(value: &Value, path: &str) -> Option<usize> {
    nested(value, path)?.as_array().map(|items| items.len())
}

fn first_array_len(value: &Value) -> Option<usize> {
    value
        .as_object()?
        .values()
        .find_map(|child| child.as_array().map(|items| items.len()))
}

fn sole_array_property(
    schema: &JsonSchema,
    schemas: &HashMap<String, JsonSchema>,
) -> Option<String> {
    let mut names = Vec::new();
    collect_array_names(schema, schemas, &mut HashSet::new(), &mut names);
    names.sort();
    names.dedup();
    const PREFERRED: &[&str] = &["orders", "items", "results", "data"];
    for preferred in PREFERRED {
        if names.iter().any(|name| name == preferred) {
            return Some((*preferred).to_string());
        }
    }
    if names.len() == 1 {
        names.pop()
    } else {
        None
    }
}

fn collect_array_names(
    schema: &JsonSchema,
    schemas: &HashMap<String, JsonSchema>,
    seen: &mut HashSet<String>,
    out: &mut Vec<String>,
) {
    let schema = follow_schema(schema, schemas, seen);
    for (name, property) in &schema.properties {
        if is_array(property, schemas) {
            out.push(name.clone());
        }
    }
    for branch in schema
        .all_of
        .iter()
        .chain(schema.any_of.iter())
        .chain(schema.one_of.iter())
    {
        collect_array_names_in_property(branch, schemas, seen, out);
    }
}

fn collect_array_names_in_property(
    property: &JsonSchemaProperty,
    schemas: &HashMap<String, JsonSchema>,
    seen: &mut HashSet<String>,
    out: &mut Vec<String>,
) {
    if let Some(name) = property.schema_ref.as_deref() {
        if let Some(schema) = schemas.get(name) {
            collect_array_names(schema, schemas, seen, out);
        }
    }
    for (name, child) in &property.properties {
        if is_array(child, schemas) {
            out.push(name.clone());
        }
    }
    for branch in property
        .all_of
        .iter()
        .chain(property.any_of.iter())
        .chain(property.one_of.iter())
    {
        collect_array_names_in_property(branch, schemas, seen, out);
    }
}

fn is_array(property: &JsonSchemaProperty, schemas: &HashMap<String, JsonSchema>) -> bool {
    if property.prop_type.as_deref() == Some("array") || property.items.is_some() {
        return true;
    }
    let Some(name) = property.schema_ref.as_deref() else {
        return false;
    };
    schemas.get(name).is_some_and(|schema| {
        schema.schema_type.as_deref() == Some("array") || schema.items.is_some()
    })
}

fn property_at_path<'a>(
    schema: &'a JsonSchema,
    path: &str,
    schemas: &'a HashMap<String, JsonSchema>,
) -> Option<&'a JsonSchemaProperty> {
    let mut seen = HashSet::new();
    let mut current_schema = Some(schema);
    let mut current_property: Option<&JsonSchemaProperty> = None;
    for (index, segment) in path.split('.').enumerate() {
        let property = if index == 0 {
            let schema = current_schema?;
            find_property(schema, segment, schemas, &mut seen)?
        } else {
            find_property_on(current_property?, segment, schemas, &mut seen)?
        };
        current_property = Some(property);
        current_schema = property
            .schema_ref
            .as_deref()
            .and_then(|name| schemas.get(name));
    }
    current_property
}

fn find_property<'a>(
    schema: &'a JsonSchema,
    name: &str,
    schemas: &'a HashMap<String, JsonSchema>,
    seen: &mut HashSet<String>,
) -> Option<&'a JsonSchemaProperty> {
    let schema = follow_schema(schema, schemas, seen);
    if let Some(property) = schema.properties.get(name) {
        return Some(property);
    }
    for branch in schema
        .all_of
        .iter()
        .chain(schema.any_of.iter())
        .chain(schema.one_of.iter())
    {
        if let Some(property) = find_property_on(branch, name, schemas, seen) {
            return Some(property);
        }
    }
    None
}

fn find_property_on<'a>(
    property: &'a JsonSchemaProperty,
    name: &str,
    schemas: &'a HashMap<String, JsonSchema>,
    seen: &mut HashSet<String>,
) -> Option<&'a JsonSchemaProperty> {
    if property.properties.contains_key(name) {
        return property.properties.get(name);
    }
    if let Some(schema_name) = property.schema_ref.as_deref() {
        if let Some(schema) = schemas.get(schema_name) {
            if let Some(found) = find_property(schema, name, schemas, seen) {
                return Some(found);
            }
        }
    }
    for branch in property
        .all_of
        .iter()
        .chain(property.any_of.iter())
        .chain(property.one_of.iter())
    {
        if let Some(found) = find_property_on(branch, name, schemas, seen) {
            return Some(found);
        }
    }
    None
}

fn follow_schema<'a>(
    schema: &'a JsonSchema,
    schemas: &'a HashMap<String, JsonSchema>,
    seen: &mut HashSet<String>,
) -> &'a JsonSchema {
    let mut current = schema;
    while let Some(name) = current.schema_ref.as_deref() {
        if !seen.insert(name.to_string()) {
            break;
        }
        match schemas.get(name) {
            Some(next) => current = next,
            None => break,
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cursor() -> InferredPagination {
        InferredPagination::Cursor {
            query_param: "cursor".into(),
            response_path: "next_cursor".into(),
        }
    }

    #[test]
    fn cursor_follows_a_token_until_has_more_is_false() {
        let step = next_step(
            &cursor(),
            &json!({"next_cursor": "page-2", "has_more": true}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Cursor("page-2".into()));

        let step = next_step(
            &cursor(),
            &json!({"next_cursor": "stale", "has_more": false}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Stop);
    }

    #[test]
    fn cursor_without_has_more_follows_a_non_empty_token() {
        let step = next_step(
            &cursor(),
            &json!({"next_cursor": "page-2"}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Cursor("page-2".into()));

        let step = next_step(
            &cursor(),
            &json!({"next_cursor": null}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Stop);
    }

    #[test]
    fn cursor_errors_when_has_more_has_no_token() {
        let err = next_step(
            &cursor(),
            &json!({"next_cursor": "", "has_more": true}),
            RequestedPage::default(),
            &[],
        )
        .unwrap_err();
        let message = err.to_string();
        assert!(message.contains("has_more=true"), "{message}");
        assert!(message.contains("next_cursor"), "{message}");
    }

    #[test]
    fn page_increments_from_the_server_default_while_has_more() {
        let inferred = InferredPagination::Page {
            query_param: "page".into(),
            default_page: 0,
            default_page_size: Some(30),
        };
        let step = next_step(
            &inferred,
            &json!({"voices": [1], "has_more": true}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Page(1));

        let step = next_step(
            &inferred,
            &json!({"voices": [1], "has_more": false}),
            RequestedPage {
                injected_page: Some(1),
                injected_offset: None,
            },
            &[],
        )
        .unwrap();
        assert_eq!(step, PageStep::Stop);
    }

    #[test]
    fn page_stops_when_meta_total_is_covered() {
        let inferred = InferredPagination::Page {
            query_param: "page".into(),
            default_page: 1,
            default_page_size: Some(30),
        };
        let first = next_step(
            &inferred,
            &json!({"results": [{"id": "a"}], "meta": {"total": 2, "page": 1, "page_size": 1}}),
            RequestedPage::default(),
            &[],
        )
        .unwrap();
        assert_eq!(first, PageStep::Page(2));

        let second = next_step(
            &inferred,
            &json!({"results": [{"id": "b"}], "meta": {"total": 2, "page": 2, "page_size": 1}}),
            RequestedPage {
                injected_page: Some(2),
                injected_offset: None,
            },
            &[],
        )
        .unwrap();
        assert_eq!(second, PageStep::Stop);
    }

    #[test]
    fn offset_advances_by_the_page_length_and_stops_on_a_short_page() {
        let inferred = InferredPagination::Offset {
            query_param: "offset".into(),
            results_path: "orders".into(),
            default_page_size: 20,
        };
        let query = [("page_size".into(), "2".into())];
        let step = next_step(
            &inferred,
            &json!({"orders": [{"id": 1}, {"id": 2}]}),
            RequestedPage::default(),
            &query,
        )
        .unwrap();
        assert_eq!(step, PageStep::Offset(2));

        let step = next_step(
            &inferred,
            &json!({"orders": [{"id": 3}]}),
            RequestedPage {
                injected_page: None,
                injected_offset: Some(2),
            },
            &query,
        )
        .unwrap();
        assert_eq!(step, PageStep::Stop);
    }

    #[test]
    fn every_paginated_operation_in_the_spec_is_inferred() {
        let raw = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/cli/elevenlabs/openapi0.json"
        ))
        .expect("openapi spec");
        let doc = super::super::parser::load_openapi_spec(&raw, "elevenlabs").expect("parse spec");
        let mut lines = Vec::new();
        walk(&doc.resources, &doc.schemas, &mut lines);
        lines.sort();
        lines.dedup();
        assert_eq!(
            lines, EXPECTED_PAGINATED_OPERATIONS,
            "a paginated operation was not classified, or a non-paginated one was"
        );
    }

    fn walk(
        resources: &HashMap<String, crate::openapi::discovery::RestResource>,
        schemas: &HashMap<String, JsonSchema>,
        out: &mut Vec<String>,
    ) {
        for resource in resources.values() {
            for method in resource.methods.values() {
                if let Some(inferred) = infer_pagination(method, schemas) {
                    out.push(format!(
                        "{} {} {}",
                        method.http_method,
                        method.path,
                        describe(&inferred)
                    ));
                }
            }
            walk(&resource.resources, schemas, out);
        }
    }

    fn describe(inferred: &InferredPagination) -> String {
        match inferred {
            InferredPagination::Cursor {
                query_param,
                response_path,
            } => format!("cursor {query_param} <- {response_path}"),
            InferredPagination::Page {
                default_page,
                default_page_size,
                ..
            } => format!("page default={default_page} size={default_page_size:?}"),
            InferredPagination::Offset {
                results_path,
                default_page_size,
                ..
            } => format!("offset {results_path} size={default_page_size}"),
        }
    }

    /// Every list operation in `cli/elevenlabs/openapi0.json` that takes a
    /// pagination query parameter. `token` on speech-to-text is an auth
    /// token, and `limit` alone (batch-call export, agent branches) caps a
    /// single response rather than selecting a page.
    const EXPECTED_PAGINATED_OPERATIONS: &[&str] = &[
        "GET /v1/assets cursor cursor <- next_cursor",
        "GET /v1/audio-isolation/history page default=1 size=Some(100)",
        "GET /v1/convai/agent-testing cursor cursor <- next_cursor",
        "GET /v1/convai/agents cursor cursor <- next_cursor",
        "GET /v1/convai/agents/{agent_id}/deployments page default=1 size=Some(30)",
        "GET /v1/convai/agents/{agent_id}/merge-proposals cursor cursor <- next_cursor",
        "GET /v1/convai/agents/{agent_id}/topics cursor cursor <- next_cursor",
        "GET /v1/convai/agents/{agent_id}/triage-tickets cursor cursor <- next_cursor",
        "GET /v1/convai/batch-calling/workspace cursor last_doc <- next_doc",
        "GET /v1/convai/conversations cursor cursor <- next_cursor",
        "GET /v1/convai/conversations/messages/smart-search cursor cursor <- next_cursor",
        "GET /v1/convai/conversations/messages/text-search cursor cursor <- next_cursor",
        "GET /v1/convai/conversations/{conversation_id}/sip-messages cursor cursor <- next_cursor",
        "GET /v1/convai/environment-variables cursor cursor <- next_cursor",
        "GET /v1/convai/knowledge-base cursor cursor <- next_cursor",
        "GET /v1/convai/knowledge-base/crawl cursor cursor <- next_cursor",
        "GET /v1/convai/knowledge-base/search cursor cursor <- next_cursor",
        "GET /v1/convai/knowledge-base/{documentation_id}/chunks cursor cursor <- next_cursor",
        "GET /v1/convai/knowledge-base/{documentation_id}/dependent-agents cursor cursor <- next_cursor",
        "GET /v1/convai/phone-numbers/{phone_number_id}/sip-messages cursor cursor <- next_cursor",
        "GET /v1/convai/secrets cursor cursor <- next_cursor",
        "GET /v1/convai/secrets/{secret_id}/dependencies/{resource_type} cursor cursor <- next_cursor",
        "GET /v1/convai/tags cursor cursor <- next_cursor",
        "GET /v1/convai/test-invocations cursor cursor <- next_cursor",
        "GET /v1/convai/tools cursor cursor <- next_cursor",
        "GET /v1/convai/tools/{tool_id}/dependent-agents cursor cursor <- next_cursor",
        "GET /v1/convai/tools/{tool_id}/executions cursor cursor <- next_cursor",
        "GET /v1/convai/triage-tickets cursor cursor <- next_cursor",
        "GET /v1/convai/users cursor cursor <- next_cursor",
        "GET /v1/convai/v2/phone-numbers cursor cursor <- next_cursor",
        "GET /v1/dubbing cursor cursor <- next_cursor",
        "GET /v1/dubbing/project cursor cursor <- next_cursor",
        "GET /v1/dubbing/project/{project_id}/language cursor cursor <- next_cursor",
        "GET /v1/flows/image cursor cursor <- next_cursor",
        "GET /v1/flows/templates cursor cursor <- next_cursor",
        "GET /v1/flows/templates/{template_id}/runs cursor cursor <- next_cursor",
        "GET /v1/flows/text-to-speech cursor cursor <- next_cursor",
        "GET /v1/flows/video cursor cursor <- next_cursor",
        "GET /v1/history cursor start_after_history_item_id <- last_history_item_id",
        "GET /v1/music/finetunes cursor cursor <- next_cursor",
        "GET /v1/productions/orders offset orders size=20",
        "GET /v1/pronunciation-dictionaries cursor cursor <- next_cursor",
        "GET /v1/shared-voices page default=0 size=Some(30)",
        "GET /v1/speech-engine cursor cursor <- next_cursor",
        "GET /v1/workspace/audit-logs cursor cursor <- next_cursor",
        "GET /v2/voices cursor next_page_token <- next_page_token",
        "POST /v1/convai/knowledge-base/dependent-agents cursor cursor <- next_cursor",
    ];
}
