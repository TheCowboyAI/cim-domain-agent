// Copyright (c) 2025 - Cowboy AI, Inc.

//! Markdown section extraction
//!
//! Pure functional parsing of markdown structure using iterator chains

use super::error::{ParseError, ParseResult};
use std::collections::HashMap;

/// Parsed markdown sections
///
/// Product type: Contains all extracted sections
#[derive(Debug, Clone, PartialEq)]
pub struct MarkdownSections {
    pub sections: HashMap<String, String>,
}

/// Heading level (h1, h2, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeadingLevel(u8);

impl HeadingLevel {
    pub fn from_markdown(line: &str) -> Option<Self> {
        let trimmed = line.trim_start();
        let hash_count = trimmed.chars().take_while(|&c| c == '#').count();

        if hash_count > 0 && hash_count <= 6 {
            Some(Self(hash_count as u8))
        } else {
            None
        }
    }

    pub fn level(&self) -> u8 {
        self.0
    }
}

/// A single heading with its content
#[derive(Debug, Clone, PartialEq)]
struct Section {
    level: HeadingLevel,
    title: String,
    content: Vec<String>,
}

/// Extract heading title from markdown line
///
/// Pure function: deterministic string manipulation
fn extract_heading_title(line: &str) -> String {
    line.trim_start_matches('#')
        .trim()
        .to_string()
}

/// Parse markdown into sections by heading
///
/// Pure function: Iterator-based transformation (Axiom 5)
///
/// # Algorithm (Catamorphism - fold pattern)
///
/// ```text
/// Lines → fold(State, process_line) → Sections
/// ```
pub fn extract_sections(markdown: &str) -> ParseResult<MarkdownSections> {
    let lines: Vec<&str> = markdown.lines().collect();

    // Fold over lines, accumulating sections
    let sections = lines
        .iter()
        .fold(Vec::new(), |mut sections: Vec<Section>, &line| {
            if let Some(level) = HeadingLevel::from_markdown(line) {
                // New section starts
                let title = extract_heading_title(line);
                sections.push(Section {
                    level,
                    title,
                    content: Vec::new(),
                });
            } else if let Some(current) = sections.last_mut() {
                // Add content to current section
                current.content.push(line.to_string());
            }
            sections
        });

    // Transform Vec<Section> → HashMap<String, String>
    let map = sections
        .into_iter()
        .map(|section| {
            let content = section.content.join("\n").trim().to_string();
            (section.title, content)
        })
        .collect();

    Ok(MarkdownSections { sections: map })
}

/// Get a required section by name
///
/// Pure function: Option-based lookup
pub fn get_section<'a>(
    sections: &'a MarkdownSections,
    name: &str,
) -> Option<&'a String> {
    sections.sections.get(name)
}

/// Validate required sections exist
///
/// Pure function: Iterator-based validation
pub fn validate_sections(
    sections: &MarkdownSections,
    required: &[&str],
) -> ParseResult<()> {
    let missing: Vec<String> = required
        .iter()
        .filter(|&&name| !sections.sections.contains_key(name))
        .map(|&s| s.to_string())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(ParseError::MissingSection {
            section: missing.join(", "),
        })
    }
}

/// Extract all h1 headings
///
/// Pure function: filter transformation
pub fn extract_h1_titles(markdown: &str) -> Vec<String> {
    markdown
        .lines()
        .filter_map(|line| {
            HeadingLevel::from_markdown(line)
                .filter(|level| level.level() == 1)
                .map(|_| extract_heading_title(line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_MARKDOWN: &str = r#"
# Introduction

This is the introduction section.

## Subsection

Subsection content.

# Configuration

Config details here.

# Usage

Usage instructions.
"#;

    #[test]
    fn test_heading_level_detection() {
        assert_eq!(HeadingLevel::from_markdown("# Title"), Some(HeadingLevel(1)));
        assert_eq!(HeadingLevel::from_markdown("## Title"), Some(HeadingLevel(2)));
        assert_eq!(HeadingLevel::from_markdown("### Title"), Some(HeadingLevel(3)));
        assert_eq!(HeadingLevel::from_markdown("Not a heading"), None);
    }

    #[test]
    fn test_extract_heading_title() {
        assert_eq!(extract_heading_title("# Title"), "Title");
        assert_eq!(extract_heading_title("## Another Title"), "Another Title");
        assert_eq!(extract_heading_title("###   Spaced   "), "Spaced");
    }

    #[test]
    fn test_extract_sections() {
        let result = extract_sections(SAMPLE_MARKDOWN);
        assert!(result.is_ok());

        let sections = result.unwrap();
        assert!(sections.sections.contains_key("Introduction"));
        assert!(sections.sections.contains_key("Configuration"));
        assert!(sections.sections.contains_key("Usage"));
    }

    #[test]
    fn test_validate_sections_all_present() {
        let sections = extract_sections(SAMPLE_MARKDOWN).unwrap();
        let required = vec!["Introduction", "Configuration"];

        assert!(validate_sections(&sections, &required).is_ok());
    }

    #[test]
    fn test_validate_sections_missing() {
        let sections = extract_sections(SAMPLE_MARKDOWN).unwrap();
        let required = vec!["Introduction", "Missing Section"];

        let result = validate_sections(&sections, &required);
        assert!(matches!(result, Err(ParseError::MissingSection { .. })));
    }

    #[test]
    fn test_extract_h1_titles() {
        let titles = extract_h1_titles(SAMPLE_MARKDOWN);
        assert_eq!(titles.len(), 3);
        assert_eq!(titles[0], "Introduction");
        assert_eq!(titles[1], "Configuration");
        assert_eq!(titles[2], "Usage");
    }

    #[test]
    fn test_get_section() {
        let sections = extract_sections(SAMPLE_MARKDOWN).unwrap();

        assert!(get_section(&sections, "Introduction").is_some());
        assert!(get_section(&sections, "Nonexistent").is_none());
    }

    // Property test: Idempotence
    #[test]
    fn test_extract_sections_idempotent() {
        let result1 = extract_sections(SAMPLE_MARKDOWN);
        let result2 = extract_sections(SAMPLE_MARKDOWN);

        assert_eq!(result1, result2);
    }
}
