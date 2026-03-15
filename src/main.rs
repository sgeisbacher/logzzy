use std::collections::HashSet;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no query specified");
        process::exit(1);
    }
    let query = &args[1];
    let text = "hello world, from stefan in graz!";

    let query_parts = query.split(" ");

    println!(
        "query {} - {}: {:?}",
        query,
        text,
        fuzzy_find(text, query_parts.collect())
    );
}

fn fuzzy_find(line: &str, query_parts: Vec<&str>) -> Vec<usize> {
    let mut indexes = HashSet::new();
    for query in query_parts {
        let mut line_rest = line;
        let mut idx_offset = 0;
        let needles = split_into_needles(query);
        for c in needles {
            if let Some(idx) = line_rest.find(c) {
                idx_offset += idx;
                indexes.insert(idx_offset);
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
                vec![0, 2, 13],
            ),
            (
                "duplicated needle",
                "hello world, from europe!",
                vec!["h", "l", "f", "f"],
                vec![0, 2, 13],
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
                vec![0, 6, 13, 18],
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
                vec![6],
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
                vec![2, 6],
            ),
            (
                "needle-group with duplicated needle",
                "hello world, from europe!",
                vec!["hll"],
                vec![0, 2, 3],
            ),
            (
                "two overlapping needle-blocks (starting with ') still match",
                "hello world, from europe!",
                vec!["'ll", "'lo"],
                vec![2, 3],
            ),
        ];

        for (descr, line, needles, expected) in cases {
            let mut result = fuzzy_find(line, needles);
            result.sort_unstable();
            assert_eq!(result, expected, "E: {}", descr);
        }
    }
}
