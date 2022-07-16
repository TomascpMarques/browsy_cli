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
    pub fn generate_query_string(&self, query: &str) -> String {
        match self {
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
        q.replace(" ", "+")
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
