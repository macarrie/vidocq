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
        .get_matches();

    let name = matches.value_of("NAME").unwrap();
    let info = vidocq::parse(name);
    let j = serde_json::to_string(&info).unwrap();

    println!("{}", j);
}
