extern crate clap;
extern crate serde_json;
extern crate vidocq;

use clap::{App, Arg};

fn main() {
    let matches = App::new("vidocq")
        .version("0.1.1")
        .author("macarrie")
        .about("Command line tool to retrieve informations from a media/torrent name. Outputs information found as json")
        .arg(Arg::with_name("NAME")
             .required(true)
             .takes_value(true)
             .index(1)
             .help("Name to parse"))
        .arg(Arg::with_name("TYPE")
            .required(false)
            .takes_value(true)
            .short("t")
            .long("type")
            .help("Type of the media to detect. Possible values are 'movie, episode'. If this option is not passed, the media type will be detected automatically"))
        .get_matches();

    let name = matches.value_of("NAME").unwrap();
    let media_type = matches.value_of("TYPE").unwrap();
    println!("Media type: '{}'", media_type);
    let info = vidocq::parse(name);
    let j = serde_json::to_string(&info).unwrap();

    println!("{}", j);
}
