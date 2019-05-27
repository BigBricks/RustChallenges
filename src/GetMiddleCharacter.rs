fn get_middle(s:&str) -> &str {
    let mut start = s.len()/2;
    let mut length = 1;
    if s.len() % 2 == 0 {
        start = s.len()/2 - 1;
        length = 2;
        &s[start..start+length]
    } else {
        &s[start..start+length]
    }
}