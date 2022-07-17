use crate::{inform, line_separator, querys::Query, text_utills::explain_something};
use chrono::Utc;
use clap::Parser;
use colored::Colorize;
use std::process::exit;

use crate::{cli::CLI, content_source::ContentSource, logger::InfoLogger, querys};

pub struct Driver {
    logger: InfoLogger,
    pub cli: CLI,
    pub web_client: reqwest::blocking::Client,
    querys: querys::QueryStore,
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
        println!("{:#?}", self.cli);

        let target_domain = ContentSource::from(self.cli.source());
        let query_string = ContentSource::generate_query_string(&target_domain, &self.cli.query());

        line_separator!();
        inform!(
            statement,
            "Searching".to_string(),
            self.cli.query().to_uppercase(),
            self.logger
        );
        line_separator!();

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
                r.text().unwrap_or(Default::default())
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
            Query::new(target_domain, &self.cli.query(), Utc::now()),
            query_response_string,
        );

        self
    }

    pub fn querys(&self) -> &querys::QueryStore {
        &self.querys
    }

    pub fn intrepert_query_result(&self) {
        todo!()
    }

    pub fn get_last_query(&self) -> &(Query, String) {
        &self.querys.last_query
    }
}
