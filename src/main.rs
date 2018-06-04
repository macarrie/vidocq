extern crate regex;
use regex::Regex;

fn main() {
    let torrents = [
        "test",
        "Westworld.S02E05.Akane.No.Mai.1080p.AMZN.WEBRip.DDP5.1.x264-NTb[rartv]"
    ];

    let version_regex = Regex::new(r"(?i)S(?P<season>\d{1,2})E(?P<episode>\d{1,2})").unwrap();

    for s in torrents.iter() {
        println!("{}", s);
        let cap = version_regex.captures(s);
        match cap {
            Some(c) => {
                println!("Season {} Episode {}", &c["season"], &c["episode"]);
            },
            None => {
                println!("No match");
            }
        }
    }
}
