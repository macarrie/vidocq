# Vidocq

vidocq is a tool designed to extract information from a torrent/media file name written in rust. It is designed to be a lighter alternative to guessit (much more complete).

## Getting Started

### Dependencies

* rust

### Installing

Clone the repository, build the binary with cargo and copy the file somewhere in your PATH.

```
git clone https://github.com/macarrie/vidocq
cargo build --release
cp target/release/vidocq /usr/local/bin/
```

### How to use

Simply pass it the filename to retrieve results
```
$ vidocq "South Park S18E05 HDTV x264-KILLERS [eztv]"
{"title":"South Park","season":18,"episode":5,"year":0,"quality":null,"release_type":"hdtv","video_codec":"h264","audio_codec":null,"audio_channels":null,"release_group":"KILLERS [eztv]"}
```

## Running the tests

A lot of vidocq tests are shamelessly stolen from the excellent following projects:

* [Guessit](https://github.com/guessit-io/guessit)
* [parse-torrent-name](https://github.com/jzjzjzj/parse-torrent-name)

To run the tests:
```
cargo test
```
