use std::cmp::Ordering;
fn order_weight(s: &str) -> String {
    // your code
    let split = s.split(" ");
    let mut vec = split.collect::<Vec<&str>>();
    vec.sort_by(|a, b| compare(a, b));
    vec.join(" ").to_string()
}

fn compare(x: &&str, y: &&str) -> Ordering {
    if word_num(x) == word_num(y) {
        x.cmp(y)
    }
    else {
        word_num(x).cmp(&word_num(y))
    }
}

fn word_num(s: &str) -> u32 {
    let mut sum = 0; 
    for x in s.chars() {
    sum += x.to_digit(10).unwrap();
    }