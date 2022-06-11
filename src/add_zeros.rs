use std::ops::Add;

pub fn add_zeros(source: &u32, count: usize) -> String {
    source
        .to_string()
        .add(String::from("0").repeat(count).as_str())
}
