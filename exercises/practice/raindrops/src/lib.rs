pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
    let mut result = String::new();
    for i in &[3, 5, 7] {
        let n_mod_i = n%i;
        match n_mod_i {
            0 => {
                match i {
                    3 => result.push_str("Pling"),
                    5 => result.push_str("Plang"),
                    7 => result.push_str("Plong"),
                    _ => {}
                }
            }
            _ => {}
        };
    }

    if result.is_empty() {
        result = n.to_string();
    }
    return result;
}
