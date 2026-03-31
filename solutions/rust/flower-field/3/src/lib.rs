pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0{
        return vec![];
    }

    garden.iter().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, c)| {
            if c == '*'{
                '*'
            }else{
                match count_mine_2(garden, x, y) {
                    0 => ' ',
                    n => (n as u8 + b'0') as char,
                }
            }
        }).collect()
    }).collect()
}

fn count_mine_2(garden: &[&str], x: usize, y: usize) -> i32{
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1{
            if dx == 0 && dy == 0{
                continue;
            }

            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            if let Some(row_str) = garden.get(ny as usize){
                // 无法直接索引 必须通过 nth() 找到第 n 个字符
                if let Some(c) = row_str.chars().nth(nx as usize){
                    if c == '*'{
                        count += 1;
                    }
                }
            }
        }
    }
    count
}