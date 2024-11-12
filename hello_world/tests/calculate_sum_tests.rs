use hello_world::calculate_sum;

#[test]
fn test_calculate_sum_empty() {
        let numbers = vec![];
        assert_eq!(calculate_sum(&numbers), 0);
}

#[test]
fn test_calculate_sum_single() {
    let numbers = vec![5];
    assert_eq!(calculate_sum(&numbers), 5);
}

#[test]
fn test_calculate_sum_multiple() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(calculate_sum(&numbers), 15);
}