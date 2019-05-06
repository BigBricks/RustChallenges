fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
  let mut v = arr.to_vec();
  v.sort_by(|a,b| a.cmp(b));
  v  
}