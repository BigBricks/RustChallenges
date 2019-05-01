fn perimeter(n: u64) -> u64 {
  // your code
    let mut total = 0;
    let mut left = 1;
    let mut right = 1;
    for x in 0..(n+1) {
        let temp = left + right;
        total += left;
        left = right;
        right = temp;
    }
    total * 4
}