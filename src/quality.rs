extern crate serde;

use regex::Regex;
use std::cmp;

#[derive(Debug, PartialEq)]
pub enum Quality {
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

pub fn parse(name :&str) -> Option<Quality> {
    lazy_static! {
        static ref RE_QUALITY :Regex = Regex::new(r"(?i)(?P<quality>\d{3,4})[pi]").unwrap();
        static ref RE_SCREEN_SIZE :Regex = Regex::new(r"(?i)\d{3,4}\s?x\s?(?P<size>\d{3,4})").unwrap();
    }

    let quality = RE_QUALITY.captures(name).map_or(0, |x| x["quality"].to_string().parse::<i32>().unwrap_or(0));
    let screen_size = RE_SCREEN_SIZE.captures(name).map_or(0, |x| x["size"].to_string().parse::<i32>().unwrap_or(0));
    let aggregated_quality :i32 = cmp::max(quality, screen_size);

    match aggregated_quality {
        480  => Some(Quality::Q480),
        576  => Some(Quality::Q576),
        720  => Some(Quality::Q720),
        900  => Some(Quality::Q900),
        1080 => Some(Quality::Q1080),
        1440 => Some(Quality::Q1440),
        2160 => Some(Quality::Q2160),
        2880 => Some(Quality::Q5K),
        4320 => Some(Quality::Q8K),
        8640 => Some(Quality::Q16K),
        _    => None,
    }

    // TODO: Handle aaaxbbb and XK qualities
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_quality() {
        let mut test_grid :HashMap<&str, Quality> = HashMap::new();
        test_grid.insert("480p"       , Quality::Q480);
        test_grid.insert("480px"      , Quality::Q480);
        test_grid.insert("480i"       , Quality::Q480);
        test_grid.insert("720x480"    , Quality::Q480);
        test_grid.insert("640x480"    , Quality::Q480);
        test_grid.insert("704x480"    , Quality::Q480);
        test_grid.insert("852x480"    , Quality::Q480);

        test_grid.insert("576p"       , Quality::Q576);
        test_grid.insert("576px"      , Quality::Q576);
        test_grid.insert("576i"       , Quality::Q576);
        test_grid.insert("480x576"    , Quality::Q576);
        test_grid.insert("544x576"    , Quality::Q576);
        test_grid.insert("704x576"    , Quality::Q576);
        test_grid.insert("720x576"    , Quality::Q576);
        test_grid.insert("768x576"    , Quality::Q576);

        test_grid.insert("720p"       , Quality::Q720);
        test_grid.insert("720i"       , Quality::Q720);
        test_grid.insert("720pHD"     , Quality::Q720);
        test_grid.insert("1280x720"   , Quality::Q720);
        test_grid.insert("1366x720"   , Quality::Q720);

        test_grid.insert("900p"       , Quality::Q900);
        test_grid.insert("900p"       , Quality::Q900);
        test_grid.insert("900i"       , Quality::Q900);
        test_grid.insert("900px"      , Quality::Q900);
        test_grid.insert("1600x900"   , Quality::Q900);

        test_grid.insert("1080p"      , Quality::Q1080);
        test_grid.insert("1080px"     , Quality::Q1080);
        test_grid.insert("1080pHD"    , Quality::Q1080);
        test_grid.insert("1080phd"    , Quality::Q1080);
        test_grid.insert("1080i"      , Quality::Q1080);
        test_grid.insert("1920x1080"  , Quality::Q1080);
        test_grid.insert("1280x1080"  , Quality::Q1080);
        test_grid.insert("2048x1080"  , Quality::Q1080);
        test_grid.insert("2560x1080"  , Quality::Q1080);

        test_grid.insert("1440p"      , Quality::Q1440);
        test_grid.insert("1440px"     , Quality::Q1440);
        test_grid.insert("1440i"      , Quality::Q1440);
        test_grid.insert("2560x1440"  , Quality::Q1440);
        test_grid.insert("3440x1440"  , Quality::Q1440);

        test_grid.insert("2160p"      , Quality::Q2160);
        test_grid.insert("2160i"      , Quality::Q2160);
        test_grid.insert("2160px"     , Quality::Q2160);
        test_grid.insert("3840x2160"  , Quality::Q2160);
        test_grid.insert("4096x2160"  , Quality::Q2160);

        test_grid.insert("5120x2880"  , Quality::Q5K);

        test_grid.insert("4320p"      , Quality::Q8K);
        test_grid.insert("4320px"     , Quality::Q8K);
        test_grid.insert("4320i"      , Quality::Q8K);
        test_grid.insert("7680x4320"  , Quality::Q8K);

        test_grid.insert("15360x8640" , Quality::Q16K);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let quality = super::parse(key).unwrap();

            assert!(val == quality);
        }
    }
}
