use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceRef {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentEntry {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: String,
    pub category: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub sources: Vec<SourceRef>,
    pub is_dynamic: bool,
    pub date_added: String,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynamicPage {
    pub path: String,
    pub title: String,
    pub content: String,
    pub keywords: Vec<String>,
}