use regex::{Regex, Captures};

pub fn regex_strip(expr: &str, line: &str) -> String {
    (*Regex::new(expr).unwrap().replace_all(line, "")).into()
}
pub fn regex_match(expr: &str, line: &str) -> bool {
    Regex::new(expr).unwrap().is_match(line)
}
pub fn regex_matches<'t>(expr: &str, line: &'t str) -> Vec<Captures<'t>> {
    if regex_match(expr, line) {
        Regex::new(expr).unwrap().captures_iter(line).collect()
    } else {
        Vec::new()
    }
}
pub fn re_matches<'t>(re: &Regex, line: &'t str) -> Vec<Captures<'t>> {
    if re.is_match(line) {
        re.captures_iter(line).collect()
    } else {
        Vec::new()
    }
}
pub fn re_match_names<'t>(re: &'t Regex) -> Vec<&'t str> {
    re.capture_names()
        .filter_map(|s| s)
        .collect::<Vec<&str>>()
        .clone()
}
