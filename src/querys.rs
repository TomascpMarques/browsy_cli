use std::collections::HashMap;

use crate::logger::InfoLogger;

use chrono::Utc;

use chrono::DateTime;

use crate::content_source::ContentSource;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct Query {
    pub(crate) target: ContentSource,
    pub query: String,
    pub date: DateTime<Utc>,
}

impl Query {
    pub fn new(target: ContentSource, query: &str, date: DateTime<Utc>) -> Self {
        Self {
            query: query.to_string(),
            date,
            target: target,
        }
    }

    pub fn query(&self) -> &str {
        self.query.as_ref()
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn target(&self) -> &ContentSource {
        &self.target
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            query: Default::default(),
            date: Utc::now(),
            target: ContentSource::default(),
        }
    }
}

pub struct QueryStore {
    pub(crate) logger: InfoLogger,
    pub(crate) history: HashMap<ContentSource, Vec<Query>>,
    pub(crate) last_query: (Query, String),
}

impl QueryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_new_query(&mut self, query: Query, result: String) {
        self.last_query = (query.clone(), result);
        if let Some(entry) = self.history.get_mut(&query.target) {
            entry.push(query)
        } else {
            self.logger
                .restate_log("Query Storing", "Could not add to query history")
                .fail()
                .log();
        }
    }
}

impl Default for QueryStore {
    fn default() -> Self {
        let mut default_hmap_keys = HashMap::<ContentSource, Vec<Query>>::default();
        default_hmap_keys.insert(ContentSource::crates(), vec![]);
        default_hmap_keys.insert(ContentSource::docs(), vec![]);
        default_hmap_keys.insert(ContentSource::lib(), vec![]);

        Self {
            history: default_hmap_keys,
            last_query: Default::default(),
            logger: Default::default(),
        }
    }
}

#[cfg(test)]
mod test_query {
    use crate::content_source::ContentSource;
    use chrono::Utc;

    use super::Query;

    #[test]
    fn test_create_filled_query() {
        let time = Utc::now().clone();

        let have = Query::new(ContentSource::docs(), "Macro Generics", time);
        let want = Query {
            target: ContentSource::docs(),
            query: "Macro Generics".to_string(),
            date: time,
        };

        assert_eq!(have, want)
    }
}

#[cfg(test)]
mod test_querys {
    use super::{Query, QueryStore};
    use crate::content_source::ContentSource;
    use chrono::Utc;

    #[test]
    fn test_query_default_values() {
        let temp = QueryStore::new();
        assert!(temp.history.keys().into_iter().all(|key| {
            *key == ContentSource::docs()
                || *key == ContentSource::crates()
                || *key == ContentSource::lib()
        }))
    }

    #[test]
    fn test_add_new_query() {
        let temp_date = Utc::now().clone();

        let mut query_store = QueryStore::new();
        let q = Query::new(ContentSource::default(), "Generics", temp_date);
        let q1 = Query::new(ContentSource::lib(), "Generics II", temp_date);
        let q2 = Query::new(ContentSource::crates(), "Generics III", temp_date);

        query_store.add_new_query(q.clone(), String::default());
        query_store.add_new_query(q1.clone(), String::default());
        query_store.add_new_query(q2.clone(), String::default());

        // The given key always exists
        let exists = query_store.history.get(&ContentSource::docs()).unwrap();
        let exists1 = query_store.history.get(&ContentSource::lib()).unwrap();
        let exists2 = query_store.history.get(&ContentSource::crates()).unwrap();

        assert!(exists.contains(&q));
        assert!(exists1.contains(&q1));
        assert!(exists2.contains(&q2))
    }
}
