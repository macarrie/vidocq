extern crate serde_json;
use regex::Regex;

fn parse_episode(name :&str) -> (i32, i32) {
    let season :i32;
    let episode :i32;

    lazy_static! {
        static ref RE_SEASON :Regex = Regex::new(r"(?i)(s)?(?P<season>\d{1,2})[ex]").unwrap();
        static ref RE_EPISODE :Regex = Regex::new(r"(?i)[ex](?P<episode>\d{1,2})").unwrap();
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

pub fn parse(name :&str) -> serde_json::Value {
    println!("Torrent name: {}", name);

    let (season_nb, episode_nb) = parse_episode(name);
    let quality = parse_quality(name);
    let year = parse_year(name);

    let json = json!({
        "season": season_nb,
        "episode": episode_nb,
        "quality": quality,
        "year": year
    });

    println!("Torrent info: {}", json.to_string());
    json
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

