fn order(s: &str) -> String {
    // code here
    let split = s.split(" ");
    let mut vec = split.collect::<Vec<&str>>();
    vec.sort_by(|a, b| find_num(a).cmp(&find_num(b)));
    vec.join(" ").to_string()
}

fn find_num(s: &str) -> usize {
   let j = s.chars().filter(|x| !x.is_alphabetic()).collect::<Vec<_>>();
   println!("findnum j {}", j[0]);
   let x = j[0] as usize;
   println!("x: {}", x);
   x - 48
}