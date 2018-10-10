extern crate serde;

use super::utils;

use regex::Regex;


#[derive(Debug, PartialEq)]
pub enum Container {
    AVI,
    Matroska,
    MP4,
    MXF,
    Ogg,
    QuickTime,
}

impl serde::Serialize for Container {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                Container::AVI       => serializer.serialize_unit_variant("Container", 0, "avi"),
                Container::Matroska  => serializer.serialize_unit_variant("Container", 0, "mkv"),
                Container::MP4       => serializer.serialize_unit_variant("Container", 0, "mp4"),
                Container::MXF       => serializer.serialize_unit_variant("Container", 0, "mxf"),
                Container::Ogg       => serializer.serialize_unit_variant("Container", 0, "ogg"),
                Container::QuickTime => serializer.serialize_unit_variant("Container", 0, "quicktime"),
            }
        }
}


pub fn parse(name :String) -> (Option<Container>, String) {
    lazy_static! {
        static ref RE_AVI :Vec<Regex> = vec![
            Regex::new(r"(?i)avi").unwrap(),
        ];

        static ref RE_MKV :Vec<Regex> = vec![
            Regex::new(r"(?i)mk[vas]").unwrap(),
            Regex::new(r"(?i)mk3d").unwrap(),
            Regex::new(r"(?i)webm").unwrap(),
        ];

        static ref RE_MP4 :Vec<Regex> = vec![
            Regex::new(r"(?i)mp4|m4[abprv]").unwrap(),
        ];

        static ref RE_MXF :Vec<Regex> = vec![
            Regex::new(r"(?i)mxf").unwrap(),
        ];

        static ref RE_OGG :Vec<Regex> = vec![
            Regex::new(r"(?i)og[gvaxm]").unwrap(),
            Regex::new(r"(?i)opus").unwrap(),
            Regex::new(r"(?i)spx").unwrap(),
        ];

        static ref RE_QUICKTIME :Vec<Regex> = vec![
            Regex::new(r"(?i)mov").unwrap(),
            Regex::new(r"(?i)qt").unwrap(),
        ];
    }

    let mut matched_container :Option<Container> = None;
    let original_name = name.clone();

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_AVI.to_vec());
    if matched {
        matched_container = Some(Container::AVI);

        return (matched_container, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_MKV.to_vec());
    if matched {
        matched_container = Some(Container::Matroska);

        return (matched_container, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_MP4.to_vec());
    if matched {
        matched_container = Some(Container::MP4);

        return (matched_container, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_MXF.to_vec());
    if matched {
        matched_container = Some(Container::MXF);

        return (matched_container, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_OGG.to_vec());
    if matched {
        matched_container = Some(Container::Ogg);

        return (matched_container, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_QUICKTIME.to_vec());
    if matched {
        matched_container = Some(Container::QuickTime);

        return (matched_container, stripped_name);
    }

    (matched_container, original_name)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_video_codec() {
        let mut test_grid :HashMap<&str, Container> = HashMap::new();

        test_grid.insert("avi", Container::AVI);

        test_grid.insert("mkv",  Container::Matroska);
        test_grid.insert("mka",  Container::Matroska);
        test_grid.insert("mks",  Container::Matroska);
        test_grid.insert("mk3d", Container::Matroska);
        test_grid.insert("webm", Container::Matroska);

        test_grid.insert("mp4", Container::MP4);
        test_grid.insert("m4a", Container::MP4);
        test_grid.insert("m4b", Container::MP4);
        test_grid.insert("m4p", Container::MP4);
        test_grid.insert("m4r", Container::MP4);
        test_grid.insert("m4v", Container::MP4);

        test_grid.insert("mxf", Container::MXF);

        test_grid.insert("ogg", Container::Ogg);

        test_grid.insert("mov", Container::QuickTime);
        test_grid.insert("qt",  Container::QuickTime);


        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let container = super::parse(key.to_string()).0.unwrap();

            assert!(val == container);
        }
    }
}

