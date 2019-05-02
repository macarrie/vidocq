extern crate serde;

use super::utils;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum VideoCodec {
    DIVX,
    XVID,
    H262,
    H263,
    H264,
    H265,
}

impl serde::Serialize for VideoCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            VideoCodec::DIVX => serializer.serialize_unit_variant("VideoCodec", 0, "divx"),
            VideoCodec::XVID => serializer.serialize_unit_variant("VideoCodec", 0, "xvid"),
            VideoCodec::H262 => serializer.serialize_unit_variant("VideoCodec", 0, "h262"),
            VideoCodec::H263 => serializer.serialize_unit_variant("VideoCodec", 0, "h263"),
            VideoCodec::H264 => serializer.serialize_unit_variant("VideoCodec", 0, "h264"),
            VideoCodec::H265 => serializer.serialize_unit_variant("VideoCodec", 0, "h265"),
        }
    }
}

pub fn parse(name: String) -> (Option<VideoCodec>, String) {
    lazy_static! {
        static ref RE_H262: Vec<Regex> = vec![
            Regex::new(r"(?i)MP[E]?G[-]?2").unwrap(),
            Regex::new(r"(?i)x?262").unwrap(),
        ];
        static ref RE_DIVX: Vec<Regex> = vec![Regex::new(r"(?i)DIV(\s)?X").unwrap(),];
        static ref RE_XVID: Vec<Regex> = vec![Regex::new(r"(?i)X(\s)?VID").unwrap(),];
        static ref RE_H263: Vec<Regex> = vec![Regex::new(r"(?i)x?263").unwrap(),];
        static ref RE_H264: Vec<Regex> = vec![
            Regex::new(r"(?i)MP[E]?G[-]?4").unwrap(),
            Regex::new(r"(?i)x?264").unwrap(),
            Regex::new(r"(?i)AVC(HD)?").unwrap(),
        ];
        static ref RE_H265: Vec<Regex> = vec![Regex::new(r"(?i)x?265").unwrap(),];
    }

    let mut matched_codec: Option<VideoCodec> = None;
    let original_name = name.clone();

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_H265.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::H265);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_H264.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::H264);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_H263.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::H263);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DIVX.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::DIVX);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_XVID.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::XVID);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_H262.to_vec());
    if matched {
        matched_codec = Some(VideoCodec::H262);

        return (matched_codec, stripped_name);
    }

    (matched_codec, original_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_video_codec() {
        let mut test_grid: HashMap<&str, VideoCodec> = HashMap::new();

        test_grid.insert("mpeg2", VideoCodec::H262);
        test_grid.insert("MPEG2", VideoCodec::H262);
        test_grid.insert("MPEG-2", VideoCodec::H262);
        test_grid.insert("mpg2", VideoCodec::H262);
        test_grid.insert("H262", VideoCodec::H262);
        test_grid.insert("H.262", VideoCodec::H262);
        test_grid.insert("x262", VideoCodec::H262);
        test_grid.insert("-xmpeg2", VideoCodec::H262);
        test_grid.insert("-mpeg2x", VideoCodec::H262);

        test_grid.insert("DivX", VideoCodec::DIVX);
        test_grid.insert("-div X", VideoCodec::DIVX);
        test_grid.insert("divx", VideoCodec::DIVX);
        test_grid.insert("dvdivx", VideoCodec::DIVX);
        test_grid.insert("DVDivX", VideoCodec::DIVX);

        test_grid.insert("XviD", VideoCodec::XVID);
        test_grid.insert("xvid", VideoCodec::XVID);
        test_grid.insert("-x vid", VideoCodec::XVID);

        test_grid.insert("h263", VideoCodec::H263);
        test_grid.insert("x263", VideoCodec::H263);
        test_grid.insert("h.263", VideoCodec::H263);

        test_grid.insert("h264", VideoCodec::H264);
        test_grid.insert("x264", VideoCodec::H264);
        test_grid.insert("h.264", VideoCodec::H264);
        test_grid.insert("x.264", VideoCodec::H264);
        test_grid.insert("mpeg4-AVC", VideoCodec::H264);
        test_grid.insert("AVC", VideoCodec::H264);
        test_grid.insert("AVCHD", VideoCodec::H264);
        test_grid.insert("AVCHD-SC", VideoCodec::H264);
        test_grid.insert("H.264-SC", VideoCodec::H264);
        test_grid.insert("H.264-AVCHD-SC", VideoCodec::H264);
        test_grid.insert("-MPEG-4", VideoCodec::H264);
        test_grid.insert("-mpeg4", VideoCodec::H264);
        test_grid.insert("-h 264", VideoCodec::H264);
        test_grid.insert("-x264", VideoCodec::H264);

        test_grid.insert("h265", VideoCodec::H265);
        test_grid.insert("x265", VideoCodec::H265);
        test_grid.insert("h.265", VideoCodec::H265);
        test_grid.insert("x.265", VideoCodec::H265);
        test_grid.insert("-h 265", VideoCodec::H265);
        test_grid.insert("-x265", VideoCodec::H265);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let video_codec = super::parse(key.to_string()).0.unwrap();

            assert_eq!(val, video_codec);
        }
    }
}
