use leptos_router::params::ParamsMap;

pub const BASE: &str = "/patterns/resource-explorer";

pub fn status_filter(query: &ParamsMap) -> &'static str {
    crate::todos::TodoStatus::parse(query.get_str("status").unwrap_or("all"))
        .unwrap_or(crate::todos::TodoStatus::All)
        .as_str()
}

pub fn status_label(completed: bool) -> &'static str {
    if completed {
        "Completed"
    } else {
        "Open"
    }
}

/// Preserve only collection state. Clone the parsed map to avoid decoding a
/// literal percent escape a second time when building detail/back links.
pub fn filter_suffix(query: &ParamsMap) -> String {
    let mut filters = query.clone();
    let keys = query
        .latest_values()
        .map(|(key, _)| key.to_string())
        .collect::<Vec<_>>();
    for key in keys {
        if key != "q" && key != "status" {
            filters.remove(&key);
        }
    }
    filters.to_query_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detail_links_preserve_encoded_search_and_drop_non_filter_state() {
        let query = ParamsMap::from_iter([
            ("q", "C%2B%2B%20%26%2050%2525"),
            ("status", "open"),
            ("draft", "must-not-travel"),
        ]);
        assert_eq!(
            filter_suffix(&query),
            "?q=C%2B%2B%20%26%2050%2525&status=open"
        );
        assert_eq!(status_filter(&query), "open");
    }

    #[test]
    fn absent_or_unknown_status_means_all() {
        assert_eq!(filter_suffix(&ParamsMap::new()), "");
        assert_eq!(status_filter(&ParamsMap::new()), "all");
        assert_eq!(
            status_filter(&ParamsMap::from_iter([("status", "unknown")])),
            "all"
        );
    }
}
