use regex::Regex;

pub fn parse(name :String) -> (String, String) {
    lazy_static! {
        static ref RE_RELEASE_GROUP :Regex = Regex::new(r"- ?(?P<group>[^-]+(?:-=\{[^-]+-?$)?)$").unwrap();
    }

    let mut group :String = String::from("");
    for capture in RE_RELEASE_GROUP.captures_iter(&name) {
        group = capture["group"].to_string();
    }

    group = group.trim().to_string();
    group = group.trim_matches('.').to_string();

    (group, RE_RELEASE_GROUP.replace_all(&name, "").to_string())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_year() {
        let mut test_grid :HashMap<&str, &str> = HashMap::new();
        test_grid.insert("Movie.Name.2018 - ReleaseGroup", "ReleaseGroup");

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let group = parse(key.to_string()).0;
            println!("Expected value: {}, result: {}", val, group);

            assert!(group == val);
        }
    }
}
