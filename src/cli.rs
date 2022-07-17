pub use clap::Parser;

/*
    TODO
    The docs.rs website can specifie throug a hash, the query, number of results and page index
    i.e: P3E9R2VuZXJpY3MmcGVyX3BhZ2U9MiZwYWdlPTE -> ?q=Generics&per_page=2&page=1
    GOTTA ACCOUNT FOR THIS
*/

/// A CLI tool to research your programming doubts
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CLI {
    /// String query to use as target search values
    #[clap(short, long, value_parser)]
    query: String,

    /// Source/s to use in the query resolution
    #[clap(short, long, value_parser = ["docs", "lib", "crates"], default_value = "docs")]
    source: String,

    /// Allows for usage of an interactive mode, selecting multiple factors
    #[clap(short, default_value = "n", required = false)]
    interactive: char,
}

impl CLI {
    pub fn interactive(&self) -> char {
        self.interactive
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn query(&self) -> &str {
        self.query.as_ref()
    }
}
