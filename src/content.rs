use crate::Content;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn cygate_graph(
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
