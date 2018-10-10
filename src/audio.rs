extern crate serde;

use super::utils;

use regex::Regex;


#[derive(Debug, PartialEq)]
pub enum AudioCodec {
    MP3,
    DolbyDigital,
    DolbyDigitalPlus,
    DolbyAtmos,
    AAC,
    FLAC,
    DTS,
    DolbyTrueHD,
    DTSHD,
    Opus,
    Vorbis,
    PCM,
    LPCM,
}

#[derive(Debug, PartialEq)]
pub enum AudioChannels {
    Mono,
    Stereo,
    Chan51,
    Chan71,
}

impl serde::Serialize for AudioCodec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                AudioCodec::MP3              => serializer.serialize_unit_variant("AudioCodec", 0, "mp3"),
                AudioCodec::DolbyDigital     => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_digital"),
                AudioCodec::DolbyDigitalPlus => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_digital_plus"),
                AudioCodec::DolbyAtmos       => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_atmos"),
                AudioCodec::AAC              => serializer.serialize_unit_variant("AudioCodec", 0, "aac"),
                AudioCodec::FLAC             => serializer.serialize_unit_variant("AudioCodec", 0, "flac"),
                AudioCodec::DTS              => serializer.serialize_unit_variant("AudioCodec", 0, "dts"),
                AudioCodec::DolbyTrueHD      => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_true_hd"),
                AudioCodec::DTSHD            => serializer.serialize_unit_variant("AudioCodec", 0, "dtshd"),
                AudioCodec::Opus             => serializer.serialize_unit_variant("AudioCodec", 0, "opus"),
                AudioCodec::Vorbis           => serializer.serialize_unit_variant("AudioCodec", 0, "vorbis"),
                AudioCodec::PCM              => serializer.serialize_unit_variant("AudioCodec", 0, "pcm"),
                AudioCodec::LPCM             => serializer.serialize_unit_variant("AudioCodec", 0, "lpcm"),
            }
        }
}

impl serde::Serialize for AudioChannels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match *self {
                AudioChannels::Mono   => serializer.serialize_unit_variant("AudioChannels", 0, "mono"),
                AudioChannels::Stereo => serializer.serialize_unit_variant("AudioChannels", 0, "stereo"),
                AudioChannels::Chan51 => serializer.serialize_unit_variant("AudioChannels", 0, "5.1"),
                AudioChannels::Chan71 => serializer.serialize_unit_variant("AudioChannels", 0, "7.1"),
            }
        }
}


pub fn parse_codec(name :String) -> (Option<AudioCodec>, String) {
    lazy_static! {
        static ref RE_MP3 :Vec<Regex> = vec![
            Regex::new(r"(?i)MP[EG]?[-]?3").unwrap(),
            Regex::new(r"(?i)lame3?").unwrap()
        ];

        static ref RE_DOLBY_DIGITAL :Vec<Regex> = vec![
            Regex::new(r"(?i)DD").unwrap(),
            Regex::new(r"(?i)DD5\.?1").unwrap(),
            Regex::new(r"(?i)Dolby[ -_]?Digital").unwrap(),
            Regex::new(r"(?i)AC3(-hq)?").unwrap(),
            Regex::new(r"(?i)DD[-]?EX").unwrap(),
            Regex::new(r"(?i)-EX").unwrap()
        ];

        static ref RE_DOLBY_DIGITAL_PLUS :Vec<Regex> = vec![
            Regex::new(r"(?i)DD[P+]").unwrap(),
            Regex::new(r"(?i)E[-]?AC3?").unwrap()
        ];

        static ref RE_DOLBY_ATMOS :Vec<Regex> = vec![
            Regex::new(r"(?i)(Dolby)?[ ]?Atmos(phere)?").unwrap(),
        ];

        static ref RE_AAC :Vec<Regex> = vec![
            Regex::new(r"(?i)AAC").unwrap()
        ];

        static ref RE_FLAC :Vec<Regex> = vec![
            Regex::new(r"(?i)FLAC").unwrap()
        ];

        static ref RE_DTS :Vec<Regex> = vec![
            Regex::new(r"(?i)DTS[-]?(ES)?").unwrap(),
            Regex::new(r"(?i)-ES").unwrap()
        ];

        static ref RE_DOLBY_TRUE_HD :Vec<Regex> = vec![
            Regex::new(  r"(?i)True[- .]?HD").unwrap()
        ];

        static ref RE_DTSHD :Vec<Regex> = vec![
            Regex::new(r"(?i)DTS[- .]?(HD)[- .]?(MA)?").unwrap(),
            Regex::new(r"(?i)HRA?").unwrap(),
            Regex::new(r"(?i)DTSMA").unwrap()
        ];

        static ref RE_OPUS :Vec<Regex> = vec![
            Regex::new(r"(?i)OPUS").unwrap()
        ];

        static ref RE_VORBIS :Vec<Regex> = vec![
            Regex::new(r"(?i)VORBIS").unwrap()
        ];

        static ref RE_LPCM :Vec<Regex> = vec![
            Regex::new(r"(?i)LPCM").unwrap()
        ];

        static ref RE_PCM :Vec<Regex> = vec![
            Regex::new(r"(?i)PCM").unwrap()
        ];
    }

    let mut matched_codec :Option<AudioCodec> = None;
    let original_name = name.clone();

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_MP3.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::MP3);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DOLBY_DIGITAL_PLUS.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DolbyDigitalPlus);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DOLBY_DIGITAL.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DolbyDigital);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DOLBY_ATMOS.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DolbyAtmos);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_AAC.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::AAC);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_FLAC.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::FLAC);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DTSHD.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DTSHD);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DTS.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DTS);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_DOLBY_TRUE_HD.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::DolbyTrueHD);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_OPUS.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::Opus);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_VORBIS.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::Vorbis);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_LPCM.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::LPCM);

        return (matched_codec, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_PCM.to_vec());
    if matched {
        matched_codec = Some(AudioCodec::PCM);

        return (matched_codec, stripped_name);
    }

    (matched_codec, original_name)
}

pub fn parse_channels(name :String) -> (Option<AudioChannels>, String) {
    lazy_static! {
        static ref RE_MONO :Vec<Regex> = vec![
            Regex::new(r"(?i)1ch").unwrap(),
            Regex::new(r"(?i)mono").unwrap(),
        ];

        static ref RE_STEREO :Vec<Regex> = vec![
            Regex::new(r"(?i)2ch").unwrap(),
            Regex::new(r"(?i)2\.0").unwrap(),
            Regex::new(r"(?i)AAC2\.?0").unwrap(),
            Regex::new(r"(?i)stereo").unwrap(),
        ];

        static ref RE_CHAN51 :Vec<Regex> = vec![
            Regex::new(r"(?i)[56]ch").unwrap(),
            Regex::new(r"(?i)5\.?1").unwrap(),
            Regex::new(r"(?i)DD5\.?1").unwrap(),
            Regex::new(r"(?i)True[-]?HD5\.?1").unwrap(),
        ];

        static ref RE_CHAN71 :Vec<Regex> = vec![
            Regex::new(r"(?i)[78]ch").unwrap(),
            Regex::new(r"(?i)7\.1").unwrap(),
        ];
    }

    let mut matched_channels :Option<AudioChannels> = None;
    let original_name = name.clone();

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_MONO.to_vec());
    if matched {
        matched_channels = Some(AudioChannels::Mono);

        return (matched_channels, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_STEREO.to_vec());
    if matched {
        matched_channels = Some(AudioChannels::Stereo);

        return (matched_channels, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_CHAN51.to_vec());
    if matched {
        matched_channels = Some(AudioChannels::Chan51);

        return (matched_channels, stripped_name);
    }

    let (matched, stripped_name) = utils::find_and_strip(&name, RE_CHAN71.to_vec());
    if matched {
        matched_channels = Some(AudioChannels::Chan71);

        return (matched_channels, stripped_name);
    }

    (matched_channels, original_name)
}

pub fn parse(name :String) -> (Option<AudioCodec>, Option<AudioChannels>, String) {
    let (codec, stripped) = parse_codec(name);
    let (channel, stripped) = parse_channels(stripped);

    (codec, channel, stripped)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_video_codec() {
        let mut test_grid :HashMap<&str, AudioCodec> = HashMap::new();

        test_grid.insert("MP3", AudioCodec::MP3);
        test_grid.insert("lame", AudioCodec::MP3);
        test_grid.insert("lame3.12", AudioCodec::MP3);
        test_grid.insert("lame3.100", AudioCodec::MP3);

        test_grid.insert("DolbyDigital", AudioCodec::DolbyDigital);
        test_grid.insert("DD", AudioCodec::DolbyDigital);
        test_grid.insert("Dolby Digital", AudioCodec::DolbyDigital);
        test_grid.insert("AC3", AudioCodec::DolbyDigital);
        test_grid.insert("AC3-hq", AudioCodec::DolbyDigital);
        test_grid.insert("DD5.1", AudioCodec::DolbyDigital);
        test_grid.insert("DD51", AudioCodec::DolbyDigital);
        test_grid.insert("DD-EX", AudioCodec::DolbyDigital);
        test_grid.insert("DDEX", AudioCodec::DolbyDigital);
        test_grid.insert("-EX", AudioCodec::DolbyDigital);

        test_grid.insert("DDP", AudioCodec::DolbyDigitalPlus);
        test_grid.insert("DD+", AudioCodec::DolbyDigitalPlus);
        test_grid.insert("EAC3", AudioCodec::DolbyDigitalPlus);

        test_grid.insert("DolbyAtmos", AudioCodec::DolbyAtmos);
        test_grid.insert("Dolby Atmos", AudioCodec::DolbyAtmos);
        test_grid.insert("Atmos", AudioCodec::DolbyAtmos);
        test_grid.insert("Atmosphere", AudioCodec::DolbyAtmos);

        test_grid.insert("AAC", AudioCodec::AAC);
        test_grid.insert("AAC-HE", AudioCodec::AAC);
        test_grid.insert("AAC-LC", AudioCodec::AAC);
        test_grid.insert("AAC2.0", AudioCodec::AAC);
        test_grid.insert("AAC20", AudioCodec::AAC);

        test_grid.insert("Flac", AudioCodec::FLAC);

        test_grid.insert("DTS", AudioCodec::DTS);
        test_grid.insert("DTSES", AudioCodec::DTS);
        test_grid.insert("DTS-ES", AudioCodec::DTS);
        test_grid.insert("-ES", AudioCodec::DTS);

        test_grid.insert("True-HD", AudioCodec::DolbyTrueHD);
        test_grid.insert("trueHD", AudioCodec::DolbyTrueHD);
        test_grid.insert("True-HD51", AudioCodec::DolbyTrueHD);
        test_grid.insert("trueHD51", AudioCodec::DolbyTrueHD);

        test_grid.insert("DTSHD", AudioCodec::DTSHD);
        test_grid.insert("DTS HD", AudioCodec::DTSHD);
        test_grid.insert("DTS-HD", AudioCodec::DTSHD);
        test_grid.insert("DTS-HDma", AudioCodec::DTSHD);
        test_grid.insert("DTSMA", AudioCodec::DTSHD);
        test_grid.insert("DTS-HD.HRA", AudioCodec::DTSHD);
        test_grid.insert("DTSHD.HRA", AudioCodec::DTSHD);
        test_grid.insert("DTS-HD.HR", AudioCodec::DTSHD);
        test_grid.insert("DTSHD.HR", AudioCodec::DTSHD);
        test_grid.insert("HRA", AudioCodec::DTSHD);
        test_grid.insert("HR", AudioCodec::DTSHD);

        test_grid.insert("OPUS", AudioCodec::Opus);

        test_grid.insert("Vorbis", AudioCodec::Vorbis);

        test_grid.insert("PCM", AudioCodec::PCM);

        test_grid.insert("LPCM", AudioCodec::LPCM);


        let mut channels_test_grid :HashMap<&str, AudioChannels> = HashMap::new();

        channels_test_grid.insert("1ch", AudioChannels::Mono);
        channels_test_grid.insert("mono", AudioChannels::Mono);

        channels_test_grid.insert("AAC2.0", AudioChannels::Stereo);
        channels_test_grid.insert("AAC20", AudioChannels::Stereo);
        channels_test_grid.insert("2ch", AudioChannels::Stereo);
        channels_test_grid.insert("2.0", AudioChannels::Stereo);
        channels_test_grid.insert("stereo", AudioChannels::Stereo);

        channels_test_grid.insert("DD5.1", AudioChannels::Chan51);
        channels_test_grid.insert("DD51", AudioChannels::Chan51);
        channels_test_grid.insert("True-HD51", AudioChannels::Chan51);
        channels_test_grid.insert("trueHD51", AudioChannels::Chan51);
        channels_test_grid.insert("5.1", AudioChannels::Chan51);
        channels_test_grid.insert("5ch", AudioChannels::Chan51);
        channels_test_grid.insert("6ch", AudioChannels::Chan51);

        channels_test_grid.insert("7.1", AudioChannels::Chan71);
        channels_test_grid.insert("7ch", AudioChannels::Chan71);
        channels_test_grid.insert("8ch", AudioChannels::Chan71);

        for (key, val) in test_grid {
            println!("Test item: {}", key);
            let audio_codec = super::parse_codec(key.to_string()).0.unwrap();

            assert!(val == audio_codec);
        }

        for (key, val) in channels_test_grid {
            println!("Test item: {}", key);
            let audio_channels = super::parse_channels(key.to_string()).0.unwrap();

            assert!(val == audio_channels);
        }
    }
}

