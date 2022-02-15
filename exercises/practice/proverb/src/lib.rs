pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut result = String::new();
    for i in 0..list.len() {
        if i == list.len() - 1 {
            result.push_str(&format!("And all for the want of a {}.", list[0]));
        } else {
            result.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[i],
                list[i + 1]
            ));
        }
    }
    result
}
