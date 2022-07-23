use crate::querys::Query;

use browsy_helpers::{
    inform, line_separator,
    logger::InfoLogger,
    text_utills::{explain_something, TextPadding},
};
use browsy_lib::docs::DocsCrate;
use chrono::Utc;
use clap::Parser;
use colored::Colorize;
use std::process::exit;
use terminal_menu::{back_button, button, label, menu, mut_menu, scroll, TerminalMenuItem};

use crate::{cli::CLI, content_source::ContentSource, querys};

pub struct Driver {
    logger: InfoLogger,
    pub cli: CLI,
    pub web_client: reqwest::blocking::Client,
    querys: querys::QueryStore,
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    pub fn new() -> Self {
        Self {
            cli: CLI::parse(),
            web_client: reqwest::blocking::Client::new(),
            querys: Default::default(),
            logger: Default::default(),
        }
    }

    pub fn run_query_against_source(&mut self) -> &mut Self {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let target_domain = ContentSource::from(self.cli.source());
        let query_string = ContentSource::generate_query_string(
            &target_domain,
            self.cli.query(),
            match self.cli.custom() {
                true => Some((self.cli.quantity(), self.cli.page_index())),
                false => None,
            },
        );

        line_separator!(35);
        inform!(
            statement,
            "Searching".to_string(),
            format!(
                "\"{}\" @ {}",
                self.cli.query().to_uppercase().bold(),
                self.cli.source().to_uppercase().underline()
            ),
            self.logger
        );
        line_separator!(35);

        let query_request = match self.web_client.get(query_string.clone()).build() {
            Ok(request) => {
                inform!(
                    success,
                    "Query building".to_string(),
                    "Given query is valid".to_string(),
                    self.logger
                );
                request
            }
            Err(_) => {
                inform!(
                    fail,
                    "Request Building".to_string(),
                    "Could not create request from string query".to_string(),
                    self.logger
                );
                exit(1)
            }
        };

        let query_response_string = match self.web_client.execute(query_request) {
            Ok(r) => {
                inform!(statement, "Querying".to_string(), query_string, self.logger);
                inform!(
                    success,
                    "Request Success".to_string(),
                    "Response is OK".to_string(),
                    self.logger
                );
                // Don't want to exit or crash here, so we will handle
                // the lack of response content later, with a nice log message
                r.text().unwrap_or_default()
            }
            Err(why) => {
                inform!(
                    fail,
                    "Bad Response".to_string(),
                    explain_something(
                        "Could not read the text content of the response",
                        why.to_string()
                    ),
                    self.logger
                );
                exit(3)
            }
        };

        self.querys.add_new_query(
            Query::new(target_domain, self.cli.query(), Utc::now()),
            query_response_string,
        );

        self
    }

    pub fn intrepert_query_result(&self) {
        match self.querys().last_search_query_source().source {
            ContentSource::Docs(_) => Self::handle_docsrs_query(
                self.querys().last_search().get_query(),
                self.querys().last_search().get_content(),
            ),
            ContentSource::Lib(_) => todo!(),
            ContentSource::Crates(_) => todo!(),
        }
    }

    fn handle_docsrs_query(query: Query, html_content: String) {
        let docs_query_results =
            browsy_lib::docs::DocsQuery::new(query.query().to_string(), html_content);

        let crates = docs_query_results
            .crate_results
            .clone()
            .into_iter()
            .map(|c| c.crate_widget_fmt())
            .collect::<Vec<String>>();

        let crates_descriptors = docs_query_results
            .crate_results
            .into_iter()
            .collect::<Vec<DocsCrate>>();

        let main = menu(vec![
            label("Crates Found".p().white().on_green().bold().to_string()),
            label("---------"),
            scroll("Select a crate", crates),
            back_button("Continue"),
            label("---------"),
            label("[q] To Quit".italic().bright_black().to_string()),
        ]);

        println!();
        line_separator!(40);
        loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            terminal_menu::run(&main);

            let mm = mut_menu(&main);
            let selector = mm.selection_value("Select a crate");

            if mm.canceled() {
                break;
            }

            match crates_descriptors
                .clone()
                .iter()
                .find(|&n| n.crate_widget_fmt() == selector)
            {
                Some(v) => crate_description_menu(v),
                None => break,
            };
        }
    }

    pub fn querys(&self) -> &querys::QueryStore {
        &self.querys
    }
}

fn crate_description_menu(v: &DocsCrate) {
    let mut menu_vec = vec![
        label(
            "Crate Description"
                .p()
                .white()
                .on_green()
                .bold()
                .to_string(),
        ),
        label("---------"),
    ];
    let mut x = v
        .crate_info_line_separated()
        .iter()
        .map(|c| label(c))
        .collect::<Vec<TerminalMenuItem>>();
    menu_vec.append(&mut x);
    menu_vec.push(button("Exit"));
    let desc_menu = terminal_menu::menu(menu_vec);
    terminal_menu::run(&desc_menu);
}
