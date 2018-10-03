extern crate serde;

use super::utils;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum ReleaseType {
    Cam,
    Telesync,
    Telecine,
    Screener,
    DVDRip,
    HDTV,
    WEBDL,
    BluRayRip,
}

impl serde::Serialize for ReleaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                ReleaseType::Cam       => serializer.serialize_unit_variant("ReleaseType", 0, "cam"),
                ReleaseType::Telesync  => serializer.serialize_unit_variant("ReleaseType", 0, "telesync"),
                ReleaseType::Telecine  => serializer.serialize_unit_variant("ReleaseType", 0, "telecine"),
                ReleaseType::Screener  => serializer.serialize_unit_variant("ReleaseType", 0, "screener"),
                ReleaseType::DVDRip    => serializer.serialize_unit_variant("ReleaseType", 0, "dvdrip"),
                ReleaseType::HDTV      => serializer.serialize_unit_variant("ReleaseType", 0, "hdtv"),
                ReleaseType::WEBDL     => serializer.serialize_unit_variant("ReleaseType", 0, "webdl"),
                ReleaseType::BluRayRip => serializer.serialize_unit_variant("ReleaseType", 0, "blurayrip"),
            }
        }
}

pub fn parse(name :String) -> (Option<ReleaseType>, String) {
    lazy_static! {
        static ref RE_CAM :Vec<Regex> = vec![
            Regex::new(r"(?i)(HD)?CAM(RIP)?").unwrap(),
        ];

        static ref RE_TELESYNC :Vec<Regex> = vec![
            Regex::new(r"(HD)?TS").unwrap(),
            Regex::new(r"(?i)TELESYNC").unwrap(),
            Regex::new(r"(?i)PDVD").unwrap(),
            Regex::new(r"(?i)PreDVDRip").unwrap(),
        ];

        static ref RE_TELECINE :Vec<Regex> = vec![
            Regex::new(r"(?i)(HD)?TC").unwrap(),
            Regex::new(r"(?i)TELECINE").unwrap(),
        ];

        static ref RE_SCREENER :Vec<Regex> = vec![
            Regex::new(r"(?i)(DVD|BD)?SCR(EENER)?").unwrap(),
            Regex::new(r"(?i)DDC").unwrap(),
        ];

        static ref RE_DVDRIP :Vec<Regex> = vec![
            Regex::new(r"(?i)DVDR(IP)?").unwrap(),
            Regex::new(r"(?i)DVDMux").unwrap(),
            Regex::new(r"(?i)DVD-?(Full|\d{1,2})").unwrap(),
        ];

        static ref RE_HDTV :Vec<Regex> = vec![
            Regex::new(r"(HD|PD)TV").unwrap(),
            Regex::new(r"(?i)(HD|DS|SAT|DTH|DVB|TV|HDTV)Rip").unwrap(),
            Regex::new(r"DSR").unwrap(),
        ];

        static ref RE_WEBDL :Vec<Regex> = vec![
            Regex::new(r"(?i)WEB[-\s]?DL").unwrap(),
            Regex::new(r"(?i)WEB[-\s]?Rip").unwrap(),
            Regex::new(r"(?i)WEB[-\s]?Cap").unwrap(),
        ];

        static ref RE_BLURAY :Vec<Regex> = vec![
            Regex::new(r"(?i)Blu[-\s]?Ray").unwrap(),
            Regex::new(r"(?i)B[RD](Rip|MV|R|25|50|5|9)").unwrap(),
        ];
    }


    let mut matched_release_type :Option<ReleaseType> = None;
    let original_name = name.clone();

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_BLURAY.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::BluRayRip);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DVDRIP.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::DVDRip);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_WEBDL.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::WEBDL);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_HDTV.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::HDTV);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_SCREENER.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::Screener);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_TELECINE.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::Telecine);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_TELESYNC.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::Telesync);

        return (matched_release_type, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_CAM.to_vec());
    if matched {
        matched_release_type = Some(ReleaseType::Cam);

        return (matched_release_type, stripped_name);
    }

    (matched_release_type, original_name)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_release_type() {
        let mut test_grid :HashMap<&str, ReleaseType> = HashMap::new();
        test_grid.insert("Cam"           , ReleaseType::Cam);
        test_grid.insert("CAMRip"        , ReleaseType::Cam);
        test_grid.insert("CAM"           , ReleaseType::Cam);

        test_grid.insert("Telesync"      , ReleaseType::Telesync);
        test_grid.insert("TS"            , ReleaseType::Telesync);
        test_grid.insert("HDTS"          , ReleaseType::Telesync);
        test_grid.insert("TELESYNC"      , ReleaseType::Telesync);
        test_grid.insert("PDVD"          , ReleaseType::Telesync);

        test_grid.insert("Telecine"      , ReleaseType::Telecine);
        test_grid.insert("TC"            , ReleaseType::Telecine);
        test_grid.insert("HDTC"          , ReleaseType::Telecine);
        test_grid.insert("TELECINE"      , ReleaseType::Telecine);

        test_grid.insert("Screener"      , ReleaseType::Screener);
        test_grid.insert("SCR"           , ReleaseType::Screener);
        test_grid.insert("SCREENER"      , ReleaseType::Screener);
        test_grid.insert("DVDSCR"        , ReleaseType::Screener);
        test_grid.insert("DVDSCREENER"   , ReleaseType::Screener);
        test_grid.insert("BDSCR"         , ReleaseType::Screener);
        test_grid.insert("DDC"           , ReleaseType::Screener);

        test_grid.insert("DVDRip"        , ReleaseType::DVDRip);
        test_grid.insert("DVDRip"        , ReleaseType::DVDRip);
        test_grid.insert("DVDMux"        , ReleaseType::DVDRip);
        test_grid.insert("DVDR"          , ReleaseType::DVDRip);
        test_grid.insert("DVD-Full"      , ReleaseType::DVDRip);
        test_grid.insert("DVD-5"         , ReleaseType::DVDRip);
        test_grid.insert("DVD-9"         , ReleaseType::DVDRip);

        test_grid.insert("HDTV"          , ReleaseType::HDTV);
        test_grid.insert("DSR"           , ReleaseType::HDTV);
        test_grid.insert("DSRip"         , ReleaseType::HDTV);
        test_grid.insert("SATRip"        , ReleaseType::HDTV);
        test_grid.insert("DTHRip"        , ReleaseType::HDTV);
        test_grid.insert("DVBRip"        , ReleaseType::HDTV);
        test_grid.insert("HDTV"          , ReleaseType::HDTV);
        test_grid.insert("PDTV"          , ReleaseType::HDTV);
        test_grid.insert("TVRip"         , ReleaseType::HDTV);
        test_grid.insert("HDTVRip"       , ReleaseType::HDTV);
        test_grid.insert("HDRip"         , ReleaseType::HDTV);

        test_grid.insert("WEBDL"         , ReleaseType::WEBDL);
        test_grid.insert("WEB DL"        , ReleaseType::WEBDL);
        test_grid.insert("WEB-DL"        , ReleaseType::WEBDL);
        test_grid.insert("WEB-DLRip"     , ReleaseType::WEBDL);
        test_grid.insert("WEBRip (P2P)"  , ReleaseType::WEBDL);
        test_grid.insert("WEB Rip (P2P)" , ReleaseType::WEBDL);
        test_grid.insert("WEB-Rip (P2P)" , ReleaseType::WEBDL);
        test_grid.insert("WEB-Cap"       , ReleaseType::WEBDL);
        test_grid.insert("WEBCAP"        , ReleaseType::WEBDL);
        test_grid.insert("WEB Cap"       , ReleaseType::WEBDL);

        test_grid.insert("BluRayRip"     , ReleaseType::BluRayRip);
        test_grid.insert("Blu-Ray"       , ReleaseType::BluRayRip);
        test_grid.insert("BluRay"        , ReleaseType::BluRayRip);
        test_grid.insert("BLURAY"        , ReleaseType::BluRayRip);
        test_grid.insert("BDRip"         , ReleaseType::BluRayRip);
        test_grid.insert("BRRip"         , ReleaseType::BluRayRip);
        test_grid.insert("BDMV"          , ReleaseType::BluRayRip);
        test_grid.insert("BDR"           , ReleaseType::BluRayRip);
        test_grid.insert("BD25"          , ReleaseType::BluRayRip);
        test_grid.insert("BD50"          , ReleaseType::BluRayRip);
        test_grid.insert("BD5"           , ReleaseType::BluRayRip);
        test_grid.insert("BD9"           , ReleaseType::BluRayRip);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let release_type = super::parse(key.to_string()).0.unwrap();

            assert!(val == release_type);
        }
    }
}

