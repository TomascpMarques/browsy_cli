/*
    TODO
    The docs.rs website can specifie throug a hash, the query, number of results and page index
    i.e: P3E9R2VuZXJpY3MmcGVyX3BhZ2U9MiZwYWdlPTE -> ?q=Generics&per_page=2&page=1
    GOTTA ACCOUNT FOR THIS
*/

use clap::Parser;

/// A CLI tool to research your Rust™ programming doubts
#[derive(Parser, Debug, Clone)]
#[clap(author = "Tomas Marques", version, about, long_about = None)]
pub struct CLI {
    /// String query to use as target search values
    #[clap(short, long, value_parser)]
    query: String,

    /// Source to use in the query resolution
    #[clap(short, long, value_parser = ["docs", "lib", "crates"], default_value("docs"))]
    source: String,

    /// Allows to specefie custom querie params, like item quantitiy in responsses
    /// and  page index for those seraches.
    #[clap(short, default_missing_value("true"))]
    custom: bool,

    /// Specefies the number of results shown in the search query
    #[clap(
        long,
        required_ifs(&vec![("custom", "true")]),
        default_value("10"),
        default_missing_value("10")
    )]
    quantity: i32,

    /// Allows for usage of an interactive mode, selecting multiple factors (unimplemented)
    #[clap(long, short, required(false), default_missing_value("false"))]
    interactive: bool,
}

impl CLI {
    pub fn interactive(&self) -> bool {
        self.interactive
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn query(&self) -> &str {
        self.query.as_ref()
    }
}
