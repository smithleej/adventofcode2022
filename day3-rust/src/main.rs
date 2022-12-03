use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let mut sum = 0;
    let mut batched = input.lines().chunks(3);

    for chunk in batched.into_iter() {
        let chunk_vec = chunk.collect::<Vec<_>>();
        let mut g1_chars = chunk_vec[0].chars();
        loop {
            if let Some(g1_char) = g1_chars.next() {
                if chunk_vec[1].chars().collect::<Vec<_>>().contains(&g1_char) && chunk_vec[2].chars().collect::<Vec<_>>().contains(&g1_char) {
                    sum = sum + get_position(chunk_vec[0], g1_char);
                    break;
                } else {
                    println!("None in {}, {}, {}", chunk_vec[0], chunk_vec[1], chunk_vec[2]);
                }
            }
        }
    }

    Ok(println!("{sum}"))
}

fn get_position(line: &str, left_char: char) -> u32 {
    if left_char.is_uppercase() {
        println!("Got upper {} from {}", left_char, line);
        (left_char as u32) - 38
    } else {
        println!("Got lower {} from {}", left_char, line);
        (left_char as u32) - 96
    }
}
