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
    /// Generates the adequate query for the, question relating to a certain content source
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///  let number_results = 30;
    ///  let docs_page = 2;
    ///  let query = "generics";
    ///  let source = ContentSource::from("docs");
    ///
    ///  let built_query = source.generate_query_string(query, Some((number_results, docs_page)));
    ///  let want =
    ///      "https://docs.rs/releases/search?paginate=P3E9Z2VuZXJpY3MmcGVyX3BhZ2U9MzAmcGFnZT0y"
    ///          .to_string();
    ///  assert_eq!(built_query, want);
    /// # }
    /// ```
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

    /// Replaces the spaces with __+__ for query url usage
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///   let query = "Proc Macros";
    ///   let number_results = 4;
    ///   let docs_page = 2;
    ///
    ///   assert_eq!(
    ///     ContentSource::parse_docs_custom_query(query, number_results, docs_page),
    ///     base64::encode("?q=Proc+Macros&per_page=4&page=2")
    ///         .trim_end_matches('=')
    ///         .to_string()
    ///   )
    /// # }
    /// ```
    pub fn parse_query(query: &str) -> String {
        query.replace(' ', "+")
    }

    /// Join all query parameters to form a corret docs.rs
    /// custom search for crates relatting to the query.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///   let query = "Proc Macros";
    ///   let number_results = 4;
    ///   let docs_page = 2;
    ///
    ///   assert_eq!(
    ///     ContentSource::parse_docs_custom_query(query, number_results, docs_page),
    ///     base64::encode("?q=Proc+Macros&per_page=4&page=2")
    ///         .trim_end_matches('=')
    ///         .to_string()
    ///   )
    /// # }
    /// ```
    pub fn parse_docs_custom_query(query: &str, per_page: i32, page: i32) -> String {
        base64::encode(format!(
            "?q={}&per_page={per_page}&page={page}",
            ContentSource::parse_query(query)
        ))
        .trim_end_matches('=')
        .to_string()
    }

    /// To quickly get a ContentSource __docs__ Variance
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///   let simple_source = ContentSource::docs();
    ///   assert_eq!(simple_source, ContentSource::from("docs"))
    /// # }
    /// ```
    pub fn docs() -> ContentSource {
        ContentSource::from("docs")
    }
    /// To quickly get a ContentSource __lib__ Variance
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///   let simple_source = ContentSource::lib();
    ///   assert_eq!(simple_source, ContentSource::from("lib"))
    /// # }
    /// ```
    pub fn lib() -> ContentSource {
        ContentSource::from("lib")
    }
    /// To quickly get a ContentSource __crates__ Variance
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::content_source::ContentSource;
    /// # fn main() {
    ///   let simple_source = ContentSource::crates();
    ///   assert_eq!(simple_source, ContentSource::from("crates"))
    /// # }
    /// ```
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

#[cfg(test)]
mod test_content_source {
    use super::ContentSource;

    #[test]
    fn test_from_trait_impl() {
        let have = vec![
            ContentSource::from("docs"),
            ContentSource::from("crates"),
            ContentSource::from("lib"),
        ];

        let want = vec![
            ContentSource::docs(),
            ContentSource::crates(),
            ContentSource::lib(),
        ];

        assert_eq!(want, have)
    }

    #[test]
    fn test_parse_query() {
        let want = "Some+generic+Url+encoding".to_string();
        let have = ContentSource::parse_query("Some+generic+Url+encoding");

        assert_eq!(want, have)
    }

    #[test]
    fn test_parse_docs_custom_query() {
        let query = "Proc Macros";
        let docs_page = 2;
        let number_results = 4;

        assert_eq!(
            ContentSource::parse_docs_custom_query(query, number_results, docs_page),
            base64::encode("?q=Proc+Macros&per_page=4&page=2")
                .trim_end_matches('=')
                .to_string()
        )
    }

    #[test]
    fn test_generate_query_string_docs() {
        let query = "Super cool";
        let source = ContentSource::from("docs");

        let built_query = source.generate_query_string(query, None);
        let want = "https://docs.rs/releases/search?query=Super+cool".to_string();
        assert_eq!(built_query, want);
    }

    #[test]
    fn test_generate_query_string_docs_custom() {
        let number_results = 30;
        let docs_page = 2;
        let query = "generics";
        let source = ContentSource::from("docs");

        let built_query = source.generate_query_string(query, Some((number_results, docs_page)));
        let want =
            "https://docs.rs/releases/search?paginate=P3E9Z2VuZXJpY3MmcGVyX3BhZ2U9MzAmcGFnZT0y"
                .to_string();
        assert_eq!(built_query, want);
    }
}
