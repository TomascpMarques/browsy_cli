use colored::{ColoredString, Colorize};

use crate::{pad, text_padding::TextPadding};

#[derive(Debug, PartialEq, Hash, Clone)]
pub struct InfoLogger {
    pub title: &'static str,
    pub message: &'static str,
    log: String,
}

impl InfoLogger {
    const LOG_TEMPLATE: &'static str = "#$1# #$2#";

    pub fn new(title: &'static str, message: &'static str) -> Self {
        Self {
            title,
            message,
            ..Default::default()
        }
    }

    pub fn template_replace(templ: &str, pairs: Vec<(i32, ColoredString)>) -> String {
        let mut builder = String::from(templ);
        pairs.iter().for_each(|pair| {
            builder = builder.replace(
                format!("#${}#", pair.0).as_str(),
                pair.1.to_string().as_str(),
            );
        });
        builder
    }

    pub fn restate_log(&mut self, title: &'static str, message: &'static str) -> &mut InfoLogger {
        self.message = message;
        self.title = title;
        self
    }

    pub fn statement(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.title).on_blue().bold()),
                (2, pad!(self.message).white().italic()),
            ],
        );
        self
    }

    pub fn warning(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.title).white().on_bright_yellow().bold()),
                (2, pad!(self.message).yellow().bold()),
            ],
        );
        self
    }

    pub fn success(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, self.title.pad(" ", 1).on_green().bold()),
                (2, self.message.pad(" ", 1).underline().bright_green()),
            ],
        );
        self
    }

    pub fn failure(&mut self) -> &mut InfoLogger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.title).on_red().white().bold()),
                (2, pad!(self.message).yellow().bold().underline()),
            ],
        );
        self
    }

    pub fn write(&self) {
        println!("{}", self.log)
    }

    pub fn log(&mut self) -> &mut Self {
        println!("{}", self.log);
        self
    }

    pub fn copy_log(&self) -> String {
        self.log.clone()
    }
}

impl Default for InfoLogger {
    fn default() -> Self {
        Self {
            title: Default::default(),
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
        let have = InfoLogger::new("title", "message");
        let want = InfoLogger {
            title: "title",
            message: "message",
            log: "".to_string(),
        };
        assert_eq!(want, have)
    }

    #[test]
    fn test_log_printing() {
        let _ = InfoLogger::new("title", "message").statement().log();
        let _ = InfoLogger::new("title", "message").warning().log();
        let _ = InfoLogger::new("title", "message").success().log();
        let _ = InfoLogger::new("title", "message").failure().log();

        // remove comment to see output
        // assert!(false)
    }

    #[test]
    fn test_copy_log_message() {
        let mut target = InfoLogger::new("title", "message");
        let want = target.statement().clone().log;
        let have = target.copy_log();

        assert_eq!(want, have)
    }

    #[test]
    fn test_restate_log_info() {
        let have = (
            InfoLogger::new("title", "message")
                .restate_log("TITLE", "MESSAGE")
                .title,
            InfoLogger::new("title", "message")
                .restate_log("TITLE", "MESSAGE")
                .message,
        );
        let want = InfoLogger::new("TITLE", "MESSAGE");

        assert_eq!((want.title, want.message), have)
    }

    #[test]
    fn test_log_template_replace() {
        let template = "#$1# #$2#";
        let temp = InfoLogger::new("title", "message").statement().copy_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("title").on_blue().bold()),
                (2, pad!("message").white().italic()),
            ],
        );

        assert_eq!(temp, have);

        let template = "#$1# #$2#";
        let temp = InfoLogger::new("title", "message").statement().copy_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("title").on_black().bold()),
                (2, pad!("messagee").white().on_bright_green()),
            ],
        );

        assert_ne!(temp, have);

        let template = "#$1# #$2#";
        let temp = InfoLogger::new("title", "message").statement().copy_log();

        let have = InfoLogger::template_replace(
            template,
            vec![
                (1, pad!("titleII").on_blue().bold()),
                (2, pad!("message###").white().italic()),
            ],
        );

        assert_ne!(temp, have)
    }
}

#[macro_export]
macro_rules! inform {
    ($loger: ident, $title:literal - $message:expr) => {
        InfoLogger::new($title, $message).$loger().log()
    };
    ($loger: ident, msg: $message:expr) => {
        InfoLogger::new("Info", $message).success().log()
    };
    ($loger: ident, ttl: $title:expr) => {
        InfoLogger::new($title, "Something has occurred...")
            .$loger()
            .log()
    };
    ($loger: ident, $title:literal - $message:expr, $source:expr) => {
        $source.restate_log($title, $message).$loger().log()
    };
    ($loger: ident, msg: $message:expr, $source:expr) => {
        $source.restate_log($source.title, $message).$loger().log()
    };
    ($loger: ident, ttl: $title:expr, $source:expr ) => {
        $source.restate_log($title, $source.message).$loger().log()
    };
}

#[cfg(test)]
mod test_log_macros {
    use crate::logger::InfoLogger;

    #[test]
    fn test_inform_macro_simple() {
        inform!(success, "Hello" - "World");
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source() {
        let mut s = InfoLogger::new("Sourced", "Log");
        inform!(failure, "Hello" - "World", s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source_no_title() {
        let mut s = InfoLogger::new("WARNING", "Log");
        inform!(warning, msg: "Hello", s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_source_no_message() {
        let mut s = InfoLogger::new("Sourced", "Log");
        inform!(statement, ttl: "Hello", s);
        assert!(true)
    }
    #[test]
    fn test_inform_macro_no_message() {
        inform!(success, ttl: "No message given");
        assert!(true)
    }
    #[test]
    fn test_inform_macro_no_title() {
        inform!(statement, msg: "No title here");
        assert!(true)
    }
}
