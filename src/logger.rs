use colored::{ColoredString, Colorize};

use crate::{pad, text_padding::TextPadding};

#[derive(Debug, PartialEq, Hash, Clone)]
pub struct Logger {
    pub title: &'static str,
    pub message: &'static str,
    log: String,
}

impl Logger {
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

    pub fn restate_log(&mut self, title: &'static str, message: &'static str) -> &mut Logger {
        self.message = message;
        self.title = title;
        self
    }

    pub fn default_log(&mut self) -> &mut Logger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.title).on_blue().bold()),
                (2, pad!(self.message).white().italic()),
            ],
        );
        self
    }

    pub fn warn_log(&mut self) -> &mut Logger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, pad!(self.title).on_blue().bold()),
                (2, pad!(self.message).yellow().bold()),
            ],
        );
        self
    }

    pub fn success_log(&mut self) -> &mut Logger {
        self.log = Self::template_replace(
            Self::LOG_TEMPLATE,
            vec![
                (1, self.title.pad(" ", 1).on_green().bold()),
                (2, self.message.pad(" ", 1).underline().bright_green()),
            ],
        );
        self
    }

    pub fn failure_log(&mut self) -> &mut Logger {
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

impl Default for Logger {
    fn default() -> Self {
        Self {
            title: Default::default(),
            message: Default::default(),
            log: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! box_log {
    ($exp: expr) => {
        Box::<LogMessage>::new($exp)
    };
}

#[cfg(test)]
mod test {
    use crate::pad;
    use crate::text_padding::TextPadding;
    use colored::Colorize;

    use super::Logger;

    #[test]
    fn build_log_struct() {
        let have = Logger::new("title", "message");
        let want = Logger {
            title: "title",
            message: "message",
            log: "".to_string(),
        };
        assert_eq!(want, have)
    }

    #[test]
    fn test_log_printing() {
        let _ = Logger::new("title", "message").default_log().log();
        let _ = Logger::new("title", "message").warn_log().log();
        let _ = Logger::new("title", "message").success_log().log();
        let _ = Logger::new("title", "message").failure_log().log();

        // remove comment to see output
        // assert!(false)
    }

    #[test]
    fn test_copy_log_message() {
        let mut target = Logger::new("title", "message");
        let want = target.default_log().clone().log;
        let have = target.copy_log();

        assert_eq!(want, have)
    }

    #[test]
    fn test_restate_log_info() {
        let have = (
            Logger::new("title", "message")
                .restate_log("TITLE", "MESSAGE")
                .title,
            Logger::new("title", "message")
                .restate_log("TITLE", "MESSAGE")
                .message,
        );
        let want = Logger::new("TITLE", "MESSAGE");

        assert_eq!((want.title, want.message), have)
    }

    #[test]
    fn test_log_template_replace() {
        let template = "#$1# #$2#";
        let temp = Logger::new("title", "message").default_log().copy_log();

        let have = Logger::template_replace(
            template,
            vec![
                (1, pad!("title").on_blue().bold()),
                (2, pad!("message").white().italic()),
            ],
        );

        assert_eq!(temp, have);

        let template = "#$1# #$2#";
        let temp = Logger::new("title", "message").default_log().copy_log();

        let have = Logger::template_replace(
            template,
            vec![
                (1, pad!("title").on_black().bold()),
                (2, pad!("messagee").white().on_bright_green()),
            ],
        );

        assert_ne!(temp, have);
        let template = "#$1# #$2#";
        let temp = Logger::new("title", "message").default_log().copy_log();

        let have = Logger::template_replace(
            template,
            vec![
                (1, pad!("titleII").on_blue().bold()),
                (2, pad!("message###").white().italic()),
            ],
        );

        assert_ne!(temp, have)
    }
}
