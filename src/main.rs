use std::fs;
use anyhow::{bail, Result};
use serde_json::Value;

fn main() {
    let data = fs::read_to_string("content/about.md").unwrap();
    let hb = parse_header_body(&data).unwrap();
    let frontmatter: Value = serde_json::from_str(hb.0).unwrap();
    println!("{:?}", frontmatter);
}

const FRONTMATTER_PATTERN: &str = "---";

fn parse_header_body(content: &str) -> Result<(&str, &str)> {
    if !content.starts_with(FRONTMATTER_PATTERN) {
       Ok(("", content)) // no frontmatter
    } else {
        let indices: Vec<_> = content.match_indices(FRONTMATTER_PATTERN).take(2).collect();
        if indices.len() < 2  {
            bail!("invalid frontmatter, expected matching '{}' for content {}", FRONTMATTER_PATTERN, content)
        }

        let offset = FRONTMATTER_PATTERN.len();
        Ok((&content[(indices[0].0+offset)..indices[1].0], &content[(indices[1].0+offset)..]))
    }
}
