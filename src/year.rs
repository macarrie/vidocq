use regex::Regex;

pub fn parse(name :&str) -> i32 {
    lazy_static! {
        static ref RE_UNMARKED_YEAR :Regex = Regex::new(r"(?P<year>19\d{2}|20\d{2})").unwrap();
        static ref RE_MARKED_YEAR :Regex = Regex::new(r"\((?P<year>19\d{2}|20\d{2})\)").unwrap();
    }

    let unmarked_years :Vec<_> = RE_UNMARKED_YEAR.captures_iter(name).map(|m| m.get(1).unwrap().as_str()).collect();
    let marked_years :Vec<_> = RE_MARKED_YEAR.captures_iter(name).map(|m| m.get(1).unwrap().as_str()).collect();

    if marked_years.len() > 0 {
        return marked_years[0].parse::<i32>().unwrap_or(0);
    }

    if unmarked_years.len() > 1 {
        return unmarked_years[1].parse::<i32>().unwrap_or(0);
    }

    unmarked_years.get(0).map_or(0, |y| y.parse::<i32>().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_year() {
        let mut test_grid :HashMap<&str, i32> = HashMap::new();
        test_grid.insert("1919", 1919);
        test_grid.insert("2030", 2030);
        test_grid.insert("2029", 2029);
        test_grid.insert("(1920)", 1920);
        test_grid.insert("2012", 2012);

        // First marked year is taken
        test_grid.insert("2011 2013 (2012) (2015)", 2012);
        // If no marked year and multiple unmarked year, second unmarked year is taken
        test_grid.insert("2012 2009 S01E02 2015", 2009);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let year = parse(key);
            println!("Expected value: {}, result: {}", val, year);

            assert_eq!(year, val);
        }
    }
}
