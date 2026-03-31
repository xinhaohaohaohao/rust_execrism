pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10{
        return true;
    }

    // 确定位数
    let num_len = num.to_string().len() as u32;
    
    let mut temp = num;
    let mut sum:u64 = 0; // 使用 u64 防止求幂过程溢出

    while temp > 0 {
        let digit = temp % 10;
        sum += (digit as u64).pow(num_len);
        temp /= 10;
    }

    num as u64 == sum 
}