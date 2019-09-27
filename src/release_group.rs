use regex::Regex;

use super::episode;
use super::year;

pub fn parse(name: &str) -> (String, String) {
    lazy_static! {
        static ref RE_RELEASE_GROUP: Regex =
            Regex::new(r"- ?(?P<group>[^-]+(?:-=\{[^-]+-?$)?)$").unwrap();
        static ref RE_RELEASE_GROUP_BEGINNING: Regex = Regex::new(r"^\[(?P<group>\w*)\]").unwrap();
    }

    let mut offsets: Vec<usize> = Vec::new();

    //Find year string in name. Everything after it is usually not part of the title
    let year_str: String = year::parse(name).to_string();
    let year_offset: usize = if year_str != "0" {
        name.rfind(&year_str).unwrap_or(0)
    } else {
        0
    } as usize;

    offsets.push(year_offset);

    //Find season/episode number in name. Everything after it is usually not part of the title
    offsets.push(
        episode::RE_SEASON_AND_EPISODE
            .captures(name)
            .map_or(0, |m| m.get(0).map_or(0, |c| c.start())),
    );
    offsets.push(
        episode::RE_SEASON_AND_EPISODE_SEPARATED
            .captures(name)
            .map_or(0, |m| m.get(0).map_or(0, |c| c.start())),
    );
    offsets.push(
        episode::RE_SEASON
            .captures(name)
            .map_or(0, |m| m.get(0).map_or(0, |c| c.start())),
    );
    offsets.push(
        episode::RE_EPISODE
            .captures(name)
            .map_or(0, |m| m.get(0).map_or(0, |c| c.start())),
    );

    let max_offset: usize = offsets.into_iter().filter(|x| *x > 0).max().unwrap_or(0);

    //    let mut work_str = name;
    let mut work_str = name;

    if max_offset != 0 {
        work_str = &work_str[max_offset..];
    }

    let mut group: String = String::from("");
    for capture in RE_RELEASE_GROUP.captures_iter(&work_str) {
        group = capture["group"].to_string();
    }

    group = group.trim().to_string();
    group = group.trim_matches('.').to_string();

    let mut group_at_beginning: String = String::from("");
    for capture in RE_RELEASE_GROUP_BEGINNING.captures_iter(&name) {
        group_at_beginning = capture["group"].to_string();
    }

    group_at_beginning = group_at_beginning.trim().to_string();
    group_at_beginning = group_at_beginning.trim_matches('.').to_string();

    if group_at_beginning != "" {
        return (
            group_at_beginning,
            RE_RELEASE_GROUP_BEGINNING
                .replace_all(&name, "")
                .to_string(),
        );
    }

    (group, RE_RELEASE_GROUP.replace_all(&name, "").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_year() {
        let mut test_grid: HashMap<&str, &str> = HashMap::new();
        test_grid.insert("Movie.Name.2018 - ReleaseGroup", "ReleaseGroup");
        test_grid.insert("[ReleaseGroup] Movie.Name.2018", "ReleaseGroup");

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let group = parse(key).0;
            println!("Expected value: {}, result: {}", val, group);

            assert!(group == val);
        }
    }
}
