# Vidocq [![Build Status](https://travis-ci.org/macarrie/vidocq.svg?branch=master)](https://travis-ci.org/macarrie/vidocq/) [![GuardRails badge](https://badges.production.guardrails.io/macarrie/vidocq.svg)](https://www.guardrails.io)

vidocq is a tool designed to extract information from a torrent/media file name written in rust. It is designed to be a lighter alternative to guessit (much more complete).

## Getting Started

### Dependencies

* Rust 1.28.0

### Installing

#### Download binary

Download binary in the releases page (https://github.com/macarrie/vidocq/releases) and copy it into $PATH

#### Build from source
Clone the repository, build the binary with cargo and copy the file somewhere in your PATH.

```
git clone https://github.com/macarrie/vidocq
cargo build --release
cp target/release/vidocq /usr/local/bin/
```

### How to use

```
vidocq 0.1.3
macarrie
Command line tool to retrieve informations from a media/torrent name. Outputs information found as json

USAGE:
    vidocq [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --type <TYPE>    Type of the media to detect. Possible values are 'movie, episode'. If this option is not
                         passed, the media type will be detected automatically

ARGS:
    <NAME>    Name to parse
```

#### Example:
```
$ vidocq "South Park S18E05 HDTV x264-KILLERS [eztv]"
```
Result:
```json
{
  "audio_channels": null,
  "audio_codec": null,
  "container": null,
  "episode": 5,
  "media_type": "episode",
  "quality": null,
  "release_group": "KILLERS [eztv]",
  "release_type": "hdtv",
  "season": 18,
  "title": "South Park",
  "raw": "South Park S18E05 HDTV x264-KILLERS [eztv]",
  "video_codec": "h264",
  "year": 0
}
```

## Running the tests

A lot of vidocq tests are shamelessly stolen from the excellent following projects:

* [Guessit](https://github.com/guessit-io/guessit)
* [parse-torrent-name](https://github.com/jzjzjzj/parse-torrent-name)

To run the tests:
```
cargo test
```

## Contributing
Vidocq is Open Source. Do not hesitate to open issues for bugs or improvements
