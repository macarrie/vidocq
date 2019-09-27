use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

use super::episode;
use super::year;

fn parse_title_from_filename(name: &str) -> String {
    lazy_static! {
        static ref RE_SQUARE_BLOCKS: Regex = Regex::new(r"(?i)\[.*\]").unwrap();
        static ref RE_PARENTHESIS: Regex = Regex::new(r"(?i)\(.*\)").unwrap();
        static ref RE_DELIMITERS: Regex = Regex::new(r"(?i)[_\.\(\)\[\]]").unwrap();
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

    let min_offset: usize = offsets.into_iter().filter(|x| *x > 0).min().unwrap_or(0);

    //    let mut work_str = name;
    let mut work_str = name;

    if min_offset != 0 {
        work_str = &work_str[..min_offset];
    }
    let mut file_path: Vec<&OsStr> = Path::new(work_str).iter().collect();
    let filename_from_path = file_path.pop().clone().unwrap().to_str().unwrap();
    work_str = filename_from_path;

    //Remove square brackets blocks
    let strip_blocks = RE_SQUARE_BLOCKS.replace_all(&work_str, "").to_string();

    //Remove parenthesis blocks
    let strip_parenthesis = RE_PARENTHESIS.replace_all(&strip_blocks, "").to_string();

    //Replace delimiters with spaces
    let strip_delimiters = RE_DELIMITERS
        .replace_all(&strip_parenthesis, " ")
        .to_string();

    strip_delimiters.trim().to_string()
}

pub fn parse(name: &str) -> String {
    let mut file_path: Vec<&OsStr> = Path::new(name).iter().collect();
    let filename_from_path = file_path.pop().clone().unwrap().to_str().unwrap();

    if file_path.len() >= 3 {
        let title_part_from_filepath = file_path[file_path.len() - 2].to_str().unwrap();

        return parse_title_from_filename(&title_part_from_filepath);
    }

    return parse_title_from_filename(filename_from_path);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_parse_title() {
        let mut test_grid: HashMap<&str, &str> = HashMap::new();

        test_grid.insert("Show.title.S01E01.HDTV-Blablabla", "Show title");
        test_grid.insert("Show.title.2018.S01E01.HDTV-Blablabla", "Show title");
        test_grid.insert("Movie.title.2018.HDTV-Blablabla", "Movie title");

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let title = super::parse(key);

            assert_eq!(val, title);
        }
    }
}
