use regex::Regex;
pub fn is_valid_html(source_code: &str) -> bool {
    let pattern = r"(?s)^<(\w+).*?>.*$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(source_code.trim())
}