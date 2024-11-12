pub fn calculate_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}