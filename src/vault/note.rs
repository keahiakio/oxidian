use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static WIKILINK_REGEX: OnceLock<Regex> = OnceLock::new();
static TAG_REGEX: OnceLock<Regex> = OnceLock::new();
static CALLOUT_REGEX: OnceLock<Regex> = OnceLock::new();

fn wikilink_re() -> &'static Regex {
    WIKILINK_REGEX.get_or_init(|| Regex::new(r"\[\[([^\]|#]+)(?:#[^\]|]+)?(?:\|([^\]]+))?\]\]").unwrap())
}

fn tag_re() -> &'static Regex {
    TAG_REGEX.get_or_init(|| Regex::new(r"(?:^|\s)#([a-zA-Z0-9_\-\/]+)").unwrap())
}

fn callout_re() -> &'static Regex {
    CALLOUT_REGEX.get_or_init(|| Regex::new(r"^>\s*\[!([a-zA-Z]+)\]\s*(.*)$").unwrap())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineKind {
    Heading(u8, String),       // level 1..6, text
    Task(bool, String),        // is_done, text
    CalloutHeader(String, String), // type (NOTE, TIP, etc), optional title
    Blockquote(String),
    CodeFence(String),         // language
    CodeLine(String),
    Bullet(u8, String),        // indent level, text
    Numbered(u8, String, String), // indent level, num string, text
    Divider,
    Normal(String),
}

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub target: String,
    pub alias: Option<String>,
    pub line_idx: usize,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub rel_path: PathBuf,
    pub title: String,
    pub raw_content: String,
    pub lines: Vec<LineKind>,
    pub outgoing_links: Vec<WikiLink>,
    pub tags: HashSet<String>,
    pub word_count: usize,
    pub line_count: usize,
}

impl Note {
    pub fn load(vault_root: &Path, file_path: &Path) -> Result<Self, std::io::Error> {
        let raw_content = fs::read_to_string(file_path)?;
        let rel_path = file_path
            .strip_prefix(vault_root)
            .unwrap_or(file_path)
            .to_path_buf();

        let title = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let mut outgoing_links = Vec::new();
        let mut tags = HashSet::new();
        let mut parsed_lines = Vec::new();

        let mut in_code_block = false;
        let mut in_frontmatter = false;

        let wl_re = wikilink_re();
        let tg_re = tag_re();
        let co_re = callout_re();

        for (idx, line) in raw_content.lines().enumerate() {
            // Frontmatter check
            if idx == 0 && line.trim() == "---" {
                in_frontmatter = true;
                parsed_lines.push(LineKind::Divider);
                continue;
            } else if in_frontmatter {
                if line.trim() == "---" {
                    in_frontmatter = false;
                    parsed_lines.push(LineKind::Divider);
                    continue;
                }
                // Parse tags in frontmatter: e.g. tags: [foo, bar] or tags: foo
                let trimmed = line.trim();
                if trimmed.starts_with("tags:") || trimmed.starts_with("tag:") {
                    let parts = trimmed.splitn(2, ':').collect::<Vec<_>>();
                    if parts.len() == 2 {
                        let clean = parts[1].trim().trim_matches(|c| c == '[' || c == ']');
                        for t in clean.split(',') {
                            let tag = t.trim().trim_matches(|c| c == '"' || c == '\'');
                            if !tag.is_empty() {
                                tags.insert(tag.to_string());
                            }
                        }
                    }
                }
                parsed_lines.push(LineKind::Normal(line.to_string()));
                continue;
            }

            // Code block fence check
            if line.trim_start().starts_with("```") {
                let trimmed = line.trim();
                if in_code_block {
                    in_code_block = false;
                    parsed_lines.push(LineKind::CodeFence("".to_string()));
                } else {
                    in_code_block = true;
                    let lang = trimmed.trim_start_matches('`').trim().to_string();
                    parsed_lines.push(LineKind::CodeFence(lang));
                }
                continue;
            }

            if in_code_block {
                parsed_lines.push(LineKind::CodeLine(line.to_string()));
                continue;
            }

            // Extract wikilinks from line
            for cap in wl_re.captures_iter(line) {
                if let Some(target) = cap.get(1) {
                    let target_str = target.as_str().trim().to_string();
                    let alias = cap.get(2).map(|a| a.as_str().trim().to_string());
                    outgoing_links.push(WikiLink {
                        target: target_str,
                        alias,
                        line_idx: idx,
                    });
                }
            }

            // Extract inline tags
            for cap in tg_re.captures_iter(line) {
                if let Some(tag_match) = cap.get(1) {
                    tags.insert(tag_match.as_str().to_string());
                }
            }

            // Line classification for rendering
            let trimmed = line.trim_start();
            let indent = (line.len() - trimmed.len()) as u8 / 2;

            if let Some(co) = co_re.captures(line) {
                let kind = co.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_default();
                let title = co.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
                parsed_lines.push(LineKind::CalloutHeader(kind, title));
            } else if trimmed.starts_with("# ") {
                parsed_lines.push(LineKind::Heading(1, trimmed[2..].to_string()));
            } else if trimmed.starts_with("## ") {
                parsed_lines.push(LineKind::Heading(2, trimmed[3..].to_string()));
            } else if trimmed.starts_with("### ") {
                parsed_lines.push(LineKind::Heading(3, trimmed[4..].to_string()));
            } else if trimmed.starts_with("#### ") {
                parsed_lines.push(LineKind::Heading(4, trimmed[5..].to_string()));
            } else if trimmed.starts_with("##### ") {
                parsed_lines.push(LineKind::Heading(5, trimmed[6..].to_string()));
            } else if trimmed.starts_with("###### ") {
                parsed_lines.push(LineKind::Heading(6, trimmed[7..].to_string()));
            } else if trimmed.starts_with("- [ ] ") || trimmed.starts_with("* [ ] ") {
                parsed_lines.push(LineKind::Task(false, trimmed[6..].to_string()));
            } else if trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ") || trimmed.starts_with("* [x] ") {
                parsed_lines.push(LineKind::Task(true, trimmed[6..].to_string()));
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                parsed_lines.push(LineKind::Bullet(indent, trimmed[2..].to_string()));
            } else if trimmed.starts_with("> ") {
                parsed_lines.push(LineKind::Blockquote(trimmed[2..].to_string()));
            } else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
                parsed_lines.push(LineKind::Divider);
            } else {
                parsed_lines.push(LineKind::Normal(line.to_string()));
            }
        }

        let words: usize = raw_content.split_whitespace().count();
        let line_count = raw_content.lines().count();

        Ok(Self {
            path: file_path.to_path_buf(),
            rel_path,
            title,
            raw_content,
            lines: parsed_lines,
            outgoing_links,
            tags,
            word_count: words,
            line_count,
        })
    }
}
