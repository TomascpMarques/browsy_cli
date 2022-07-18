#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentSource {
    Docs(String),
    Lib(String),
    Crates(String),
}

impl Default for ContentSource {
    fn default() -> Self {
        ContentSource::docs()
    }
}

impl ContentSource {
    pub fn generate_query_string(&self, query: &str, custom_q: Option<(i32, i32)>) -> String {
        match self {
            ContentSource::Docs(q) if custom_q.is_some() => {
                format!(
                    "{}releases/search?paginate={}",
                    q,
                    ContentSource::parse_docs_custom_query(
                        query,
                        custom_q.unwrap().0,
                        custom_q.unwrap().1
                    )
                )
            }
            ContentSource::Docs(q) => {
                format!(
                    "{}releases/search?query={}",
                    q,
                    ContentSource::parse_query(query)
                )
            }
            ContentSource::Lib(q) => format!("{}search?q={}", q, ContentSource::parse_query(query)),
            ContentSource::Crates(q) => {
                format!("{}search?q={}", q, ContentSource::parse_query(query))
            }
        }
    }

    pub fn parse_query(q: &str) -> String {
        q.replace(' ', "+")
    }

    pub fn parse_docs_custom_query(query: &str, per_page: i32, page: i32) -> String {
        base64::encode(format!(
            "?q={}&per_page={}&page={}",
            query.replace(' ', "+"),
            per_page,
            page
        ))
        .trim_end_matches('=')
        .to_string()
    }

    pub fn docs() -> ContentSource {
        ContentSource::from("docs")
    }
    pub fn lib() -> ContentSource {
        ContentSource::from("lib")
    }
    pub fn crates() -> ContentSource {
        ContentSource::from("crates")
    }
}

impl From<&str> for ContentSource {
    fn from(target: &str) -> Self {
        match target {
            "docs" => Self::Docs("https://docs.rs/".to_string()),
            "lib" => Self::Lib("https://lib.rs/".to_string()),
            "crates" => Self::Crates("https://crates.io/".to_string()),
            _ => {
                println!(
                    "Could not parsse the given source <{:?}>, defaulting to http://docs.rs",
                    target
                );
                Self::Docs("https://docs.rs/".to_string())
            }
        }
    }
}
