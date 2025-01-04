use std::collections::HashSet;

pub fn unique_items<I>(iterator: I) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    let mut seen = HashSet::new();
    let mut result: Vec<String> = iterator
        .filter_map(|item| {
            let trimmed = item.trim().to_string();
            if !trimmed.is_empty() && seen.insert(trimmed.clone()) {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect();

    result.sort();
    result
}

pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}
