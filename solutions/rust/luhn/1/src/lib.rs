pub fn is_valid(code: &str) -> bool {
    // 1. 预处理：去掉空格，并检查是否有非法字符
    // 直接在一次迭代中完成：过滤空格 + 检查数字 + 统计长度
    let mut sum = 0;
    let mut count = 0;

    let filter_space_iter = code.chars().filter(|c| !c.is_whitespace());
    for (i, c) in filter_space_iter.rev().enumerate() {
        match c.to_digit(10) {
            None => return false,
            Some(mut digit) => {
                if i % 2 == 1{
                    digit *= 2;
                    if digit > 9 {
                        digit = digit - 9;
                    }
                }
                
                sum += digit;
                count += 1;
            }
        }
    }
    count > 1 && sum % 10 == 0
}