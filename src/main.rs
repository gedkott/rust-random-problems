use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn sparse_arrays(strings: &[&str], queries: &[&str]) -> Vec<usize> {
    queries.iter().map(|q| {
        let mut number_of_occurrences: usize = 0;
        for s in strings {
            if q == s {
                number_of_occurrences += 1
            }
        }
        number_of_occurrences
    }).collect()
}

fn sparse_arrays_with_hash_map<'a>(strings: &[&'a str], queries: &[&str]) -> Vec<usize> {
    let hash_map_counter = 
        |mut acc: HashMap<&'a str, usize>, s: &&'a str| {
            acc.entry(*s)
                .and_modify(|n| *n += 1)
                .or_insert(1);
            acc
        };

    let strings_map: HashMap<&str, usize> = strings
        .iter()
        .fold(HashMap::new(), hash_map_counter);

    queries.iter().map(|q| {
        *strings_map.get(*q).unwrap_or(&0)
    }).collect()
}


#[test]
fn test_sparse_arrays() {
    let strings = ["aba", "baba", "aba", "xzxb"];
    let queries = ["aba", "xzxb", "ab"];
    let result = sparse_arrays(&strings, &queries);
    assert_eq!(result[0], 2);
    assert_eq!(result[1], 1);
    assert_eq!(result[2], 0);
}

#[test]
fn test_sparse_arrays_with_hash_map() {
    let strings = ["aba", "baba", "aba", "xzxb"];
    let queries = ["aba", "xzxb", "ab"];
    let result = sparse_arrays_with_hash_map(&strings, &queries);
    assert_eq!(result[0], 2);
    assert_eq!(result[1], 1);
    assert_eq!(result[2], 0);
}

#[test]
fn test_sparse_arrays_with_hash_map_with_strings() {
    let ts = String::from("aba");
    let strings = [ts.as_ref(), "baba", ts.as_ref(), "xzxb"];
    let queries = [ts.as_ref(), "xzxb", "ab"];
    let result = sparse_arrays_with_hash_map(&strings, &queries);
    assert_eq!(result[0], 2);
    assert_eq!(result[1], 1);
    assert_eq!(result[2], 0);
}