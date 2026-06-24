use std::env;
use std::io::{self, BufRead};
use std::process;

use matcher::models::Match;

mod matcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no query specified");
        process::exit(1);
    }
    let query = &args[1];
    println!("query: {}\n", query);
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let text = line.unwrap();
        let query_parts = query.split(" "); // move outside
        let mut idxs: Vec<Match> = matcher::fuzzy_find(&text, query_parts.collect());
        idxs.sort();
        if idxs.is_empty() {
            continue;
        }
        let colored_text = matcher::highlight_matches(&text, &idxs);

        // println!("{}    -> {:?}", colored_text, idxs);
        println!("{}", colored_text);
    }
}
