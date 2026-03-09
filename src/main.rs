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
    let mut indexes = Vec::new();
    for query in query_parts {
        if let Some(idx) = line.find(query) {
            indexes.push(idx)
        } else {
            return vec![];
        }
    }
    indexes
}
