# Project Title

vidocq is a tool designed to extract information from a torrent/media file name. It is designed to be a lighter alternative to guessit (much more complete).

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

* rust

### Installing

Clone the repository, build the binary with cargo and copy the file somewhere in your PATH.

```
git clone https://github.com/macarrie/vidocq
cargo build --release
cp target/release/vidocq /usr/local/bin/
```

Simply pass it the filename to retrieve results
```
vidocq "South Park S18E05 HDTV x264-KILLERS [eztv]"
SAMPLE OUTPUT
```

## Running the tests

```
cargo test
```
