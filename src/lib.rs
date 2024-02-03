extern crate regex;

use std::borrow::Cow;

use pgrx::prelude::*;
use regex::Regex;

pgrx::pg_module_magic!();

#[pg_extern]
fn repl_str(str_in: &str, str_old: &str, str_new: &str) -> String {
    let re = Regex::new(str_old).expect("Failed to create regex.");
    let result: Cow<str> = re.replace_all(str_in, str_new);
    result.to_string()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_substring() {
        assert_eq!("hello", crate::repl_str("Hello", "H", "h"));
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
