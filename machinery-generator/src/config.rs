use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub projects: IndexMap<String, Project>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub headers: String,
    pub headers_prefix: String,
    pub target: String,
    pub prefix_headers: Option<Vec<String>>,
    pub uses: Option<Vec<String>>,
    pub blocklist: Option<Vec<String>>,
}
