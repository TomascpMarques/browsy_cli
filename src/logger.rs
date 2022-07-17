use std::fmt::Display;

use colored::Colorize;

use crate::{pad, text_padding::TextPadding};

#[derive(Debug, PartialEq, Hash, Clone)]
/// A InfoLogger log is represented to the user as a pair
/// made of a __tittle and a message__, these after being
/// applied a log type _(ie.: success, warn, failure, etc.)_
/// will be built into the final log message, ready
/// to be print out.
pub struct InfoLogger {
    pub tittle: String,
    pub message: String,
    log: String,
}

#[macro_export]
/// __inform!()__ is a macro that simplifies log usage, when
/// the need is for a simple message or two, and not a fully
/// detailed comprehension of an operation or state is needed.
/// There for, it can be used with an existing logging source,
/// or create a new one, depending on the invocation of the
/// macro.
/// ## Example:
/// ```
/// # use crate::browsy_cli::logger::InfoLogger;
/// # use crate::browsy_cli::inform;
/// # fn main() {
/// // No existing logger usage:
///   inform!(success, "tittle".to_string(), "message".to_string());
///   inform!(success, msg: "message".to_string());
///   inform!(success, ttl: "tittle".to_string());
/// // Existing logger usage:
///   let mut logger = InfoLogger::new_default();
///   inform!(warn, "tittle".to_string(), "message".to_string(), logger);
///   inform!(statement, msg: "message".to_string(), logger);
///   inform!(failure, ttl: "tittle".to_string(), logger);
/// # }
/// ```
macro_rules! inform {
    ($loger: ident, $tittle:expr, $message:expr) => {
        InfoLogger::new($tittle, $message).$loger().log()
    };
    ($loger: ident, msg $message:expr) => {
        InfoLogger::new("Info".to_string(), $message).$loger().log()
    };
    ($loger: ident, ttl $tittle:expr) => {
        InfoLogger::new($tittle, String::default()).$loger().log()
    };
    ($loger: ident, $tittle:expr, $message:expr, $source:expr) => {
        $source.restate_log($tittle, $message).$loger().log()
    };
    ($loger: ident, msg $message:expr, $source:expr) => {
        $source
            .restate_log($source.tittle.clone(), $message)
            .$loger()
            .log()
    };
    ($loger: ident, ttl $tittle:expr, $source:expr ) => {
        $source
            .restate_log($tittle, $source.message.clone())
            .$loger()
            .log()
    };
    ($loger: ident, $source:expr ) => {
        $source.$loger().log()
    };
}

impl InfoLogger {
    const LOG_TEMPLATE: &'static str = "#$1# #$2#";

    pub fn new_default() -> Self {
        Self {
            tittle: Default::default(),
            message: Default::default(),
            log: Default::default(),
        }
    }

    pub fn new(tittle: String, message: String) -> Self {
        Self {
            tittle,
            message,
            ..Default::default()
        }
    }

    /// Replaces template literals in a `&str`, with the correspondig value,
    /// insside a (index, value) tuple.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # fn main() {
    ///   let template_str = "This is a cool template string bool !";
    ///   let built_template = InfoLogger::template_replace(
    ///      // Tou can repeat template literals in the string
    ///      // allowing you to reuse values allong the entire string
    ///      "This is #$1# co#$2# template #$3# bo#$2# #$4#",
    ///       vec![
    ///           (1, "a"), (2, "ol"),
    ///           (3, "string"), (4, "!"),
    ///       ],
    ///   );
    ///   assert_eq!(template_str, built_template)
    /// # }
    /// ```
    pub fn template_replace<T>(templ: &'static str, pairs: Vec<(i32, T)>) -> String
    where
        T: Display,
    {
        let mut builder = String::from(templ);
        pairs.iter().for_each(|pair| {
            builder = builder.replace(
                format!("#${}#", pair.0).as_str(),
                pair.1.to_string().as_str(),
            );
        });
        builder
    }

    /// Restates the tittle and message used for each log message, use it to change the
    /// info shown to the user, usually between log printing.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1")
    ///     .warning().log()
    ///     .restate_log("AAA", "BBB")
    ///     .success().log();
    /// # }
    /// ```
    pub fn restate_log(&mut self, tittle: String, message: String) -> &mut InfoLogger {
        self.message = message;
        self.tittle = tittle;
        self
    }

    /// Builds a `default` log, a statement, with no conotations attached.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # use crate::browsy_cli::inform;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1");
    ///   info_logger.statement().log();
    ///
    ///   // Or with a simple to use macro:
    ///   // Example 1 - Uses an existing Logger
    ///   inform!(statement, info_logger);
    ///   // Example 2 - No existing Logger, just spit out the info
    ///   inform!(statement, "Tip" - "You look great :)");
    /// # }
    /// ```
    pub fn statement(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, self.tittle.p().on_blue().bold()),
                (2, self.message.p().white().italic()),
            ],
        );
        self
    }

    /// Builds a `warning` log, colored to look like one.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # use crate::browsy_cli::inform;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1");
    ///   info_logger.warn().log();
    ///
    ///   // Or with a simple to use macro:
    ///   // Example 1 - Uses an existing Logger
    ///   inform!(warn, info_logger);
    ///   // Example 2 - No existing Logger, just spit out the info
    ///   inform!(warn, "WARNING" - "Did you remember to have lunch?");
    /// # }
    /// ```
    pub fn warn(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (
                    1,
                    pad!(self.tittle)
                        .white()
                        .on_bright_yellow()
                        .bold()
                        .to_string(),
                ),
                (2, self.message.p().yellow().bold().to_string()),
            ],
        );
        self
    }

    /// Builds a `success` log, colored to look like one.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # use crate::browsy_cli::inform;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1");
    ///   info_logger.success().log();
    ///
    ///   // Or with a simple to use macro:
    ///   // Example 1 - Uses an existing Logger
    ///   inform!(success, info_logger);
    ///   // Example 2 - No existing Logger, just spit out the info
    ///   inform!(success, "WARNING" - "Did you remember to have lunch?");
    /// # }
    /// ```
    pub fn success(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, self.tittle.pad(" ", 1).on_green().bold()),
                (2, self.message.pad(" ", 1).underline().bright_green()),
            ],
        );
        self
    }

    /// Builds a `failure` log, colored to look like one.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # use crate::browsy_cli::inform;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1");
    ///   info_logger.fail().log();
    ///
    ///   // Or with a simple to use macro:
    ///   // Example 1 - Uses an existing Logger
    ///   inform!(fail, info_logger);
    ///   // Example 2 - No existing Logger, just spit out the info
    ///   inform!(fail, "WARNING" - "Did you remember to have lunch?");
    /// # }
    /// ```
    pub fn fail(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.tittle).on_red().white().bold()),
                (2, pad!(self.message).yellow().bold().underline()),
            ],
        );
        self
    }

    /// Prints to the standard output, with a newline, the colored
    /// contents of the log message. __If no template was applied to the
    /// logger, it will return an empty string__.
    /// ## Example:
    /// ```
    /// # use crate::browsy_cli::logger::InfoLogger;
    /// # use crate::browsy_cli::inform;
    /// # fn main() {
    ///   let mut info_logger = InfoLogger::new("1tittle1", "1Message1");
    ///   info_logger.fail().log();
    /// # }
    /// ```
    pub fn log(&mut self) -> &mut Self {
        println!("{}", self.log);
        self
    }

    /// Clone the logs contents, and returns that cloned `String`.
    pub fn clone_log(&self) -> String {
        self.log.clone()
    }
}

impl Default for InfoLogger {
    fn default() -> Self {
        Self {
            tittle: Default::default(),
            message: Default::default(),
            log: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pad;
    use crate::text_padding::TextPadding;
    use colored::Colorize;

    use super::InfoLogger;

    #[test]
    fn build_log_struct() {
        let have = InfoLogger::new("tittle".to_string(), "message".to_string());
        let want = InfoLogger {
            tittle: "tittle".to_string(),
            message: "message".to_string(),
            log: "".to_string(),
        };
        assert_eq!(want, have)
    }

    #[test]
    fn test_log_printing() {
        let _ = InfoLogger::new("tittle".to_string(), "message".to_string())
            .statement()
            .log();
        let _ = InfoLogger::new("tittle".to_string(), "message".to_string())
            .warn()
            .log();
        let _ = InfoLogger::new("tittle".to_string(), "message".to_string())
            .success()
            .log();
        let _ = InfoLogger::new("tittle".to_string(), "message".to_string())
            .fail()
            .log();

        // remove comment to see output
        // assert!(false)
    }

    #[test]
    fn test_copy_log_message() {
        let mut target = InfoLogger::new("tittle".to_string(), "message".to_string());
        let want = target.statement().clone().log;
        let have = target.clone_log();

        assert_eq!(want, have)
    }

    #[test]
    fn test_restate_log_info() {
        let have = (
            InfoLogger::new("tittle".to_string(), "message".to_string())
                .restate_log("tittle".to_string(), "MESSAGE".to_string())
                .tittle
                .clone(),
            InfoLogger::new("tittle".to_string(), "message".to_string())
                .restate_log("tittle".to_string(), "MESSAGE".to_string())
                .message
                .clone(),
        );
        let want = InfoLogger::new("tittle".to_string(), "MESSAGE".to_string());

        assert_eq!((want.tittle, want.message), have)
    }

    #[test]
    fn test_log_template_replace() {
        let template = "#$1# #$2#";
        let temp = InfoLogger::new("tittle".to_string(), "message".to_string())
            .statement()
            .clone_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("tittle".to_string()).on_blue().bold()),
                (2, pad!("message".to_string()).white().italic()),
            ],
        );

        assert_eq!(temp, have);

        let template = "#$1# #$2#";
        let temp = InfoLogger::new("tittle".to_string(), "message".to_string())
            .statement()
            .clone_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("tittle".to_string()).on_black().bold()),
                (2, pad!("messagee").white().on_bright_green()),
            ],
        );

        assert_ne!(temp, have);

        let template = "#$1# #$2#";
        let temp = InfoLogger::new("tittle".to_string(), "message".to_string())
            .statement()
            .clone_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("tittleII").on_blue().bold()),
                (2, pad!("message###").white().italic()),
            ],
        );

        assert_ne!(temp, have)
    }
}

#[cfg(test)]
mod test_log_macros {
    use crate::logger::InfoLogger;

    #[test]
    fn test_inform_macro_simple() {
        inform!(success, "Hello".to_string(), "World".to_string());
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source() {
        let mut s = InfoLogger::new("Sourced".to_string(), "Log".to_string());
        inform!(fail, "Hello".to_string(), "World".to_string(), s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source_no_tittle() {
        let mut s = InfoLogger::new("WARNING".to_string(), "Log".to_string());
        inform!(warn, msg "Hello".to_string(), s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source_no_message() {
        let mut s = InfoLogger::new("Sourced".to_string(), "Log".to_string());
        inform!(statement, ttl "Hello".to_string(), s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_no_message() {
        inform!(success, ttl "No message given".to_string());
        assert!(true)
    }
    #[test]
    fn test_inform_macro_no_tittle() {
        inform!(statement, msg "No tittle here".to_string());
        assert!(true)
    }
}
