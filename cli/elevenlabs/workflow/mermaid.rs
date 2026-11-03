//! Shared Mermaid plumbing for `agents workflow diagram` / `export` and
//! `agents graph`: Mermaid-safe IDs and escaping, the node palette, and
//! drawing Mermaid source as terminal text with mmdflux, a pure-Rust Mermaid
//! renderer.

use std::collections::{BTreeMap, BTreeSet};
use std::io::IsTerminal;

use fern_cli_sdk::error::CliError;
use serde_json::Value;

pub(super) fn fence(diagram: &str) -> String {
    format!("```mermaid\n{diagram}```\n")
}

/// Longest node label, in characters, before it is cut with an ellipsis.
pub(super) const NODE_LABEL_MAX: usize = 60;
/// Longest edge label. LLM conditions are full sentences; the diagram only
/// needs enough to tell edges apart.
pub(super) const EDGE_LABEL_MAX: usize = 40;
/// Line width node labels are wrapped to in the terminal. Side-by-side nodes
/// are what make a diagram wide, so they are kept narrow.
pub(super) const TERMINAL_WRAP: usize = 24;

/// What `--format` asks a diagram command to print.
#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
pub(super) enum Format {
    /// Boxes and arrows drawn with box-drawing characters.
    Ascii,
    /// Mermaid source, which GitHub and most docs tools render.
    Mermaid,
    /// Mermaid source in a ```mermaid fence.
    MermaidMd,
}

/// Who the Mermaid source is for.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Target {
    /// mermaid.js (GitHub, docs): one-line labels, entity-escaped, with edge
    /// conditions inline.
    Export,
    /// mmdflux, drawing in a terminal: plain-text labels wrapped onto lines,
    /// and edge conditions replaced by numbered keys explained in a legend.
    /// Inline conditions are what make a terminal diagram too wide to read.
    Terminal,
}

/// How each node type is drawn: Mermaid shape delimiters and its class.
/// Class names are prefixed because `end` is a Mermaid keyword.
pub(super) struct Style {
    pub(super) open: &'static str,
    pub(super) close: &'static str,
    pub(super) class: &'static str,
}

/// `classDef` lines in emission order. Light fills with dark text read on
/// both GitHub's light and dark themes.
pub(super) const CLASS_DEFS: &[(&str, &str)] = &[
    ("t_start", "fill:#d1fae5,stroke:#059669,color:#064e3b"),
    ("t_end", "fill:#fee2e2,stroke:#dc2626,color:#7f1d1d"),
    ("t_say", "fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e"),
    ("t_tool", "fill:#fef3c7,stroke:#d97706,color:#78350f"),
    ("t_state", "fill:#ede9fe,stroke:#7c3aed,color:#4c1d95"),
    ("t_subagent", "fill:#f1f5f9,stroke:#475569,color:#0f172a"),
    ("t_transfer", "fill:#fce7f3,stroke:#db2777,color:#831843"),
    (
        "t_unknown",
        "fill:#f3f4f6,stroke:#6b7280,stroke-dasharray:4 2,color:#111827",
    ),
    // `agents graph`: knowledge-base documents.
    ("t_doc", "fill:#ecfccb,stroke:#65a30d,color:#365314"),
    // Terminal only: the numbered condition keys.
    ("t_key", "fill:#f3f4f6,stroke:#6b7280,color:#111827"),
];

/// A node's label: the kind of node (absent for start/end, which say it
/// themselves) and the text that identifies this one.
pub(super) struct NodeLabel {
    pub(super) kind: Option<String>,
    pub(super) text: String,
}

impl NodeLabel {
    /// The quoted label for `target`. Export puts it on one line, `kind ·
    /// text`; the terminal stacks the kind over the text, wrapped narrow.
    pub(super) fn render(&self, target: Target) -> String {
        match target {
            Target::Export => {
                let line = match &self.kind {
                    Some(kind) => format!("{kind} · {}", self.text),
                    None => self.text.clone(),
                };
                quote(&truncate(&line, NODE_LABEL_MAX))
            }
            Target::Terminal => {
                let mut lines: Vec<String> = self.kind.iter().cloned().collect();
                lines.extend(wrap(&truncate(&self.text, NODE_LABEL_MAX), TERMINAL_WRAP));
                let lines: Vec<String> = lines.iter().map(|l| plain(l)).collect();
                format!("\"{}\"", lines.join("<br/>"))
            }
        }
    }
}

pub(super) fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

/// Collapse whitespace and cut to `max` characters, ending in `…` when cut.
pub(super) fn truncate(text: &str, max: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max {
        return collapsed;
    }
    let cut: String = collapsed.chars().take(max - 1).collect();
    format!("{}…", cut.trim_end())
}

/// Quote label text for Mermaid, entity-encoding everything that would end
/// the string or be read as syntax. `#` goes first: it starts every entity.
pub(super) fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '#' => out.push_str("#35;"),
            '"' => out.push_str("#quot;"),
            '<' => out.push_str("#lt;"),
            '>' => out.push_str("#gt;"),
            '|' => out.push_str("#124;"),
            '`' => out.push_str("#96;"),
            '\n' | '\r' => out.push(' '),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Label text for mmdflux, which prints quoted text literally but does not
/// decode Mermaid's `#…;` entities. Only a quote (ends the string) and angle
/// brackets (read as tags, like the `<br/>` that wraps lines) need replacing,
/// and lookalikes keep the text readable.
pub(super) fn plain(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '"' => '\'',
            '<' => '‹',
            '>' => '›',
            '\n' | '\r' => ' ',
            c => c,
        })
        .collect()
}

/// Greedy word wrap to `width` characters. A word longer than the width
/// gets a line of its own rather than being split.
pub(super) fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// A node ID for a `%%` comment: one line, nothing else to escape.
pub(super) fn comment_safe(id: &str) -> String {
    id.replace(['\n', '\r'], " ")
}

/// Mermaid-safe node IDs.
///
/// Workflow and resource IDs are arbitrary strings; Mermaid IDs must be plain words and
/// must not be keywords (`end`) or start with `o`/`x` right after a link
/// (`A---oB` is a circle edge). Prefixing `n_` and replacing everything else
/// with `_` covers all of it; a numeric suffix separates IDs that collapse to
/// the same word. IDs are assigned in sorted order so they are stable.
pub(super) struct MermaidIds<'a> {
    map: BTreeMap<&'a str, String>,
}

impl<'a> MermaidIds<'a> {
    pub(super) fn new(ids: impl Iterator<Item = &'a str>) -> Self {
        let sorted: BTreeSet<&str> = ids.collect();
        let mut map = BTreeMap::new();
        let mut taken = BTreeSet::new();
        for id in sorted {
            let base: String = format!("n_{id}")
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            let mut candidate = base.clone();
            let mut n = 2;
            while !taken.insert(candidate.clone()) {
                candidate = format!("{base}_{n}");
                n += 1;
            }
            map.insert(id, candidate);
        }
        MermaidIds { map }
    }

    pub(super) fn get(&self, id: &str) -> &str {
        // Every id rendered was passed to `new`.
        self.map.get(id).map(String::as_str).unwrap_or("n_")
    }
}

/// One node's line: `id`, then `label` (already quoted) inside the shape's
/// delimiters.
///
/// mmdflux ends a round `( )` node at the first `)`, even inside the quoted
/// label, and then fails to parse the rest ("Say · Confirm (booking)", or
/// "Test (external)"). Other shapes are unaffected, and so is mermaid.js. So in
/// the terminal, a round node's brackets become same-width lookalikes.
pub(super) fn node_line(id: &str, open: &str, label: &str, close: &str, target: Target) -> String {
    let label = if target == Target::Terminal && open == "(" {
        label.replace('(', "❨").replace(')', "❩")
    } else {
        label.to_string()
    };
    format!("    {id}{open}{label}{close}\n")
}

/// Whether terminal drawings should be coloured: stdout is a terminal and
/// `NO_COLOR` is unset.
pub(super) fn use_color() -> bool {
    std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

/// Draw Mermaid source as box-drawing text. `color` adds ANSI colour from the
/// `classDef`s. `fallback` names the command that prints the source instead,
/// for the error message.
pub(super) fn draw_text(source: &str, color: bool, fallback: &str) -> Result<String, CliError> {
    let mut config = mmdflux::RenderConfig::default();
    if color {
        config.text_color_mode = mmdflux::TextColorMode::Ansi;
    }
    let mut out = mmdflux::render_diagram(source, mmdflux::OutputFormat::Text, &config)
        .map_err(|e| {
            CliError::Other(anyhow::anyhow!(
                "Could not draw this in the terminal ({e}). '{fallback}' prints it as Mermaid instead."
            ))
        })?;
    if !out.ends_with('\n') {
        out.push('\n');
    }
    Ok(out)
}

/// Append `classDef` and `class` lines for the classes in use, in
/// [`CLASS_DEFS`] order with sorted members, so output is stable.
pub(super) fn push_classes<S: AsRef<str>>(
    out: &mut String,
    classes: &BTreeMap<&'static str, Vec<S>>,
) {
    for (class, style) in CLASS_DEFS {
        if let Some(members) = classes.get(class) {
            out.push_str(&format!("    classDef {class} {style}\n"));
            let mut members: Vec<&str> = members.iter().map(AsRef::as_ref).collect();
            members.sort_unstable();
            out.push_str(&format!("    class {} {class}\n", members.join(",")));
        }
    }
}

/// The width to fit drawings to: `COLUMNS` when set (so scripts and CI can
/// ask for a width), otherwise the terminal's when stdout is one. `None`
/// means draw at full size.
pub(super) fn terminal_width() -> Option<usize> {
    let columns = std::env::var("COLUMNS").ok().and_then(|c| c.parse().ok());
    if let Some(columns) = columns.filter(|c| *c > 0) {
        return Some(columns);
    }
    if !std::io::stdout().is_terminal() {
        return None;
    }
    terminal_size::terminal_size_of(std::io::stdout()).map(|(w, _)| w.0 as usize)
}

/// The widest line of a drawing, in characters.
pub(super) fn text_width(text: &str) -> usize {
    text.lines().map(|l| l.chars().count()).max().unwrap_or(0)
}

/// Longest word kept whole when IDs are shortened. Names are shorter; IDs
/// such as `tool_01jxcrhpk0ehdvnxw3mw4sfb67` are longer.
pub(super) const SHORT_ID_MIN: usize = 16;

/// The first `head` and last `tail` characters, joined by `…`: `tool_01j…sfb67`.
/// Text that short already is returned whole.
pub(super) fn abbreviate(chars: &[char], head: usize, tail: usize) -> String {
    if head + tail + 1 >= chars.len() {
        return chars.iter().collect();
    }
    let start: String = chars[..head].iter().collect();
    let end: String = chars[chars.len() - tail..].iter().collect();
    format!("{start}…{end}")
}

/// Characters where lines cross (`┼`), or meet and share a stretch (`├`,
/// `┬`, ...). Boxes never use them, so each one is a tangle a reader has to
/// unpick to see what connects to what.
pub(super) const TANGLES: &[char] = &[
    '┼', '╋', '╂', '┿', '╪', '╫', '╬', '├', '┤', '┬', '┴', '┣', '┫', '┳', '┻', '┠', '┨', '┯', '┷',
    '┝', '┥', '┰', '┸',
];

/// How many tangles a drawing has (see [`TANGLES`]).
pub(super) fn tangles(text: &str) -> usize {
    text.chars().filter(|c| TANGLES.contains(c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_in_a_round_terminal_node_do_not_end_it() {
        for label in [
            r#""Say<br/>Confirm (booking)""#,
            r#""Test (external)<br/>t1""#,
            r#""a ) b""#,
        ] {
            let line = node_line("n_a", "(", label, ")", Target::Terminal);
            let source = format!("flowchart LR\n{line}");
            draw_text(&source, false, "export").unwrap_or_else(|e| panic!("{line}: {e:?}"));
        }
        // Other shapes, and export, keep the text as it is.
        assert_eq!(
            node_line("n_a", "[", r#""a (b)""#, "]", Target::Terminal),
            "    n_a[\"a (b)\"]\n"
        );
        assert_eq!(
            node_line("n_a", "(", r#""a (b)""#, ")", Target::Export),
            "    n_a(\"a (b)\")\n"
        );
    }

    #[test]
    fn wrap_keeps_long_words_whole() {
        assert_eq!(wrap("a bb ccc", 4), ["a bb", "ccc"]);
        assert_eq!(
            wrap("system__is_text_only x", 8),
            ["system__is_text_only", "x"]
        );
        assert!(wrap("   ", 8).is_empty());
    }
}
