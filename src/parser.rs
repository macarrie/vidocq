use regex::Regex;

#[derive(Serialize)]
pub struct MediaInfo {
    season :i32,
    episode :i32,
    year :i32,
    quality :i32,
}

fn parse_episode(name :&str) -> (i32, i32) {
    let season :i32;
    let episode :i32;

    lazy_static! {
        static ref RE_SEASON :Regex = Regex::new(r"(?i)(s)?(?P<season>\d{1,2})[ex]").unwrap();
        static ref RE_EPISODE :Regex = Regex::new(r"(?i)[ex]p?(?P<episode>\d{1,2})").unwrap();
    }

    let season_search = RE_SEASON.captures(name);
    match season_search {
        Some(x) => {
            let season_nb :i32 = x["season"].to_string().parse().unwrap();
            season = season_nb;
        },
        None => {
            season = 0;
        }
    }

    let episode_search = RE_EPISODE.captures(name);
    match episode_search {
        Some(x) => {
            let episode_nb :i32 = x["episode"].to_string().parse().unwrap();
            episode = episode_nb;
        },
        None => {
            episode = 0;
        }
    }

    (season, episode)
}

fn parse_quality(name :&str) -> i32 {
    lazy_static! {
        static ref RE_QUALITY :Regex = Regex::new(r"(?i)(?P<quality>\d{3,4})[pi]").unwrap();
    }

    let quality_search = RE_QUALITY.captures(name);
    match quality_search {
        Some(x) => {
            let quality :i32 = x["quality"].to_string().parse().unwrap();
            quality
        },
        None => {
            0
        }
    }
}

fn parse_year(name :&str) -> i32 {
    lazy_static! {
        static ref RE_YEAR :Regex = Regex::new(r"(?P<year>19\d{2}|20\d{2})").unwrap();
    }

    let year_search = RE_YEAR.captures(name);
    match year_search {
        Some(x) => {
            let year :i32 = x["year"].to_string().parse().unwrap();
            year
        },
        None => {
            0
        }
    }
}

pub fn parse(name :&str) -> MediaInfo {
    println!("Torrent name: {}", name);

    let (season_nb, episode_nb) = parse_episode(name);
    let quality = parse_quality(name);
    let year = parse_year(name);

    MediaInfo {
        season: season_nb,
        episode: episode_nb,
        year: year,
        quality: quality,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_parse_episode() {
        let testlist = [ "+2x5",
            "+2X5",
            "+02x05",
            "+2X05",
            "+02x5",
            "S02E05",
            "s02e05",
            "s02e5",
            "s2e05",
            "s02ep05",
            "s2EP5",
            "-s02e05",
            "-2x05"];

        for s in testlist.iter() {
            println!("Test item: {}", s);
            let info = super::parse_episode(s);
            assert!(info.0 == 2);
            assert!(info.1 == 5);
        }
    }

    #[test]
    fn test_parse_year() {
        let mut test_grid :HashMap<&str, i32> = HashMap::new();
        test_grid.insert("1919", 1919);
        test_grid.insert("2030", 2030);
        test_grid.insert("2029", 2029);
        test_grid.insert("(1920)", 1920);
        test_grid.insert("2012", 2012);
        // First marked year is taken
        //test_grid.insert("2011 2013 (2012) (2015)", 2012);
        // If no marked year and multiple unmarked year, second unmarked year is taken
        //test_grid.insert("2012 2009 S01E02 2015", 2009);

        for (key, val) in test_grid {
            let year = super::parse_year(key);

            println!("Test item: {}", key);
            println!("Expected value: {}, result: {}", val, year);

            assert!(year == val);
        }
    }
}

