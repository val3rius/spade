use crate::{Article, Asset, Content};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn get_article<'a>(contents: &'a HashMap<String, Content>, id: &str) -> Option<&'a Article> {
  if let Some(Content::Article(article)) = contents.get(id) {
    return Some(article);
  }
  let maybe_prefixed_key = contents.keys().find(|key| key.ends_with(id));
  if let Some(key) = maybe_prefixed_key {
    if let Some(Content::Article(article)) = contents.get(key) {
      return Some(article);
    }
  }
  None
}

pub fn get_asset<'a>(contents: &'a HashMap<String, Content>, id: &str) -> Option<&'a Asset> {
  if let Some(Content::Asset(asset)) = contents.get(id) {
    return Some(asset);
  }
  let maybe_prefixed_key = contents.keys().find(|key| key.ends_with(id));
  if let Some(key) = maybe_prefixed_key {
    if let Some(Content::Asset(asset)) = contents.get(key) {
      return Some(asset);
    }
  }
  None
}

pub fn get_inbound_references(refs: &HashMap<String, Vec<String>>, id: &str) -> Vec<String> {
  let mut inbound = vec![];
  for (key, value) in refs.iter() {
    if value.contains(&id.to_string()) {
      inbound.push(key.to_owned());
    }
  }
  inbound
}

pub fn json_graph(
  nodes: &HashMap<String, Content>,
  edges: &HashMap<String, Vec<String>>,
) -> String {
  let n: Vec<Value> = nodes
    .iter()
    .filter_map(|(_k, c)| match c {
      Content::Article(a) => Some(a),
      _ => None,
    })
    .map(|a| {
      let mut m = Map::new();
      let mut data = Map::new();
      data.insert("id".to_string(), Value::String(a.id.clone()));
      data.insert(
        "url".to_string(),
        Value::String(format!("/{}", a.permalink.clone())),
      );
      m.insert("data".to_string(), Value::Object(data));
      Value::Object(m)
    })
    .collect();

  let e = edges
    .iter()
    .flat_map(|(k, v)| {
      v.iter()
        .filter_map(|id| match nodes.get(id) {
          Some(Content::Article(_)) => Some(id),
          _ => None,
        })
        .map(|inner_v| {
          let mut m = Map::new();
          let mut data = Map::new();
          data.insert(
            "id".to_string(),
            Value::String(format!("{}-{}", k, inner_v)),
          );
          data.insert("source".to_string(), Value::String(k.to_string()));
          data.insert("target".to_string(), Value::String(inner_v.to_string()));
          m.insert("data".to_string(), Value::Object(data));
          Value::Object(m)
        })
        .collect::<Vec<Value>>()
    })
    .collect();

  let mut object = Map::new();
  object.insert("edges".to_string(), Value::Array(e));
  object.insert("nodes".to_string(), Value::Array(n));

  serde_json::to_string(&object).unwrap()
}
