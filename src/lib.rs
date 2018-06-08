extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate matches;

use regex::Regex;

#[derive(Serialize)]
pub struct MediaInfo {
    season :i32,
    episode :i32,
    year :i32,
    quality :Option<Quality>,
}

#[derive(Debug)]
enum Quality {
    Q480,
    Q576,
    Q720,
    Q900,
    Q1080,
    Q1440,
    Q2160,
    Q5K,
    Q8K,
    Q16K,
}

impl serde::Serialize for Quality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                Quality::Q480  => serializer.serialize_unit_variant("Quality", 0, "480p"),
                Quality::Q576  => serializer.serialize_unit_variant("Quality", 0, "576p"),
                Quality::Q720  => serializer.serialize_unit_variant("Quality", 0, "720p"),
                Quality::Q900  => serializer.serialize_unit_variant("Quality", 0, "900p"),
                Quality::Q1080 => serializer.serialize_unit_variant("Quality", 0, "1080p"),
                Quality::Q1440 => serializer.serialize_unit_variant("Quality", 0, "1440p"),
                Quality::Q2160 => serializer.serialize_unit_variant("Quality", 0, "2160p"),
                Quality::Q5K   => serializer.serialize_unit_variant("Quality", 0, "5K"),
                Quality::Q8K   => serializer.serialize_unit_variant("Quality", 0, "8K"),
                Quality::Q16K  => serializer.serialize_unit_variant("Quality", 0, "16K"),
            }
        }
}

fn parse_episode(name :&str) -> (i32, i32) {
    lazy_static! {
        static ref RE_SEASON :Regex = Regex::new(r"(?i)(s)?(?P<season>\d{1,2})[ex]").unwrap();
        static ref RE_EPISODE :Regex = Regex::new(r"(?i)[ex]p?(?P<episode>\d{1,2})").unwrap();
    }

    let season :i32 = RE_SEASON.captures(name)
        .map_or(0, |x| x["season"].to_string().parse::<i32>().unwrap_or(0));

    let episode :i32 = RE_EPISODE.captures(name)
        .map_or(0, |x| x["episode"].to_string().parse::<i32>().unwrap_or(0));

    (season, episode)
}

fn parse_quality(name :&str) -> Option<Quality> {
    lazy_static! {
        static ref RE_QUALITY :Regex = Regex::new(r"(?i)(?P<quality>\d{3,4})[pi]").unwrap();
    }

    match RE_QUALITY.captures(name).map_or(0, |x| x["quality"].to_string().parse::<i32>().unwrap_or(0)) {
        480 => Some(Quality::Q480),
        576  => Some(Quality::Q576),
        720  => Some(Quality::Q720),
        900  => Some(Quality::Q900),
        1080 => Some(Quality::Q1080),
        1440 => Some(Quality::Q1440),
        2160 => Some(Quality::Q2160),
        4320 => Some(Quality::Q8K),
        _ => None,
    }

    // TODO: Handle aaaxbbb and XK qualities
}

fn parse_year(name :&str) -> i32 {
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
    use super::*;
    extern crate matches;

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
        test_grid.insert("2011 2013 (2012) (2015)", 2012);
        // If no marked year and multiple unmarked year, second unmarked year is taken
        test_grid.insert("2012 2009 S01E02 2015", 2009);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let year = super::parse_year(key);
            println!("Expected value: {}, result: {}", val, year);

            assert!(year == val);
        }
    }

    #[test]
    fn test_parse_quality() {
        let mut test_grid :HashMap<&str, Quality> = HashMap::new();
        test_grid.insert("480p"       , Quality::Q480);
        test_grid.insert("480px"      , Quality::Q480);
        test_grid.insert("480i"       , Quality::Q480);
        //test_grid.insert("720x480"    , Quality::Q480);
        //test_grid.insert("640x480"    , Quality::Q480);
        //test_grid.insert("704x480"    , Quality::Q480);
        //test_grid.insert("852x480"    , Quality::Q480);

        test_grid.insert("576p"       , Quality::Q576);
        test_grid.insert("576px"      , Quality::Q576);
        test_grid.insert("576i"       , Quality::Q576);
        //test_grid.insert("480x576"    , Quality::Q576);
        //test_grid.insert("544x576"    , Quality::Q576);
        //test_grid.insert("704x576"    , Quality::Q576);
        //test_grid.insert("720x576"    , Quality::Q576);
        //test_grid.insert("768x576"    , Quality::Q576);

        test_grid.insert("720p"       , Quality::Q720);
        test_grid.insert("720i"       , Quality::Q720);
        //test_grid.insert("720hd"      , Quality::Q720);
        test_grid.insert("720pHD"     , Quality::Q720);
        //test_grid.insert("1280x720"   , Quality::Q720);
        //test_grid.insert("1366x720"   , Quality::Q720);

        test_grid.insert("900p"       , Quality::Q900);
        test_grid.insert("900p"       , Quality::Q900);
        test_grid.insert("900i"       , Quality::Q900);
        test_grid.insert("900px"      , Quality::Q900);
        //test_grid.insert("1600x900"   , Quality::Q900);

        test_grid.insert("1080p"      , Quality::Q1080);
        test_grid.insert("1080px"     , Quality::Q1080);
        test_grid.insert("1080pHD"    , Quality::Q1080);
        test_grid.insert("1080phd"    , Quality::Q1080);
        test_grid.insert("1080i"      , Quality::Q1080);
        //test_grid.insert("1920x1080"  , Quality::Q1080);
        //test_grid.insert("1280x1080"  , Quality::Q1080);
        //test_grid.insert("2048x1080"  , Quality::Q1080);
        //test_grid.insert("2560x1080"  , Quality::Q1080);

        test_grid.insert("1440p"      , Quality::Q1440);
        test_grid.insert("1440px"     , Quality::Q1440);
        test_grid.insert("1440i"      , Quality::Q1440);
        //test_grid.insert("2560x1440"  , Quality::Q1440);
        //test_grid.insert("3440x1440"  , Quality::Q1440);

        test_grid.insert("2160p"      , Quality::Q2160);
        test_grid.insert("2160i"      , Quality::Q2160);
        test_grid.insert("2160px"     , Quality::Q2160);
        //test_grid.insert("3840x2160"  , Quality::Q2160);
        //test_grid.insert("4096x2160"  , Quality::Q2160);

        //test_grid.insert("5120x2880"  , Quality::Q5K);

        test_grid.insert("4320p"      , Quality::Q8K);
        test_grid.insert("4320px"     , Quality::Q8K);
        test_grid.insert("4320i"      , Quality::Q8K);
        //test_grid.insert("7680x4320"  , Quality::Q8K);

        //test_grid.insert("15360x8640" , Quality::Q16K);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let quality = super::parse_quality(key).unwrap();

            assert!(matches!(val, quality));
        }
    }
}
