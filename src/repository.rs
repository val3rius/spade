use crate::links;
use crate::{Article, Asset, Content};
use std::collections::HashMap;

pub struct Repository {
  pub contents: HashMap<String, Content>,
  pub reference_cache: HashMap<String, Vec<String>>,
}

impl Repository {
  pub fn new(contents: HashMap<String, Content>) -> Self {
    let mut repo = Repository {
      contents,
      reference_cache: HashMap::new(),
    };
    for article in repo.contents.iter().filter_map(|(_k, c)| match c {
      Content::Article(a) => Some(a),
      _ => None,
    }) {
      let refs = links::extract(&repo, article);
      repo.reference_cache.insert(article.id.to_string(), refs);
    }
    repo
  }

  pub fn get_article(&self, id: &str) -> Option<&Article> {
    if let Some(Content::Article(article)) = self.contents.get(id) {
      return Some(article);
    }

    let maybe_prefixed_key = self.contents.keys().find(|key| key.ends_with(id));
    if let Some(key) = maybe_prefixed_key {
      if let Some(Content::Article(article)) = self.contents.get(key) {
        return Some(article);
      }
    }
    None
  }

  pub fn get_asset(&self, id: &str) -> Option<&Asset> {
    if let Some(Content::Asset(asset)) = self.contents.get(id) {
      return Some(asset);
    }

    let maybe_prefixed_key = self.contents.keys().find(|key| key.ends_with(id));
    if let Some(key) = maybe_prefixed_key {
      if let Some(Content::Asset(asset)) = self.contents.get(key) {
        return Some(asset);
      }
    }
    None
  }

  // Gets all inbound references, i.e. Articles that link to the provided one.
  pub fn get_inbound_references(&self, id: &str) -> Vec<String> {
    let mut inbound = vec![];
    for (key, value) in self.reference_cache.iter() {
      if value.contains(&id.to_string()) {
        inbound.push(key.to_owned());
      }
    }
    inbound
  }
}
