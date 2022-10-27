use crate::frontmatter::Frontmatter;
use crate::links;
use comrak::{format_html, nodes::NodeValue, parse_document, Arena, ComrakOptions};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Content is any item of data that we want to move or process
/// from our source to our destination.
#[derive(Debug)]
pub enum Content {
    Article(Box<Article>),
    Asset(Asset),
}

#[derive(Clone, Debug, Serialize)]
pub struct Article {
    pub id: String,
    pub permalink: String,
    pub src: String,
    pub meta: Option<Frontmatter>,
    pub content: Option<ArticleContent>,
    pub raw: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArticleContent {
    pub title: Option<String>,
    pub ingress: Option<String>,
    pub body: String,
    pub toc: HashMap<String, String>,
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

pub fn parse_raw(raw: &str, allow_html: bool) -> Option<ArticleContent> {
    let mut content = ArticleContent::default();

    let mut comrak_opts = ComrakOptions::default();
    comrak_opts.extension.tasklist = true;
    comrak_opts.extension.table = true;
    if allow_html {
        comrak_opts.render.unsafe_ = true;
    }

    let arena = Arena::new();
    let root = parse_document(&arena, raw, &comrak_opts);

    for node in root.children() {
        match node.data.clone().into_inner().value {
            NodeValue::Heading(c) if content.title == None && c.level == 1 => {
                let mut title = vec![];
                format_html(node, &comrak_opts, &mut title).unwrap();
                content.title = Some(String::from_utf8_lossy(&title).to_string());
                continue;
            }
            NodeValue::Heading(c) if c.level == 2 => {
                if let Some(text_node) = node.first_child() {
                    if let Some(text) = text_node.data.borrow().value.text() {
                        let heading = String::from_utf8_lossy(text).to_string();
                        content.toc.insert(heading.clone(), slug::slugify(&heading));
                        let mut html = vec![];
                        format_html(node, &comrak_opts, &mut html).unwrap();
                        content.body = format!(
                            "{}<a id=\"{}\">{}</a>",
                            content.body,
                            slug::slugify(&heading),
                            String::from_utf8_lossy(&html)
                        );
                    }
                }

                continue;
            }
            NodeValue::Paragraph if content.ingress == None => {
                if let Some(sibling) = node.previous_sibling() {
                    if let NodeValue::Heading(c) = sibling.data.clone().into_inner().value {
                        if c.level == 1 {
                            let mut ingress = vec![];
                            format_html(node, &comrak_opts, &mut ingress).unwrap();
                            content.ingress = Some(String::from_utf8_lossy(&ingress).to_string());
                            continue;
                        }
                    }
                }
                let mut html = vec![];
                format_html(node, &comrak_opts, &mut html).unwrap();
                content.body = format!("{}{}", content.body, String::from_utf8_lossy(&html));
                continue;
            }
            _ => {
                let mut html = vec![];
                format_html(node, &comrak_opts, &mut html).unwrap();
                content.body = format!("{}{}", content.body, String::from_utf8_lossy(&html));
                continue;
            }
        }
    }

    Some(content)
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
            data.insert("content".to_string(), Value::String(a.raw.clone()));
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
