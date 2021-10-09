use crate::content::{get_article, get_asset};
use crate::content::{Article, Content};
use regex::{Regex, RegexSet};
use std::collections::HashMap;

lazy_static! {
  static ref SET: RegexSet = RegexSet::new(&[
      // Image links
      r"!\[\[[\w\s/\.-_]+?\]\]",
      // Aliased links
      r"[^!]\[\[[\w\s/\.-_]+?\|[\w\s/\.-_]+?\]\]",
      // Normal links
      r"[^!]\[\[[\w\s/\.-_]+\]\]",
    ])
    .unwrap();

  static ref IMAGE: Regex = Regex::new(r"!\[\[([\w\s/\.\-_]+?)\]\]").unwrap();
  static ref ALIAS: Regex = Regex::new(r"[^!]\[\[([\w\s/\.\-_]+?)\|([\w\s/\.\-_]+?)\]\]").unwrap();
  static ref NORMAL: Regex = Regex::new(r"[^!]\[\[([\w\s/\.\-_]+?)\]\]").unwrap();
}

pub fn replace(
  contents: &HashMap<std::string::String, Content>,
  article: &Article,
) -> Result<Article, crate::error::Error> {
  let matches: Vec<_> = SET.matches(&article.content).into_iter().collect();
  let mut content = article.content.clone();

  // If there are image matches, replace them.
  if matches.contains(&0) {
    IMAGE.captures_iter(&article.content).for_each(|cap| {
      if let Some(asset) = get_asset(contents, &cap[1]) {
        content = content.replace(
          &format!("![[{}]]", &cap[1]),
          &format!("![Image](/{})", asset.permalink),
        );
      }
    });
  }

  // Replace aliased links
  if matches.contains(&1) {
    ALIAS.captures_iter(&article.content).for_each(|cap| {
      if let Some(article) = get_article(contents, &cap[1]) {
        content = content.replace(
          &format!("[[{}|{}]]", &cap[1], &cap[2]),
          &format!("[{}](/{})", &cap[2], article.permalink),
        );
      }
    });
  }

  // Replace normal links
  if matches.contains(&2) {
    NORMAL.captures_iter(&article.content).for_each(|cap| {
      if let Some(article) = get_article(contents, &cap[1]) {
        content = content.replace(
          &format!("[[{}]]", &cap[1]),
          &format!("[{}](/{})", &cap[1], article.permalink),
        );
      }
    });
  }

  let mut article = article.clone();
  article.content = content;

  Ok(article)
}

pub fn extract(contents: &HashMap<String, Content>, article: &Article) -> Vec<String> {
  let matches: Vec<_> = SET.matches(&article.content).into_iter().collect();
  let mut links = vec![];

  // If there are image matches, replace them.
  if matches.contains(&0) {
    IMAGE.captures_iter(&article.content).for_each(|cap| {
      if get_asset(contents, &cap[1]).is_some() {
        links.push(cap[1].to_string());
      }
    });
  }

  // Replace aliased links
  if matches.contains(&1) {
    ALIAS.captures_iter(&article.content).for_each(|cap| {
      if get_article(contents, &cap[1]).is_some() {
        links.push(cap[1].to_string());
      }
    });
  }

  // Replace normal links
  if matches.contains(&2) {
    NORMAL.captures_iter(&article.content).for_each(|cap| {
      if get_article(contents, &cap[1]).is_some() {
        links.push(cap[1].to_string());
      }
    });
  }
  links
}
