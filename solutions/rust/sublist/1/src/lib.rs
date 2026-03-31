#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match(first_list.len(), second_list.len()){
        (a, b) if a == b && first_list == second_list => Comparison::Equal,
        (a, b) if a < b && is_sublist(first_list, second_list) => Comparison::Sublist,
        (a, b) if a > b && is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist(small: &[i32], large: &[i32]) -> bool{
    if small.is_empty(){
        return true;
    }

    // windows(n) 会创建一个长度为 n 的滑动窗口迭代器
    // position 会返回第一个匹配项的索引，如果找到了说明是子列表
    large.windows(small.len()).any(|window| window == small)
}