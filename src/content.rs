use crate::links;
use crate::meta::Meta;
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Content is any item of data that we want to move or process
/// from our source to our destination.
#[derive(Debug)]
pub enum Content {
    Article(Article),
    Asset(Asset),
}

#[derive(Clone, Debug, Serialize)]
pub struct Article {
    pub id: String,
    pub permalink: String,
    pub src: String,
    pub meta: Option<Meta>,
    pub content: String,
}
#[derive(Debug)]
pub struct Asset {
    pub id: String,
    pub permalink: String,
    pub src: String,
}

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

pub fn get_references(contents: &HashMap<String, Content>) -> HashMap<String, Vec<String>> {
    contents
        .iter()
        .filter_map(|(_k, c)| match c {
            Content::Article(a) => Some(a),
            _ => None,
        })
        .fold(HashMap::new(), |mut m, a| {
            let refs = links::extract(contents, a);
            m.insert(a.id.to_string(), refs);
            m
        })
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
            data.insert("url".to_string(), Value::String(a.permalink.clone()));
            m.insert("data".to_string(), Value::Object(data));
            Value::Object(m)
        })
        .collect();

    let e = edges
        .iter()
        .flat_map(|(k, v)| {
            v.iter()
                .filter(|&id| matches!(nodes.get(id), Some(Content::Article(_))))
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
