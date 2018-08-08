extern crate serde;

use regex::RegexSet;

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
                AudioCodec::MP3              => serializer.serialize_unit_variant("AudioCodec", 0, "MP3"),
                AudioCodec::DolbyDigital     => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_digital"),
                AudioCodec::DolbyDigitalPlus => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_digital_plus"),
                AudioCodec::DolbyAtmos       => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_atmos"),
                AudioCodec::AAC              => serializer.serialize_unit_variant("AudioCodec", 0, "AAC"),
                AudioCodec::FLAC             => serializer.serialize_unit_variant("AudioCodec", 0, "FLAC"),
                AudioCodec::DTS              => serializer.serialize_unit_variant("AudioCodec", 0, "DTS"),
                AudioCodec::DolbyTrueHD      => serializer.serialize_unit_variant("AudioCodec", 0, "dolby_true_hd"),
                AudioCodec::DTSHD            => serializer.serialize_unit_variant("AudioCodec", 0, "DTSHD"),
                AudioCodec::Opus             => serializer.serialize_unit_variant("AudioCodec", 0, "opus"),
                AudioCodec::Vorbis           => serializer.serialize_unit_variant("AudioCodec", 0, "vorbis"),
                AudioCodec::PCM              => serializer.serialize_unit_variant("AudioCodec", 0, "PCM"),
                AudioCodec::LPCM             => serializer.serialize_unit_variant("AudioCodec", 0, "LPCM"),
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

pub fn parse_codec(name :&str) -> Option<AudioCodec> {
    lazy_static! {
        static ref RE_MP3 :RegexSet = RegexSet::new(&[
            r"(?i)MP[EG]?[-]?3",
            r"(?i)lame3?",
        ]).unwrap();
        static ref RE_DOLBY_DIGITAL :RegexSet = RegexSet::new(&[
            r"(?i)DD",
            r"(?i)DD5\.?1",
            r"(?i)Dolby[ -_]?Digital",
            r"(?i)AC3(-hq)?",
            r"(?i)DD[-]?EX",
            r"(?i)-EX",
        ]).unwrap();
        static ref RE_DOLBY_DIGITAL_PLUS :RegexSet = RegexSet::new(&[
            r"(?i)DD[P+]",
            r"(?i)E[-]?AC3?",
        ]).unwrap();
        static ref RE_DOLBY_ATMOS :RegexSet = RegexSet::new(&[
            r"(?i)(Dolby)?[ ]?Atmos(phere)?",
        ]).unwrap();
        static ref RE_AAC :RegexSet = RegexSet::new(&[
            r"(?i)AAC",
        ]).unwrap();
        static ref RE_FLAC :RegexSet = RegexSet::new(&[
            r"(?i)FLAC",
        ]).unwrap();
        static ref RE_DTS :RegexSet = RegexSet::new(&[
            r"(?i)DTS[-]?(ES)?",
            r"(?i)-ES",
        ]).unwrap();
        static ref RE_DOLBY_TRUE_HD :RegexSet = RegexSet::new(&[
            r"(?i)True[- .]?HD",
        ]).unwrap();
        static ref RE_DTSHD :RegexSet = RegexSet::new(&[
            r"(?i)DTS[- .]?(HD)[- .]?(MA)?",
            r"(?i)HRA?",
            r"(?i)DTSMA",
        ]).unwrap();
        static ref RE_OPUS :RegexSet = RegexSet::new(&[
            r"(?i)OPUS",
        ]).unwrap();
        static ref RE_VORBIS :RegexSet = RegexSet::new(&[
            r"(?i)VORBIS",
        ]).unwrap();
        static ref RE_LPCM :RegexSet = RegexSet::new(&[
            r"(?i)LPCM",
        ]).unwrap();
        static ref RE_PCM :RegexSet = RegexSet::new(&[
            r"(?i)PCM",
        ]).unwrap();
    }

    if RE_MP3.is_match(name) {
        return Some(AudioCodec::MP3);
    }
    if RE_DOLBY_DIGITAL_PLUS.is_match(name) {
        return Some(AudioCodec::DolbyDigitalPlus);
    }
    if RE_DOLBY_DIGITAL.is_match(name) {
        return Some(AudioCodec::DolbyDigital);
    }
    if RE_DOLBY_ATMOS.is_match(name) {
        return Some(AudioCodec::DolbyAtmos);
    }
    if RE_AAC.is_match(name) {
        return Some(AudioCodec::AAC);
    }
    if RE_FLAC.is_match(name) {
        return Some(AudioCodec::FLAC);
    }
    if RE_DTSHD.is_match(name) {
        return Some(AudioCodec::DTSHD);
    }
    if RE_DTS.is_match(name) {
        return Some(AudioCodec::DTS);
    }
    if RE_DOLBY_TRUE_HD.is_match(name) {
        return Some(AudioCodec::DolbyTrueHD);
    }
    if RE_OPUS.is_match(name) {
        return Some(AudioCodec::Opus);
    }
    if RE_VORBIS.is_match(name) {
        return Some(AudioCodec::Vorbis);
    }
    if RE_LPCM.is_match(name) {
        return Some(AudioCodec::LPCM);
    }
    if RE_PCM.is_match(name) {
        return Some(AudioCodec::PCM);
    }

    None
}

pub fn parse_channels(name :&str) -> Option<AudioChannels> {
    lazy_static! {
        static ref RE_MONO :RegexSet = RegexSet::new(&[
            r"(?i)1ch",
            r"(?i)mono",
        ]).unwrap();
        static ref RE_STEREO :RegexSet = RegexSet::new(&[
            r"(?i)2ch",
            r"(?i)2\.0",
            r"(?i)AAC2\.?0",
            r"(?i)stereo",
        ]).unwrap();
        static ref RE_CHAN51 :RegexSet = RegexSet::new(&[
            r"(?i)[56]ch",
            r"(?i)5\.1",
            r"(?i)DD5\.?1",
            r"(?i)True[-]?HD5\.?1",
        ]).unwrap();
        static ref RE_CHAN71 :RegexSet = RegexSet::new(&[
            r"(?i)[78]ch",
            r"(?i)7\.1",
        ]).unwrap();
    }

    if RE_MONO.is_match(name) {
        return Some(AudioChannels::Mono);
    }
    if RE_STEREO.is_match(name) {
        return Some(AudioChannels::Stereo);
    }
    if RE_CHAN51.is_match(name) {
        return Some(AudioChannels::Chan51);
    }
    if RE_CHAN71.is_match(name) {
        return Some(AudioChannels::Chan71);
    }

    None
}

pub fn parse(name :&str) -> (Option<AudioCodec>, Option<AudioChannels>) {
    (parse_codec(name), parse_channels(name))
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
            let audio_codec = super::parse_codec(key).unwrap();

            assert!(val == audio_codec);
        }

        for (key, val) in channels_test_grid {
            println!("Test item: {}", key);
            let audio_channels = super::parse_channels(key).unwrap();

            assert!(val == audio_channels);
        }
    }
}

