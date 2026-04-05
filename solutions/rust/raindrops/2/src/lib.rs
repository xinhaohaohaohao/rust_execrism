pub fn raindrops(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let res: String = sounds.iter()
        .filter(|&(divisor, _)| n % divisor == 0)
        .map(|&(_, sound)| sound)
        .collect();

    if res.is_empty() {n.to_string()}else{res}
}
