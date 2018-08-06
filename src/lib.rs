extern crate regex;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod quality;
mod year;
mod episode;
mod release_type;
mod video_codec;
mod title;

#[derive(Serialize, Debug, PartialEq)]
pub struct MediaInfo {
    title :String,
    season :i32,
    episode :i32,
    year :i32,
    quality :Option<quality::Quality>,
    release_type :Option<release_type::ReleaseType>,
    video_codec :Option<video_codec::VideoCodec>,
}

pub fn parse(name :&str) -> MediaInfo {
    let (season_nb, episode_nb) = episode::parse(name);
    let quality = quality::parse(name);
    let year = year::parse(name);
    let release_type = release_type::parse(name);
    let video_codec = video_codec::parse(name);
    let title = title::parse(name);

    MediaInfo {
        title: title,
        season: season_nb,
        episode: episode_nb,
        year: year,
        quality: quality,
        release_type: release_type,
        video_codec: video_codec
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
            title: "2047 - Sights of Death".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::BluRayRip),
            video_codec: Some(video_codec::VideoCodec::H264)
        });
        test_grid.insert("The Flash 2014 S01E04 HDTV x264-FUM[ettv]", MediaInfo{
            title: "The Flash".to_string(),
            season: 1,
            episode: 4,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264)
        });
        test_grid.insert( "The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]", MediaInfo{
            title: "The Walking Dead".to_string(),
            season: 5,
            episode: 3,
            year: 0,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
            //group: "ASAP[ettv]",
        });
        test_grid.insert( "Hercules (2014) 1080p BrRip H264 - YIFY", MediaInfo{
            title: "Hercules".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::BluRayRip),
            video_codec: Some(video_codec::VideoCodec::H264),
            //group: "YIFY",
        });
        test_grid.insert( "Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO", MediaInfo{
            title: "Dawn of the Planet of the Apes".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
            //group: "EVO",
        });
        test_grid.insert( "The Big Bang Theory S08E06 HDTV XviD-LOL [eztv]", MediaInfo{
            title: "The Big Bang Theory".to_string(),
            season: 8,
            episode: 6,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
            //group: "LOL [eztv]",
        });
        test_grid.insert( "22 Jump Street (2014) 720p BrRip x264 - YIFY", MediaInfo{
            title: "22 Jump Street".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::BluRayRip),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG", MediaInfo{
            title: "Hercules".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Hercules.2014.EXTENDED.HDRip.XViD-juggs[ETRG]", MediaInfo{
            title: "Hercules".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Hercules (2014) WEBDL DVDRip XviD-MAX", MediaInfo{
            title: "Hercules".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::DVDRip),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "WWE Hell in a Cell 2014 PPV WEB-DL x264-WD -={SPARROW}=-", MediaInfo{
            title: "WWE Hell in a Cell".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
            //group: "WD -={SPARROW}=-",
        });
        test_grid.insert( "Marvels Agents of S H I E L D S02E05 HDTV x264-KILLERS [eztv]", MediaInfo{
            title: "Marvels Agents of S H I E L D".to_string(),
            season: 2,
            episode: 5,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "X-Men.Days.of.Future.Past.2014.1080p.WEB-DL.DD5.1.H264-RARBG", MediaInfo{
            title: "X-Men Days of Future Past".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Guardians Of The Galaxy 2014 R6 720p HDCAM x264-JYK", MediaInfo{
            title: "Guardians Of The Galaxy".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::Cam),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Marvel's.Agents.of.S.H.I.E.L.D.S02E01.Shadows.1080p.WEB-DL.DD5.1", MediaInfo{
            title: "Marvel's Agents of S H I E L D".to_string(),
            season: 2,
            episode: 1,
            year: 0,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: None,
        });
        test_grid.insert( "Marvels Agents of S.H.I.E.L.D. S02E06 HDTV x264-KILLERS[ettv]", MediaInfo{
            title: "Marvels Agents of S H I E L D".to_string(),
            season: 2,
            episode: 6,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "The.Walking.Dead.S05E03.1080p.WEB-DL.DD5.1.H.264-Cyphanix[rartv]", MediaInfo{
            title: "The Walking Dead".to_string(),
            season: 5,
            episode: 3,
            year: 0,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Brave.2012.R5.DVDRip.XViD.LiNE-UNiQUE", MediaInfo{
            title: "Brave".to_string(),
            season: 0,
            episode: 0,
            year: 2012,
            quality: None,
            release_type: Some(release_type::ReleaseType::DVDRip),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Lets.Be.Cops.2014.BRRip.XViD-juggs[ETRG]", MediaInfo{
            title: "Lets Be Cops".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::BluRayRip),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Downton Abbey 5x06 HDTV x264-FoV [eztv]", MediaInfo{
            title: "Downton Abbey".to_string(),
            season: 5,
            episode: 6,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]", MediaInfo{
            title: "Annabelle".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Lucy.2014.HC.HDRip.XViD-juggs[ETRG]", MediaInfo{
            title: "Lucy".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "The Flash 2014 S01E04 HDTV x264-FUM[ettv]", MediaInfo{
            title: "The Flash".to_string(),
            season: 1,
            episode: 4,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "South Park S18E05 HDTV x264-KILLERS [eztv]", MediaInfo{
            title: "South Park".to_string(),
            season: 18,
            episode: 5,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "The Flash 2014 S01E03 HDTV x264-LOL[ettv]", MediaInfo{
            title: "The Flash".to_string(),
            season: 1,
            episode: 3,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "The Simpsons S26E05 HDTV x264 PROPER-LOL [eztv]", MediaInfo{
            title: "The Simpsons".to_string(),
            season: 26,
            episode: 5,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "2047 - Sights of Death (2014) 720p BrRip x264 - YIFY", MediaInfo{
            title: "2047 - Sights of Death".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q720),
            release_type: Some(release_type::ReleaseType::BluRayRip),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Two and a Half Men S12E01 HDTV x264 REPACK-LOL [eztv]", MediaInfo{
            title: "Two and a Half Men".to_string(),
            season: 12,
            episode: 1,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Dinosaur 13 2014 WEBrip XviD AC3 MiLLENiUM", MediaInfo{
            title: "Dinosaur 13".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Teenage.Mutant.Ninja.Turtles.2014.HDRip.XviD.MP3-RARBG", MediaInfo{
            title: "Teenage Mutant Ninja Turtles".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: None,
            release_type: Some(release_type::ReleaseType::HDTV),
            video_codec: Some(video_codec::VideoCodec::XVID),
        });
        test_grid.insert( "Dawn.Of.The.Planet.of.The.Apes.2014.1080p.WEB-DL.DD51.H264-RARBG", MediaInfo{
            title: "Dawn Of The Planet of The Apes".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Gotham.S01E05.Viper.WEB-DL.x264.AAC", MediaInfo{
            title: "Gotham".to_string(),
            season: 1,
            episode: 5,
            year: 0,
            quality: None,
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });
        test_grid.insert( "Into.The.Storm.2014.1080p.WEB-DL.AAC2.0.H264-RARBG", MediaInfo{
            title: "Into The Storm".to_string(),
            season: 0,
            episode: 0,
            year: 2014,
            quality: Some(quality::Quality::Q1080),
            release_type: Some(release_type::ReleaseType::WEBDL),
            video_codec: Some(video_codec::VideoCodec::H264),
        });

        for (key, val) in test_grid.iter() {
            println!("Test item: {}", key);
            let info = parse(key);

            assert_eq!(val, &info);
        }
    }
}
