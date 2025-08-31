
//! # rust_mini_grep
//! 
//! A simple grep-like text search library for Rust.
//!
//! This crate provides basic text searching functionality with support for both
//! case-sensitive and case-insensitive searches. 
//!
//! # features
//!
//! - Case-sensitive text searching
//! - Case-insensitive text searching


/// Search for lines that contain the given query, **case-sensitive**.
///
/// # Arguments
///
/// * `query` - The substring to search for.
/// * `contents` - The text in which to search.
///
/// # Returns
///
/// A vector of string slices (`&str`) containing the lines from `contents`
/// that match the query.
///
/// # Examples
///
/// ```
/// use rust_mini_grep::search_case_sensitive;
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// let result = search_case_sensitive(query, contents);
/// assert_eq!(result, vec!["safe, fast, productive."]);
/// ```
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search for lines that contain the given query, **case-insensitive**.
///
/// # Arguments
///
/// * `query` - The substring to search for.
/// * `contents` - The text in which to search.
///
/// # Returns
///
/// A vector of string slices (`&str`) containing the lines from `contents`
/// that match the query, ignoring case differences.
///
/// # Examples
///
/// ```
/// use rust_mini_grep::search_case_insensitive;
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.";
///
/// let result = search_case_insensitive(query, contents);
/// assert_eq!(result, vec!["Rust:"]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
