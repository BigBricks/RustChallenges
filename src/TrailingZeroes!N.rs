fn zeros(n: u64) -> u64 {
    let mut count: u64 = 0;
    let mut x = n;
    if n <= 0 {
        return count;
    }

    while x > 0 {
        x/=5;
        count += x;
    }
    count
}