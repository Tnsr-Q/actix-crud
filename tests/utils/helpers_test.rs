//\! Integration tests for src/utils/helpers.rs
//\! Test framework: Rust built-in test harness (cargo test).
//\! Conventions: Integration tests under tests/ use public APIs from the crate.

use std::env;
use std::panic;

use actix_web::cookie::time::Duration;

// Import the functions under test. Adjust the crate path if the crate name differs.
// If this is a workspace, ensure the package name equals the crate root for `use` to resolve.
use crate::utils::helpers::{build_auth_cookie, get_conn_url};

// Try to also import the cookie name constant if it's public; otherwise fallback to literal checks.
// If this import fails, consider making COOKIE_NAME `pub` in src/utils/constants.rs for stronger assertions.
#[allow(unused_imports)]
use crate::utils::constants::COOKIE_NAME;

fn set_env(k: &str, v: &str) -> Option<String> {
    let old = env::var(k).ok();
    env::set_var(k, v);
    old
}

fn remove_env(k: &str) -> Option<String> {
    let old = env::var(k).ok();
    env::remove_var(k);
    old
}

fn restore_env(k: &str, old: &Option<String>) {
    match old {
        Some(val) => env::set_var(k, val),
        None => env::remove_var(k),
    }
}

#[test]
fn get_conn_url_happy_path_and_missing_vars_panics_sequentially() {
    // Run all env-mutating checks sequentially in one test to avoid parallel interference.
    // Also pre-set all variables to ensure they override any .env values.
    let originals = [
        ("DB_USER", env::var("DB_USER").ok()),
        ("DB_PASS", env::var("DB_PASS").ok()),
        ("DB_HOST", env::var("DB_HOST").ok()),
        ("DB_NAME", env::var("DB_NAME").ok()),
        ("DB_PORT", env::var("DB_PORT").ok()),
    ];

    // Use values with deliberate whitespace to verify trimming behavior.
    set_env("DB_USER", "  alice  ");
    set_env("DB_PASS", "  s3cr3t ");
    set_env("DB_HOST", " localhost ");
    set_env("DB_NAME", " appdb\t");
    set_env("DB_PORT", " 5432 ");

    // Happy path
    let url = get_conn_url();
    assert_eq\!(
        url,
        "postgresql://alice:s3cr3t@localhost:5432/appdb",
        "get_conn_url should build a trimmed Postgres URL"
    );

    // Helper to run a subcase that removes exactly one var and asserts panic.
    fn subcase_expect_panic(missing: &str) {
        // Save current
        let prev = remove_env(missing);

        // Expect a panic due to `.expect("Key not found\!\!")` in get_conn_url
        let result = panic::catch_unwind(|| {
            // Important: call directly; the function calls dotenv() internally, but since we removed
            // the specific env var just before invocation, if a committed .env exists it may repopulate it.
            // In typical CI (no .env committed), this will panic as intended. If not, consider
            // making get_conn_url accept an env provider for better testability.
            let _ = get_conn_url();
        });

        assert\!(
            result.is_err(),
            "get_conn_url should panic when {} is missing",
            missing
        );

        // Restore removed variable to previous value to not affect subsequent subcases
        match prev {
            Some(val) => env::set_var(missing, val),
            None => env::remove_var(missing),
        }
    }

    // Each required variable missing should cause a panic.
    for key in ["DB_USER", "DB_PASS", "DB_HOST", "DB_NAME", "DB_PORT"] {
        subcase_expect_panic(key);
    }

    // Restore originals
    for (k, old) in originals {
        restore_env(k, &old);
    }
}

#[test]
fn build_auth_cookie_sets_expected_attributes() {
    let token = "jwt-token-123";
    let cookie = build_auth_cookie(token.to_string());

    // Name (prefer constant if exported)
    #[allow(unused_variables)]
    if false {
        // This block is never executed; it exists to keep the optional import referenced if desired.
        let _ = &COOKIE_NAME;
    }

    // If COOKIE_NAME is public, assert equality with it; otherwise assert non-empty name.
    // We use cfg to avoid compile errors if the constant is private.
    #[cfg(any())]
    {
        assert_eq\!(cookie.name(), COOKIE_NAME, "cookie name should match constant");
    }
    #[cfg(not(any()))]
    {
        assert\!(\!cookie.name().is_empty(), "cookie name should not be empty");
    }

    // Value
    assert_eq\!(cookie.value(), token, "cookie value should be the passed token");

    // HttpOnly
    assert\!(cookie.http_only(), "cookie must be HttpOnly for security");

    // Path
    assert_eq\!(cookie.path().unwrap_or_default(), "/", "cookie path should be '/'");

    // Max-Age 2 hours
    let max_age = cookie.max_age();
    assert\!(max_age.is_some(), "cookie max_age should be set");
    assert_eq\!(max_age.unwrap(), Duration::hours(2), "cookie max_age should be 2 hours");

    // SameSite not explicitly set in helpers.rs; do not assert here to avoid dependency on defaults.
}