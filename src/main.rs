mod slp_chart_object;

use std::{fs, io};

fn main() {
    let mut buf = io::BufReader::new(fs::File::open("game.slp").unwrap());
    let game = peppi::game(&mut buf, None, None).unwrap();
    println!("{:#?}", game);
    game.
}

// fn parse_slp(filename: String) -> Result<String, String> {}
