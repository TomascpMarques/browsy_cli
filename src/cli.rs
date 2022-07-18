use clap::Parser;
/// A CLI tool to research your Rust™ programming doubts
#[derive(Parser, Debug, Clone)]
#[clap(author = "Tomas Marques", version, about, long_about = None)]
pub struct CLI {
    /// String query to use as target search values
    #[clap(short, long, value_parser)]
    query: String,

    /// Source to use in the query resolution
    #[clap(short, long, value_parser = ["docs", "lib", "crates"], default_value("docs"), default_missing_value("docs"))]
    source: String,

    /// Allows to specefie custom querie params, like item quantitiy in responsses
    /// and  page index for those seraches.
    #[clap(short, default_missing_value("true"))]
    custom: bool,

    /// Specefies the number of results shown in the search query
    #[clap(
        long = "quantity",
        required_ifs(&vec![("custom", "true")]),
        default_value("10"),
        default_missing_value("10")
    )]
    per_page: i32,

    /// Specefies website page index for result pagination
    #[clap(
        long = "page",
        // required_ifs(&vec![("custom", "true")]),
        default_value("1"),
        default_missing_value("1")
    )]
    page: i32,

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

    pub fn page_index(&self) -> i32 {
        self.page
    }

    pub fn quantity(&self) -> i32 {
        self.per_page
    }

    pub(crate) fn custom(&self) -> bool {
        self.custom
    }
}
