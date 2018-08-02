use regex::Regex;

use super::year;

pub fn parse(name :&str) -> String {
    lazy_static! {
        static ref RE_SQUARE_BLOCKS :Regex = Regex::new(r"(?i)\[.*\]").unwrap();
        static ref RE_PARENTHESIS :Regex = Regex::new(r"(?i)\(.*\)").unwrap();
        static ref RE_DELIMITERS :Regex = Regex::new(r"(?i)[_\.\(\)\[\]]").unwrap();
    }

    let year_str :String = year::parse(name).to_string();
    println!("year: {}", year_str);

    let year_str_position= name.rfind(&year_str).unwrap_or(0);
    println!("year position: {}", year_str_position);

    let mut work_str = name;
    if year_str_position != 0 {
        work_str = &work_str[..year_str_position];
    }
    println!("Cleaned year: {}", work_str);

    //Remove square brackets blocks
    let strip_blocks = RE_SQUARE_BLOCKS.replace_all(&work_str, "").to_string();
    println!("Block stripped: {}", strip_blocks);

    //Remove parenthesis blocks
    let strip_parenthesis = RE_PARENTHESIS.replace_all(&strip_blocks, "").to_string();
    println!("parenthesis stripped: {}", strip_parenthesis);

    //Replace delimiters with spaces
    let strip_delimiters = RE_DELIMITERS.replace_all(&strip_parenthesis, " ").to_string();
    println!("delimiters stripped: {}", strip_delimiters);

    strip_delimiters.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_title() {
        let mut test_grid :HashMap<&str, &str> = HashMap::new();

        test_grid.insert("Ant-Man.2015.3D.1080p.BRRip.Half-SBS.x264.AAC-m2g", "Ant-Man");
        test_grid.insert("Ice.Age.Collision.Course.2016.READNFO.720p.HDRIP.X264.AC3.TiTAN", "Ice Age Collision Course");
        test_grid.insert("Red.Sonja.Queen.Of.Plagues.2016.BDRip.x264-W4F[PRiME]", "Red Sonja Queen Of Plagues");
        test_grid.insert("The Purge: Election Year (2016) HC - 720p HDRiP - 900MB - ShAaNi", "The Purge: Election Year");
        test_grid.insert("War Dogs (2016) HDTS 600MB - NBY", "War Dogs");


        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let title = super::parse(key);

            assert!(val == title);
        }
    }
}
