use crate::querys::Query;
use chrono::Utc;
use clap::Parser;
use std::process::exit;

use crate::{cli::CLI, content_source::ContentSource, logger::Logger, querys};

pub struct Driver {
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
        }
    }

    pub fn run_query_against_source(&mut self) -> () {
        let mut proc_logger = Logger::default();

        let target_domain = ContentSource::from(self.cli.source());
        let query_string = ContentSource::generate_query_string(&target_domain, &self.cli.query());

        let query_request = match self.web_client.get(query_string).build() {
            Ok(request) => {
                proc_logger
                    .restate_log("Query building", "Given query is valid")
                    .success_log()
                    .log();
                request
            }
            Err(_) => {
                proc_logger
                    .restate_log(
                        "Request Building",
                        "Could not create request from string query",
                    )
                    .failure_log()
                    .write();
                exit(1)
            }
        };

        let query_response_string = match self.web_client.execute(query_request) {
            Ok(r) => {
                proc_logger
                    .restate_log("Request Success", "Response is OK")
                    .success_log();
                r.text().unwrap_or(Default::default())
            }
            Err(_) => {
                proc_logger
                    .restate_log(
                        "Bad Response",
                        "Could not read the text content of the response",
                    )
                    .failure_log()
                    .write();
                exit(3)
            }
        };

        self.querys.add_new_query(
            Query::new(target_domain, &self.cli.query(), Utc::now()),
            query_response_string,
        );
    }
}
