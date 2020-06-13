fn is_number(s: String) -> bool {
    let j = s.trim();
    if (j.parse::<i64>().is_ok()) {
        return true;
    } else if (j.parse::<f64>().is_ok()) {
        return true;
    } else {
        return false;
    }
}