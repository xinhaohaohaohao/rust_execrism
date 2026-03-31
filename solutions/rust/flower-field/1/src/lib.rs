pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len() as i32;
    if height == 0{
        return vec![];
    }

    let width = garden[0].len() as i32;

    (0..height).map(|y| {
        (0..width).map(|x| {
            if garden[y as usize].as_bytes()[x as usize] == b'*'{
                '*' 
            }else{
                match count_mine(garden, x, y){
                    0 => ' ',
                    n => (n as u8 + b'0') as char, 
                }
            }
        }).collect()
    }).collect()

    
}

fn count_mine(garden: &[&str], x: i32, y: i32) -> i32{
    let mut count = 0;

    for dy in -1..=1{
        for dx in -1..=1{
            if dx == 0 && dy == 0{
                continue;
            }

            let ny = y + dy;
            let nx = x + dx;

            if ny >= 0 && ny < garden.len() as i32{
                let row = garden[ny as usize].as_bytes();
                if nx >= 0 && nx < row.len() as i32{
                    if row[nx as usize] == b'*'{
                        count += 1;
                    }
                }
            }
        }
    }

    count
}