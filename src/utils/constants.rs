use actix_web::http::{header, Method};

pub const METHODS: &[Method] = &[
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::DELETE,
    Method::PATCH,
];

pub const HEADERS: &[header::HeaderName] =
    &[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE];

pub const COOKIE_NAME: &str = "OKIJ";

#[cfg(test)]
mod tests_constants {
    use super::*;
    use actix_web::http::{header, Method};
    use std::collections::HashSet;

    #[test]
    fn methods_list_matches_expected_and_ordered() {
        // Exact content and ordering
        assert_eq\!(
            METHODS,
            &[Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH]
        );
        // Length sanity check
        assert_eq\!(METHODS.len(), 5);
    }

    #[test]
    fn methods_are_unique_and_exclude_disallowed() {
        // Uniqueness by semantic identity (method string)
        let unique: HashSet<_> = METHODS.iter().map(|m| m.as_str()).collect();
        assert_eq\!(unique.len(), METHODS.len(), "METHODS contains duplicates");

        // Ensure unexpected/unsafe methods are not permitted
        for disallowed in [Method::HEAD, Method::OPTIONS, Method::CONNECT, Method::TRACE] {
            assert\!(
                \!METHODS.contains(&disallowed),
                "Unexpected disallowed method present: {}",
                disallowed
            );
        }
    }

    #[test]
    fn headers_match_expected_and_unique() {
        // Exact content and ordering
        assert_eq\!(
            HEADERS,
            &[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]
        );
        // Uniqueness via canonical string form
        let unique: HashSet<_> = HEADERS.iter().map(|h| h.as_str()).collect();
        assert_eq\!(unique.len(), HEADERS.len(), "HEADERS contains duplicates");
        // Length sanity check
        assert_eq\!(HEADERS.len(), 3);
    }

    #[test]
    fn cookie_name_is_expected_and_well_formed() {
        assert_eq\!(COOKIE_NAME, "OKIJ");
        assert\!(\!COOKIE_NAME.is_empty(), "COOKIE_NAME should not be empty");
        assert\!(COOKIE_NAME.is_ascii(), "COOKIE_NAME should be ASCII");
        assert\!(
            \!COOKIE_NAME.chars().any(|c| c.is_whitespace()),
            "COOKIE_NAME should not contain whitespace"
        );
        // Optional: keep cookie token short and simple
        assert\!(COOKIE_NAME.len() <= 16, "COOKIE_NAME should be reasonably short");
    }
}
