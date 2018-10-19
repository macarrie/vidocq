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
vidocq 0.1.0
macarrie
Command line tool to retrieve informations from a media/torrent name. Outputs information found as json

USAGE:
    vidocq <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

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
    "title": "South Park",
    "season": 18,
    "episode": 5,
    "year": 0,
    "quality": null,
    "release_type": "hdtv",
    "video_codec": "h264",
    "audio_codec": null,
    "audio_channels": null,
    "release_group": "KILLERS [eztv]"
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
Vidocq is Open Source. Do not hesitate to opne issues for bugs or improvements
