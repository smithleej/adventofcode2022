fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let mut left_chars = left.chars().into_iter();
        loop {
            if let Some(left_char) = left_chars.next() {
                if right.chars().collect::<Vec<_>>().contains(&left_char) {
                    if left_char.is_uppercase() {
                        println!("Got upper {} from {}", left_char, line);
                        sum = sum + (left_char as u32) - 38;
                    } else {
                        println!("Got lower {} from {}", left_char, line);
                        sum = sum + (left_char as u32) - 96;
                    }
                    break;
                }
            }
        }
    }

    Ok(println!("{sum}"))
}
