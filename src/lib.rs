use pgrx::prelude::*;
extern crate regex;
use regex::Regex;

pgrx::pg_module_magic!();

#[pg_extern]
fn repl_str(str_in: &str, str_old: &str, str_new: &str) -> Result<String, regex::Error> {
    let re = Regex::new(str_old)?;
    let result = re.replace_all(str_in, str_new).to_string();
    Ok(result)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_substring() {
        assert_eq!("hello", crate::repl_str("Hello", "H", "h").unwrap().as_str());
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
