use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use super::note::Note;

#[derive(Debug, Clone)]
pub struct VaultIndex {
    pub root: PathBuf,
    pub name: String,
    pub notes: HashMap<PathBuf, Note>,
    // Map from lowercase note stem/name to full path
    pub title_to_path: HashMap<String, PathBuf>,
    // Map from target note path to set of source note paths linking to it
    pub backlinks: HashMap<PathBuf, HashSet<PathBuf>>,
    // Map from tag to set of note paths
    pub tags: HashMap<String, HashSet<PathBuf>>,
    // List of relative markdown files for fast iteration/sorting
    pub files: Vec<PathBuf>,
}

impl VaultIndex {
    pub fn build(root: PathBuf, name: String) -> Self {
        let mut notes = HashMap::new();
        let mut title_to_path = HashMap::new();
        let mut files = Vec::new();
        let mut tags: HashMap<String, HashSet<PathBuf>> = HashMap::new();

        // 1. Traverse and index all markdown files
        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Skip hidden folders like .obsidian, .git, and Trash
                !name.starts_with('.') && name != "Trash" && name != "node_modules"
            })
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(note) = Note::load(&root, path) {
                    let stem_lower = note.title.to_lowercase();
                    title_to_path.insert(stem_lower, path.to_path_buf());

                    for tag in &note.tags {
                        tags.entry(tag.clone())
                            .or_default()
                            .insert(path.to_path_buf());
                    }

                    files.push(path.to_path_buf());
                    notes.insert(path.to_path_buf(), note);
                }
            }
        }

        // Sort files alphabetically by relative path
        files.sort();

        // 2. Build backlinks
        let mut backlinks: HashMap<PathBuf, HashSet<PathBuf>> = HashMap::new();

        for (src_path, note) in &notes {
            for link in &note.outgoing_links {
                let target_lower = link.target.trim().to_lowercase();
                // Remove trailing .md if present
                let target_stem = target_lower.strip_suffix(".md").unwrap_or(&target_lower);

                if let Some(target_path) = title_to_path.get(target_stem) {
                    backlinks
                        .entry(target_path.clone())
                        .or_default()
                        .insert(src_path.clone());
                }
            }
        }

        Self {
            root,
            name,
            notes,
            title_to_path,
            backlinks,
            tags,
            files,
        }
    }

    /// Resolve a wiki-link target string (e.g. "Weekly Notes" or "Weekly Notes/2026-W28") to a note path
    pub fn resolve_wikilink(&self, target: &str) -> Option<&PathBuf> {
        let clean = target.trim().to_lowercase();
        let stem = clean.strip_suffix(".md").unwrap_or(&clean);

        // 1. Try exact stem match
        if let Some(path) = self.title_to_path.get(stem) {
            return Some(path);
        }

        // 2. Try match by relative path ending
        for path in &self.files {
            if let Ok(rel) = path.strip_prefix(&self.root) {
                let rel_str = rel.to_string_lossy().to_lowercase();
                if rel_str == clean || rel_str == format!("{}.md", clean) || rel_str.ends_with(&format!("/{}", clean)) || rel_str.ends_with(&format!("/{}.md", clean)) {
                    return Some(path);
                }
            }
        }

        // 3. Fallback: Substring match on title stem
        for (stem_key, path) in &self.title_to_path {
            if stem_key.contains(stem) {
                return Some(path);
            }
        }

        None
    }

    pub fn get_backlinks(&self, note_path: &Path) -> Vec<&Note> {
        if let Some(sources) = self.backlinks.get(note_path) {
            let mut list: Vec<&Note> = sources
                .iter()
                .filter_map(|p| self.notes.get(p))
                .collect();
            list.sort_by(|a, b| a.title.cmp(&b.title));
            list
        } else {
            Vec::new()
        }
    }
}
