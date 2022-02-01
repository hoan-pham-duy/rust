pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    // return String::from("Failed");
    let result = input.chars().rev().collect();
    return result;
}