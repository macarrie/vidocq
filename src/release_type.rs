extern crate serde;

use regex::RegexSet;

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

pub fn parse(name :&str) -> Option<ReleaseType> {
    lazy_static! {
        static ref RE_CAM :RegexSet = RegexSet::new(&[
            r"(?i)(HD)?CAM(RIP)?",
        ]).unwrap();
        static ref RE_TELESYNC :RegexSet = RegexSet::new(&[
            r"(HD)?TS",
            r"(?i)TELESYNC",
            r"(?i)PDVD",
            r"(?i)PreDVDRip",
        ]).unwrap();
        static ref RE_TELECINE :RegexSet = RegexSet::new(&[
            r"(?i)(HD)?TC",
            r"(?i)TELECINE",
        ]).unwrap();
        static ref RE_SCREENER :RegexSet = RegexSet::new(&[
            r"(?i)(DVD|BD)?SCR(EENER)?",
            r"(?i)DDC",
        ]).unwrap();
        static ref RE_DVDRIP :RegexSet = RegexSet::new(&[
           r"(?i)DVDR(IP)?",
           r"(?i)DVDMux",
           r"(?i)DVD-?(Full|\d{1,2})",
        ]).unwrap();
        static ref RE_HDTV :RegexSet = RegexSet::new(&[
            r"(HD|PD)TV",
            r"(?i)(HD|DS|SAT|DTH|DVB|TV|HDTV)Rip",
            r"DSR",
        ]).unwrap();
        static ref RE_WEBDL :RegexSet = RegexSet::new(&[
            r"(?i)WEB[-\s]?DL",
            r"(?i)WEB[-\s]?Rip",
            r"(?i)WEB[-\s]?Cap",
        ]).unwrap();
        static ref RE_BLURAY :RegexSet = RegexSet::new(&[
            r"(?i)Blu[-\s]?Ray",
            r"(?i)B[RD](Rip|MV|R|25|50|5|9)",
        ]).unwrap();
    }

    if RE_BLURAY.is_match(name) {
        return Some(ReleaseType::BluRayRip);
    }
    if RE_DVDRIP.is_match(name) {
        return Some(ReleaseType::DVDRip);
    }
    if RE_WEBDL.is_match(name) {
        return Some(ReleaseType::WEBDL);
    }
    if RE_HDTV.is_match(name) {
        return Some(ReleaseType::HDTV);
    }
    if RE_SCREENER.is_match(name) {
        return Some(ReleaseType::Screener);
    }
    if RE_TELECINE.is_match(name) {
        return Some(ReleaseType::Telecine);
    }
    if RE_TELESYNC.is_match(name) {
        return Some(ReleaseType::Telesync);
    }
    if RE_CAM.is_match(name) {
        return Some(ReleaseType::Cam);
    }

    None
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
            let release_type = super::parse(key).unwrap();

            assert!(val == release_type);
        }
    }
}

