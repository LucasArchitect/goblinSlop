pub mod insert;
pub mod queries;
pub mod schema;
pub mod types;

pub use types::{ContentEntry, DynamicPage, SourceRef};
pub use schema::init_db;
pub use insert::insert_content;
pub use queries::{
    count_all_content,
    get_all_content,
    get_content_by_category,
    get_content_by_slug,
    get_content_by_tag,
    get_content_paginated,
    search_content,
};
