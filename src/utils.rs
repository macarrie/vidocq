use regex::Regex;

pub fn find_and_strip(name: &str, regex_table: Vec<Regex>) -> (bool, String) {
    for reg in regex_table {
        if reg.is_match(name) {
            let stripped = &reg.replace_all(name, "");
            return (true, stripped.to_string());
        }
    }
    (false, name.to_string())
}
