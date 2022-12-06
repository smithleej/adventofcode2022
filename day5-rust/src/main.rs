use regex::{Captures, Regex};
use lazy_static::lazy_static;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    /*
    [N]         [C]     [Z]
    [Q] [G]     [V]     [S]         [V]
    [L] [C]     [M]     [T]     [W] [L]
    [S] [H]     [L]     [C] [D] [H] [S]
    [C] [V] [F] [D]     [D] [B] [Q] [F]
    [Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
    [P] [P] [C] [W] [W] [F] [W] [J] [C]
    [T] [L] [D] [G] [P] [P] [V] [N] [R]
     1   2   3   4   5   6   7   8   9
    */

    let stack1 = vec!['T', 'P', 'Z', 'C', 'S', 'L', 'Q', 'N'];
    let stack2 = vec!['L', 'P', 'T', 'V', 'H', 'C', 'G'];
    let stack3 = vec!['D', 'C', 'Z', 'F'];
    let stack4 = vec!['G', 'W', 'T', 'D', 'L', 'M', 'V', 'C'];
    let stack5 = vec!['P', 'W', 'C'];
    let stack6 = vec!['P', 'F', 'J', 'D', 'C', 'T', 'S', 'Z'];
    let stack7 = vec!['V', 'W', 'G', 'B', 'D'];
    let stack8 = vec!['N', 'J', 'S', 'Q', 'H', 'W'];
    let stack9 = vec!['R', 'C', 'Q', 'F', 'S', 'L', 'V'];

    let mut stacks = vec![stack1, stack2, stack3, stack4, stack5, stack6, stack7, stack8, stack9];

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();
    }

    let x = include_str!("input.txt");
    for line in x.lines() {
        for instruction in RE.captures_iter(line).map(|cap| Instruction::try_from(cap)) {
            let instruction_thing = dbg!(instruction?);

            let from_stack = &mut stacks[instruction_thing.from-1];
            
            let taken_items: Vec<char> = from_stack.split_off(from_stack.len()-instruction_thing.count);

            let to_stack = &mut stacks[instruction_thing.to-1];
            
            to_stack.append(&mut dbg!(taken_items));
            
            println!("{:#?}", stacks);
        }
    }
            
    println!("{:#?}", stacks);

    print!("{:?}", 
        stacks.iter().map(|stack| {
            stack.last().unwrap()
        }).collect::<String>()
    );

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize
}

impl TryFrom<Captures<'_>> for Instruction {
    type Error = color_eyre::Report;

    fn try_from(cap: Captures) -> Result<Self, Self::Error> {
        let groups = (cap.get(1), cap.get(2), cap.get(3));
        match groups {
            (Some(count), Some(from), Some(to)) =>
                Ok(Instruction { count: count.as_str().parse::<usize>()?, from: from.as_str().parse::<usize>()?, to: to.as_str().parse::<usize>()? }),
            _ => Err(color_eyre::eyre::eyre!("expected format: move 'count' from 'stack' to 'stack', got {cap:?}"))
        }
    }
}
