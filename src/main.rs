use std::collections::HashSet;

fn main() {
    let query = "h l f";
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
        if let Some(idx) = line.find(query) {
            indexes.insert(idx);
        } else {
            return vec![];
        }
    }
    indexes.into_iter().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

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
        ];

        for (descr, line, needles, expected) in cases {
            let mut result = fuzzy_find(line, needles);
            result.sort_unstable();
            assert_eq!(result, expected, "failed: {}", descr);
        }
    }
}
