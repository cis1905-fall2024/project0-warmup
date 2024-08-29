use rusty::rainfall;

#[test]
fn test_sqrt() {
    for i in 0..30 {
        assert_eq!(rusty::sqrt(i), (i as f64).sqrt() as u32);
    }
}

#[test]
fn test_binary_search_empty() {
    let data = vec![];
    assert_eq!(rusty::binary_search(&data, 0), None);
    assert_eq!(rusty::binary_search(&data, -10), None);
    assert_eq!(rusty::binary_search(&data, 78), None);
}
#[test]
fn test_binary_search_small() {
    let data = vec![1];
    assert_eq!(rusty::binary_search(&data, 0), None);
    assert_eq!(rusty::binary_search(&data, 1), Some(0));
    assert_eq!(rusty::binary_search(&data, 2), None);

    let data = vec![-2, -1];
    assert_eq!(rusty::binary_search(&data, -3), None);
    assert_eq!(rusty::binary_search(&data, -2), Some(0));
    assert_eq!(rusty::binary_search(&data, -1), Some(1));
    assert_eq!(rusty::binary_search(&data, 0), None);

    let data = vec![10, 20, 30];
    assert_eq!(rusty::binary_search(&data, 5), None);
    assert_eq!(rusty::binary_search(&data, 10), Some(0));
    assert_eq!(rusty::binary_search(&data, 15), None);
    assert_eq!(rusty::binary_search(&data, 20), Some(1));
    assert_eq!(rusty::binary_search(&data, 25), None);
    assert_eq!(rusty::binary_search(&data, 30), Some(2));
    assert_eq!(rusty::binary_search(&data, 35), None);
}
#[test]
fn test_binary_search() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in 0..10 {
        assert_eq!(rusty::binary_search(&data, data[i]), Some(i as u32));
    }
    assert_eq!(rusty::binary_search(&data, 11), None);
}
#[test]
fn test_binary_search_pbt() {
    let data = vec![-17, -3, -1, 0, 5, 8, 9, 20, 100, 9283];
    for i in 0..(data.len()) {
        assert_eq!(rusty::binary_search(&data, data[i]), Some(i as u32));
    }
}

#[test]
fn test_rainfall_empty() {
    let data = vec![];
    assert_eq!(rainfall(&data), None);
}
#[test]
fn test_rainfall_no_nonnegative_values() {
    let data = vec![-1, -3, -100, -998, -1000];
    assert_eq!(rainfall(&data), None);
}
#[test]
fn test_rainfall_no_nonnegative_values_with_sentinel() {
    let data = vec![-1, -3, -100, -998, -999, -1000];
    assert_eq!(rainfall(&data), None);
}
#[test]
fn test_rainfall_no_sentinel() {
    let data = vec![2, 3, 1, 3, 6];
    assert_eq!(rainfall(&data), Some(3.0));
}
#[test]
fn test_rainfall_smaller_than_sentinel() {
    let data = vec![2, 3, 1, 3, 6, -1000];
    assert_eq!(rainfall(&data), Some(3.0));
}
#[test]
fn test_rainfall_with_zeros() {
    let data = vec![2, 0, 0, 3, -1, 5, -999, 10];
    assert_eq!(rainfall(&data), Some(2.0));
}
#[test]
fn test_rainfall_with_sentinel() {
    let data = vec![6, 8, 1, 1, -999, 4, 5, 6];
    assert_eq!(rainfall(&data), Some(4.0));
}
#[test]
fn test_rainfall_with_negative_values() {
    let data = vec![2, -1, 7, -5, -2, 9];
    assert_eq!(rainfall(&data), Some(6.0));
}
#[test]
fn test_rainfall_with_negative_values_and_sentinel() {
    let data = vec![1, -1, 7, -5, -2, 10, 999, 3, 17, 6, -999, 4, 999, 6, -1, -1, -1, -999, 1, 2, 3];
    assert_eq!(rainfall(&data), Some(149.0));
}
