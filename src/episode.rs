use regex::Regex;

lazy_static! {
    pub static ref RE_SEASON_AND_EPISODE: Regex =
        Regex::new(r"(?i)(s)?(?P<season>\d{1,3})[ex]p?(?P<episode>\d{1,3})").unwrap();
    pub static ref RE_SEASON_AND_EPISODE_SEPARATED: Regex =
        Regex::new(r"(?i)(s)?(?P<season>\d{1,3})\s?[-:]\s?(?P<episode>\d{1,3})").unwrap();
}

pub fn parse(name: String) -> (i32, i32, String) {
    let (season, episode): (i32, i32) = RE_SEASON_AND_EPISODE.captures(&name).map_or((0, 0), |x| {
        let season = x["season"].to_string().parse::<i32>().unwrap_or(0);
        let episode = x["episode"].to_string().parse::<i32>().unwrap_or(0);

        (season, episode)
    });

    let (season_sep, episode_sep): (i32, i32) = RE_SEASON_AND_EPISODE_SEPARATED
        .captures(&name)
        .map_or((0, 0), |x| {
            let season = x["season"].to_string().parse::<i32>().unwrap_or(0);
            let episode = x["episode"].to_string().parse::<i32>().unwrap_or(0);

            (season, episode)
        });

    if season_sep != 0 && episode_sep != 0 {
        return (season_sep, episode_sep, name);
    }

    (season, episode, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_episode() {
        let testlist = [
            "+2x5",
            "+2X5",
            "2x05",
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
            "-s002e005",
            "S2 - 05",
            "S02-005",
            "-2x05",
        ];

        for s in testlist.iter() {
            println!("Test item: {}", s);
            let info = parse(s.to_string());
            assert_eq!(info.0, 2);
            assert_eq!(info.1, 5);
        }
    }
}
