// use color_eyre::eyre::eyre;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let split_lines = input.lines().map(|line| line.split(",").collect::<Vec<_>>());
    let answer: u32 = split_lines.map (|line| {
        let (first_elf, second_elf) = (line[0], line[1]);
        let first_elf_sections = first_elf.split("-").map(|string|string.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let second_elf_sections = second_elf.split("-").map(|string|string.parse::<u32>().unwrap()).collect::<Vec<_>>();
        if first_elf_sections[0] <= second_elf_sections[0] && first_elf_sections[1] >= second_elf_sections[1] {
            println!("Second {:?} contained in {:?}", second_elf_sections, first_elf_sections);
            1
        } else if second_elf_sections[0] <= first_elf_sections[0] && second_elf_sections[1] >= first_elf_sections[1] {
            println!("First {:?} contained in {:?}", first_elf_sections, second_elf_sections);
            1
        } else { 0 }
    }).sum();
    Ok(println!("Such pairs {}", answer))
}
