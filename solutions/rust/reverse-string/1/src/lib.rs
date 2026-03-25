pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    // 1. 先获取字符迭代器
    // 2. 再反转迭代器
    // 3. 再收集
    input.chars().rev().collect()
}
