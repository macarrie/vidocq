use regex::Regex;

lazy_static! {
    pub static ref RE_SEASON :Regex = Regex::new(r"(?i)(s)?(?P<season>\d{1,3})[ex]").unwrap();
    static ref RE_EPISODE :Regex = Regex::new(r"(?i)\d[ex]p?(?P<episode>\d{1,3})").unwrap();
}

pub fn parse(name :&str) -> (i32, i32) {
    let season :i32 = RE_SEASON.captures(name)
        .map_or(0, |x| x["season"].to_string().parse::<i32>().unwrap_or(0));

    let episode :i32 = RE_EPISODE.captures(name)
        .map_or(0, |x| x["episode"].to_string().parse::<i32>().unwrap_or(0));

    (season, episode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_episode() {
        let testlist = [ "+2x5",
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
        "-2x05"];

        for s in testlist.iter() {
            println!("Test item: {}", s);
            let info = parse(s);
            assert_eq!(info.0, 2);
            assert_eq!(info.1, 5);
        }
    }
}
