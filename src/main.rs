use colored::Colorize;
use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};
use std::process;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Match {
    idx: usize,
    len: usize,
}

impl Hash for Match {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Eq for Match {}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Match {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

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
        let mut idxs: Vec<Match> = fuzzy_find(&text, query_parts.collect());
        idxs.sort();
        if idxs.is_empty() {
            continue;
        }
        let colored_text = color_matches(&text, &idxs);

        // println!("{}    -> {:?}", colored_text, idxs);
        println!("{}", colored_text);
    }
}

fn color_matches(line: &str, matches: &[Match]) -> String {
    let mut new_str = String::new();
    let mut prev_idx = 0;

    for m in matches {
        if m.idx > prev_idx {
            let chunk = &line[prev_idx..m.idx];
            new_str.push_str(chunk);
        }

        let hit_str = &line[m.idx..m.idx + m.len].green().to_string();
        new_str.push_str(hit_str);
        prev_idx = m.idx + m.len;
    }

    new_str.push_str(&line[prev_idx..]);

    new_str
}

fn fuzzy_find(line: &str, query_parts: Vec<&str>) -> Vec<Match> {
    let mut indexes = HashSet::new();
    for query in query_parts {
        let mut line_rest = line;
        let mut idx_offset = 0;
        let needles = split_into_needles(query);
        for c in needles {
            if let Some(idx) = line_rest.find(c) {
                idx_offset += idx;
                indexes.insert(Match {
                    idx: idx_offset,
                    len: c.len(),
                });
                let next_idx = idx + c.len();
                idx_offset += c.len();
                line_rest = &line_rest[next_idx..];
            } else {
                return vec![];
            }
        }
    }
    indexes.into_iter().collect()
}

fn split_into_needles(query: &str) -> Vec<&str> {
    if let Some(query_strip) = query.strip_prefix("'") {
        vec![&query_strip]
    } else {
        query.split("").filter(|c| c.len() == 1).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_into_needles() {
        let f = split_into_needles;
        assert_eq!(f("'ll"), vec!["ll"]);
        assert_eq!(f("hll"), vec!["h", "l", "l"]);
    }

    #[test]
    fn test_fuzzy_find() {
        let cases = [
            (
                "simple search",
                "hello world, from europe!",
                vec!["h", "l", "f"],
                vec![(0, 1), (2, 1), (13, 1)],
            ),
            (
                "duplicated needle",
                "hello world, from europe!",
                vec!["h", "l", "f", "f"],
                vec![(0, 1), (2, 1), (13, 1)],
            ),
            (
                "one needle not found = no result",
                "hello world, from europe!",
                vec!["h", "l", "f", "x"],
                vec![],
            ),
            (
                "needle-group does forward search",
                "hello world, from europe!",
                vec!["hwfe"],
                vec![(0, 1), (6, 1), (13, 1), (18, 1)],
            ),
            (
                "needle-group does only forward search",
                "hello world, from europe!",
                vec!["hwfel"],
                vec![],
            ),
            (
                "needle-block (starting with ') is used as whole",
                "hello world, from europe!",
                vec!["'world"],
                vec![(6, 5)],
            ),
            (
                "needle-block (starting with ') is not splitted into needles for search",
                "hello world, from europe!",
                vec!["'word"],
                vec![],
            ),
            (
                "two needle-blocks (starting with ')",
                "hello world, from europe!",
                vec!["'ll", "'world"],
                vec![(2, 2), (6, 5)],
            ),
            (
                "needle-group with duplicated needle",
                "hello world, from europe!",
                vec!["hll"],
                vec![(0, 1), (2, 1), (3, 1)],
            ),
            (
                "two overlapping needle-blocks (starting with ') still match",
                "hello world, from europe!",
                vec!["'ll", "'lo"],
                vec![(2, 2), (3, 2)],
            ),
        ];

        for (descr, line, needles, expected) in cases {
            let mut result = fuzzy_find(line, needles);
            result.sort_unstable();
            let result2: Vec<(usize, usize)> = result.into_iter().map(|m| (m.idx, m.len)).collect();
            assert_eq!(result2, expected, "E: {}", descr);
        }
    }
}
