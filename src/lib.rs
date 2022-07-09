pub use clap::Parser;

/// A CLI tool to research your programming doubts
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CLI {
    /*
        /// Name of the person to greet
        // #[clap(short, long, value_parser)]
        // pub name: String,

        /// Number of times to greet
        // #[clap(short, long, value_parser, default_value_t = 1)]
        // pub count: u8,
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
