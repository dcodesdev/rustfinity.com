use concurrency::*;

#[test]
fn test_int() {
    let mut list = vec![1, 2, 3, 4, 5];

    let handles = concurrent_add(list.clone(), 3);

    for handle in handles {
        let result = handle.join().unwrap();
        let original = list.remove(0);

        assert_eq!(result, original + 3);
    }
}

#[test]
fn test_float() {
    let mut list = vec![10., 20., 30., 40., 50.];

    let handles = concurrent_add(list.clone(), 5.);

    for handle in handles {
        let result = handle.join().unwrap();
        let original = list.remove(0);

        assert_eq!(result, original + 5.);
    }
}

#[test]
fn test_large_numbers() {
    let mut list = vec![100000000, 200000000, 300000000, 400000000, 500000000];

    let handles = concurrent_add(list.clone(), 1000);

    for handle in handles {
        let result = handle.join().unwrap();
        let original = list.remove(0);

        assert_eq!(result, original + 1000);
    }
}
