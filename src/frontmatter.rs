/// Frontmatter
///
///
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Frontmatter
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct Frontmatter {
    //
    pub title: Option<String>,
    //
    pub tags: Option<Vec<String>>,
    //
    pub template: Option<String>,
    //
    pub created_at: Option<DateTime<Utc>>,
    //
    pub updated_at: Option<DateTime<Utc>>,
}

// Splits the incoming bytes into a Frontmatter object and the leftover bytes.
//
pub fn extract(content: String) -> (Option<Frontmatter>, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^---\n((?s).*?)\n---\n((?s).*)").unwrap();
    }
    if let Some(captures) = RE.captures(&content) {
        let yaml_str = captures.get(1).map_or("", |m| m.as_str());
        let fm: Option<Frontmatter> = serde_yaml::from_str(yaml_str).unwrap_or(None);
        let trimmed = captures.get(2).map_or("", |m| m.as_str());
        return (fm, trimmed.to_string());
    }
    (Some(Frontmatter::default()), content)
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_handles_yaml() {
        let bytes = "\
---
tags:
- some-tag
template: main
unsupported_key:
- with unsupported values
---
# Here comes the markdown!"
            .to_string();

        let (fm, new_bytes) = super::extract(bytes);

        assert_eq!(
            fm.unwrap(),
            super::Frontmatter {
                tags: Some(vec!["some-tag".to_string()]),
                title: None,
                template: Some("main".to_string()),
                created_at: None,
                updated_at: None,
            }
        );
        assert_eq!(new_bytes, "# Here comes the markdown!");
    }

    #[test]
    fn extract_handles_no_frontmatter() {
        let bytes = "\
# Here comes the markdown!"
            .to_string();

        let (fm, new_bytes) = super::extract(bytes);

        assert_eq!(fm.unwrap(), super::Frontmatter::default());
        assert_eq!(new_bytes, "# Here comes the markdown!");
    }
}
