/*
 * FileName:     docopt
 * Author: 8ucchiman
 * CreatedDate:  2023-02-07 18:25:16 +0900
 * LastModified: 2023-02-07 18:26:19 +0900
 * Reference: https://github.com/docopt/docopt.rs
 */


use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
Naval Fate.

Usage:
  naval_fate.py ship new <name>...
  naval_fate.py ship <name> move <x> <y> [--speed=<kn>]
  naval_fate.py ship shoot <x> <y>
  naval_fate.py mine (set|remove) <x> <y> [--moored | --drifting]
  naval_fate.py (-h | --help)
  naval_fate.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --speed=<kn>  Speed in knots [default: 10].
  --moored      Moored (anchored) mine.
  --drifting    Drifting mine.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_speed: isize,
    flag_drifting: bool,
    arg_name: Vec<String>,
    arg_x: Option<i32>,
    arg_y: Option<i32>,
    cmd_ship: bool,
    cmd_mine: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
