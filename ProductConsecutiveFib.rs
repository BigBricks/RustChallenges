fn product_fib(prod: u64) -> (u64, u64, bool) {
    // your code
    let mut xs: [u64; 2] = [2,3];
    while (xs[0] * xs[1]) < prod {
        let temp = xs[1] + xs[0];
        xs[0] = xs[1];
        xs[1] = temp;  
    }
    return (xs[0], xs[1], (xs[0]*xs[1]==prod))
}