use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let mut elves: Vec<u32> = Vec::new();
    let file = File::open("elves.txt")?;
    let reader = BufReader::new(file);
    let mut current_elf: u32 = 0;
    for line in reader.lines() {
        let line2 = line?;
        if line2.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf = current_elf + line2.parse::<u32>().unwrap();
        }
    }

    elves.sort();

    let top_elves: &[u32] = elves.split_at(elves.len() - 3).1;

    Ok(println!("Contents: {:?}", top_elves.iter().sum::<u32>()))
}
