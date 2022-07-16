pub use clap::Parser;

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
    #[clap(short, default_missing_value = "y")]
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
