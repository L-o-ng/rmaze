use std::{env, process};
use rmaze::conf::Config;
use rmaze::maze;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {err}");
        process::exit(1);
    });

    maze::run(cfg);
}
