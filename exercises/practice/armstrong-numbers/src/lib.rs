pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num);
    let num_digit_count = num.to_string().len();
    let mut sum = 0;
    for digit in num.to_string().chars() {
        sum += digit.to_digit(10).unwrap().pow(num_digit_count as u32);
    }
    sum == num
}
