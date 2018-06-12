extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod quality;
mod year;
mod episode;
mod release_type;

#[derive(Serialize, Debug, PartialEq)]
pub struct MediaInfo {
    season :i32,
    episode :i32,
    year :i32,
    quality :Option<quality::Quality>,
    release_type :Option<release_type::ReleaseType>,
}

pub fn parse(name :&str) -> MediaInfo {
    let (season_nb, episode_nb) = episode::parse(name);
    let quality = quality::parse(name);
    let year = year::parse(name);
    let release_type = release_type::parse(name);

    MediaInfo {
        season: season_nb,
        episode: episode_nb,
        year: year,
        quality: quality,
        release_type: release_type,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse() {


        let mut test_grid :HashMap<&str, MediaInfo> = HashMap::new();

        test_grid.insert("2047 - Sights of Death (2014) 720p BrRip x264 - YIFY", MediaInfo{
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::BluRayRip)
        });
        test_grid.insert("The Flash 2014 S01E04 HDTV x264-FUM[ettv]", MediaInfo{
            season: 1,
            episode: 4,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV)
        });

        for (key, val) in test_grid.iter() {
            println!("Test item: {}", key);
            let info = parse(key);

            assert_eq!(val, &info);
        }
    }
}
