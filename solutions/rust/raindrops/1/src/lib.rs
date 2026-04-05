pub fn raindrops(n: u32) -> String {
    let mut res = String::new();

    if n % 3 == 0 {res.push_str("Pling")}
    if n % 5 == 0 {res.push_str("Plang")}
    if n % 7 == 0 {res.push_str("Plong")}

    // 2. 如果没有任何因数符合，返回数字本身的字符串形式
    if res.is_empty() {
        return n.to_string();
    }

    res
}
