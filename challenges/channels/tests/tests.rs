use channels::*;

#[test]
fn test_single_producer_single_message() {
    let result = start_channel_system(1, 1);
    assert_eq!(result, vec!["Processed: Message from producer 0 - 0"]);
}

#[test]
fn test_multiple_producers() {
    let result = start_channel_system(3, 2);
    assert_eq!(result.len(), 6);
    for i in 0..3 {
        for j in 0..2 {
            assert!(result.contains(&format!("Processed: Message from producer {} - {}", i, j)));
        }
    }
}

#[test]
fn test_no_producers() {
    let result = start_channel_system(0, 5);
    assert_eq!(result, Vec::<String>::new());
}
