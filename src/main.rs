use std::{env, process};
use cmaze::conf::Config;
use cmaze::maze;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {err}");
        process::exit(1);
    });

    maze::run(cfg);
}
